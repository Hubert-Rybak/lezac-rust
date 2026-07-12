//! Touch / pointer input so the game is playable on phones, which have no
//! keyboard. The original game (and this port) is entirely keyboard driven, so
//! on an iPhone you cannot even get past the "PRESS ANY KEY" title screen.
//!
//! This module maps touches into the 320x200 virtual render space, exposes
//! tap/menu helpers for the menu screens, an on-screen virtual gamepad for
//! gameplay, and draws that gamepad. It stays inert on desktop until a real
//! touch happens, so keyboard play is unchanged.

use crate::assets::SpriteSheet;
use crate::input::PlayerInput;
use crate::renderer::{draw_text_small, SCREEN_H, SCREEN_W};
use macroquad::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

static TOUCH_ACTIVE: AtomicBool = AtomicBool::new(false);

/// True once the player has used touch at least once. Gates the on-screen
/// controls so desktop / keyboard play is left untouched.
pub fn is_active() -> bool {
    TOUCH_ACTIVE.load(Ordering::Relaxed)
}

/// Call once per rendered frame. Latches "touch mode" the first time a real
/// touch is seen so the on-screen controls appear on phones but stay hidden on
/// desktop.
pub fn update() {
    if !touches().is_empty() {
        TOUCH_ACTIVE.store(true, Ordering::Relaxed);
    }
}

/// On-screen gamepad button rectangles, in 320x200 virtual space. The action
/// cluster sits bottom-right, movement bottom-left, above the 40px HUD strip.
pub struct Buttons {
    pub left: Rect,
    pub right: Rect,
    pub down: Rect,
    pub jump: Rect,
    pub fire: Rect,
    pub weapon: Rect,
}

pub const BUTTONS: Buttons = Buttons {
    left: Rect { x: 4.0, y: 120.0, w: 32.0, h: 34.0 },
    right: Rect { x: 40.0, y: 120.0, w: 32.0, h: 34.0 },
    down: Rect { x: 4.0, y: 84.0, w: 68.0, h: 30.0 },
    jump: Rect { x: 248.0, y: 120.0, w: 32.0, h: 34.0 },
    fire: Rect { x: 284.0, y: 120.0, w: 32.0, h: 34.0 },
    weapon: Rect { x: 248.0, y: 84.0, w: 68.0, h: 30.0 },
};

/// Which menu entry a tap landed on. Matches `renderer::draw_menu` layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuTap {
    OnePlayer,
    TwoPlayer,
    Info,
    Instructions,
    Records,
    Language,
}

/// Map the window-space point into the 320x200 render space, inverting the
/// letterboxed scale/offset applied in `Renderer::end`.
fn screen_to_virtual(p: Vec2) -> Vec2 {
    let sw = screen_width();
    let sh = screen_height();
    let scale = (sw / SCREEN_W).min(sh / SCREEN_H).max(f32::MIN_POSITIVE);
    let ox = (sw - SCREEN_W * scale) / 2.0;
    let oy = (sh - SCREEN_H * scale) / 2.0;
    vec2((p.x - ox) / scale, (p.y - oy) / scale)
}

/// Points currently held down (touch or, once active, the mouse), in virtual
/// space.
fn held_points() -> Vec<Vec2> {
    let mut v: Vec<Vec2> = touches()
        .into_iter()
        .filter(|t| !matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled))
        .map(|t| screen_to_virtual(t.position))
        .collect();
    if is_active() && is_mouse_button_down(MouseButton::Left) {
        let (mx, my) = mouse_position();
        v.push(screen_to_virtual(vec2(mx, my)));
    }
    v
}

/// Points that began this frame (touch just started, or mouse just pressed).
fn started_points() -> Vec<Vec2> {
    let mut v: Vec<Vec2> = touches()
        .into_iter()
        .filter(|t| matches!(t.phase, TouchPhase::Started))
        .map(|t| screen_to_virtual(t.position))
        .collect();
    if is_active() && is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        v.push(screen_to_virtual(vec2(mx, my)));
    }
    v
}

fn any_in(points: &[Vec2], r: Rect) -> bool {
    points.iter().any(|p| r.contains(*p))
}

/// True if a tap began anywhere this frame. Used to advance "press any key"
/// screens.
pub fn tapped() -> bool {
    !started_points().is_empty()
}

/// Player-1 input from the on-screen gamepad, ORed into the keyboard input by
/// the caller. Empty until touch mode is active.
pub fn player1_input() -> PlayerInput {
    if !is_active() {
        return PlayerInput::default();
    }
    let held = held_points();
    let started = started_points();
    PlayerInput {
        left: any_in(&held, BUTTONS.left),
        right: any_in(&held, BUTTONS.right),
        down: any_in(&held, BUTTONS.down),
        jump: any_in(&held, BUTTONS.jump),
        fire: any_in(&started, BUTTONS.fire),
        weapon_change: any_in(&started, BUTTONS.weapon),
    }
}

/// If a tap began on a menu entry this frame, return which one.
pub fn menu_tap() -> Option<MenuTap> {
    let started = started_points();
    menu_choice_at(*started.first()?)
}

fn menu_choice_at(p: Vec2) -> Option<MenuTap> {
    // Mirrors `renderer::draw_menu`: entries drawn at y = 80,92,104,116,128,140
    // inside the 20..300 box. Each row claims a ~12px band.
    if p.x < 20.0 || p.x > 300.0 {
        return None;
    }
    match p.y {
        y if (76.0..88.0).contains(&y) => Some(MenuTap::OnePlayer),
        y if (88.0..100.0).contains(&y) => Some(MenuTap::TwoPlayer),
        y if (100.0..112.0).contains(&y) => Some(MenuTap::Info),
        y if (112.0..124.0).contains(&y) => Some(MenuTap::Instructions),
        y if (124.0..136.0).contains(&y) => Some(MenuTap::Records),
        y if (136.0..150.0).contains(&y) => Some(MenuTap::Language),
        _ => None,
    }
}

const BTN_BG: Color = Color::new(1.0, 1.0, 1.0, 0.12);
const BTN_BG_ON: Color = Color::new(1.0, 1.0, 1.0, 0.32);
const BTN_LINE: Color = Color::new(1.0, 1.0, 1.0, 0.35);
const GLYPH: Color = Color::new(1.0, 1.0, 1.0, 0.75);

/// Draw the virtual gamepad (only once touch mode is active). Call inside the
/// virtual-space camera pass, after the HUD.
pub fn draw_controls(fonts: &SpriteSheet) {
    if !is_active() {
        return;
    }
    let held = held_points();
    draw_button(BUTTONS.left, any_in(&held, BUTTONS.left));
    draw_tri(BUTTONS.left, Dir::Left);
    draw_button(BUTTONS.right, any_in(&held, BUTTONS.right));
    draw_tri(BUTTONS.right, Dir::Right);
    draw_button(BUTTONS.down, any_in(&held, BUTTONS.down));
    draw_tri(BUTTONS.down, Dir::Down);
    draw_button(BUTTONS.jump, any_in(&held, BUTTONS.jump));
    draw_tri(BUTTONS.jump, Dir::Up);
    draw_button(BUTTONS.fire, any_in(&held, BUTTONS.fire));
    let fc = center(BUTTONS.fire);
    draw_circle(fc.x, fc.y, 6.0, GLYPH);
    draw_button(BUTTONS.weapon, any_in(&held, BUTTONS.weapon));
    let wc = center(BUTTONS.weapon);
    draw_text_small(fonts, "wpn", wc.x - 12.0, wc.y - 4.0, GLYPH);
}

fn draw_button(r: Rect, on: bool) {
    draw_rectangle(r.x, r.y, r.w, r.h, if on { BTN_BG_ON } else { BTN_BG });
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 1.0, BTN_LINE);
}

fn center(r: Rect) -> Vec2 {
    vec2(r.x + r.w / 2.0, r.y + r.h / 2.0)
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn draw_tri(r: Rect, dir: Dir) {
    let c = center(r);
    let s = 8.0;
    let (a, b, d) = match dir {
        Dir::Left => (
            vec2(c.x - s, c.y),
            vec2(c.x + s, c.y - s),
            vec2(c.x + s, c.y + s),
        ),
        Dir::Right => (
            vec2(c.x + s, c.y),
            vec2(c.x - s, c.y - s),
            vec2(c.x - s, c.y + s),
        ),
        Dir::Up => (
            vec2(c.x, c.y - s),
            vec2(c.x - s, c.y + s),
            vec2(c.x + s, c.y + s),
        ),
        Dir::Down => (
            vec2(c.x, c.y + s),
            vec2(c.x - s, c.y - s),
            vec2(c.x + s, c.y - s),
        ),
    };
    draw_triangle(a, b, d, GLYPH);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_bands_match_drawn_menu_rows() {
        assert_eq!(menu_choice_at(vec2(160.0, 80.0)), Some(MenuTap::OnePlayer));
        assert_eq!(menu_choice_at(vec2(160.0, 92.0)), Some(MenuTap::TwoPlayer));
        assert_eq!(menu_choice_at(vec2(160.0, 104.0)), Some(MenuTap::Info));
        assert_eq!(
            menu_choice_at(vec2(160.0, 116.0)),
            Some(MenuTap::Instructions)
        );
        assert_eq!(menu_choice_at(vec2(160.0, 128.0)), Some(MenuTap::Records));
        assert_eq!(menu_choice_at(vec2(160.0, 140.0)), Some(MenuTap::Language));
    }

    #[test]
    fn menu_taps_outside_the_box_are_ignored() {
        assert_eq!(menu_choice_at(vec2(5.0, 80.0)), None);
        assert_eq!(menu_choice_at(vec2(160.0, 40.0)), None);
        assert_eq!(menu_choice_at(vec2(160.0, 180.0)), None);
    }

    #[test]
    fn gamepad_buttons_do_not_overlap_the_hud_strip() {
        // HUD occupies y = 160..200; controls must stay above it.
        for r in [
            BUTTONS.left,
            BUTTONS.right,
            BUTTONS.down,
            BUTTONS.jump,
            BUTTONS.fire,
            BUTTONS.weapon,
        ] {
            assert!(r.y + r.h <= 160.0, "button {r:?} overlaps HUD");
            assert!(r.x >= 0.0 && r.x + r.w <= SCREEN_W, "button {r:?} off-screen");
        }
    }

    #[test]
    fn held_point_hit_tests_action_buttons() {
        let pts = [center(BUTTONS.jump)];
        assert!(any_in(&pts, BUTTONS.jump));
        assert!(!any_in(&pts, BUTTONS.fire));
    }
}
