//! Rendering: tiles, sprites, HUD, backgrounds

use macroquad::prelude::*;
use crate::assets::{Palette, SpriteSheet, Background};
use crate::level::Level;
use crate::player::{Player, Bomb, BombType, Debris};
use crate::monsters::{Monster, MonsterType, Powerup, PowerupType};

pub const SCREEN_W: f32 = 320.0;
pub const SCREEN_H: f32 = 200.0;
pub const TILE_SIZE: f32 = 8.0;
pub const HUD_HEIGHT: f32 = 40.0;
pub const PLAY_HEIGHT: f32 = SCREEN_H - HUD_HEIGHT;

pub struct Renderer {
    pub render_target: RenderTarget,
    pub camera: Camera2D,
}

impl Renderer {
    pub fn new() -> Self {
        let rt = render_target(SCREEN_W as u32, SCREEN_H as u32);
        rt.texture.set_filter(FilterMode::Nearest);
        let cam = Camera2D {
            render_target: Some(rt.clone()),
            zoom: vec2(2.0 / SCREEN_W, 2.0 / SCREEN_H),
            target: vec2(SCREEN_W / 2.0, SCREEN_H / 2.0),
            ..Default::default()
        };
        Renderer { render_target: rt, camera: cam }
    }

    pub fn begin(&self) {
        set_camera(&self.camera);
        clear_background(BLACK);
    }

    pub fn end(&self) {
        set_default_camera();
        let sw = screen_width();
        let sh = screen_height();
        let scale = (sw / SCREEN_W).min(sh / SCREEN_H);
        let dw = SCREEN_W * scale;
        let dh = SCREEN_H * scale;
        let ox = (sw - dw) / 2.0;
        let oy = (sh - dh) / 2.0;
        clear_background(BLACK);
        draw_texture_ex(&self.render_target.texture, ox, oy, WHITE, DrawTextureParams {
            dest_size: Some(vec2(dw, dh)),
            flip_y: true,
            ..Default::default()
        });
    }
}

pub fn tile_color(tile: u8, palette: &Palette) -> Color {
    if tile == 0 { return Color::new(0.0, 0.0, 0.0, 0.0); }
    let pi = match tile {
        1 => 0x33, 2 => 0x31, 3 => 0x30, 4 => 0x04, 5 => 0x38,
        6 => 0x06, 7 => 0x07, 8 => 0x08, 9 => 0x35, 0x0a => 0x32,
        0x0b => 0x34, 0x0c => 0x0C, 0x0d => 0x36, 0x0e => 0x0E, 0x0f => 0x0F,
        _ => tile,
    };
    palette.to_color(pi)
}

pub fn draw_background(bg: &Background, scroll_x: f32, _palette: &Palette) {
    let off = (scroll_x * 0.5) % bg.width as f32;
    let src_h = (PLAY_HEIGHT / SCREEN_H * bg.height as f32).min(bg.height as f32);
    draw_texture_ex(&bg.texture, -off, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(bg.width as f32, PLAY_HEIGHT)),
        source: Some(Rect::new(0.0, 0.0, bg.width as f32, src_h)),
        ..Default::default()
    });
    if off > 0.0 {
        draw_texture_ex(&bg.texture, bg.width as f32 - off, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(bg.width as f32, PLAY_HEIGHT)),
            source: Some(Rect::new(0.0, 0.0, bg.width as f32, src_h)),
            ..Default::default()
        });
    }
}

pub fn draw_tiles(level: &Level, sx: f32, sy: f32, palette: &Palette, show_bg: bool) {
    let stx = (sx / TILE_SIZE) as i32;
    let sty = (sy / TILE_SIZE) as i32;
    let etx = stx + (SCREEN_W / TILE_SIZE) as i32 + 2;
    let ety = sty + (PLAY_HEIGHT / TILE_SIZE) as i32 + 2;

    if show_bg {
        for ty in sty..ety {
            for tx in stx..etx {
                if tx >= 0 && ty >= 0 {
                    let t = level.bg_tile_at(tx as usize, ty as usize);
                    if t != 0 {
                        let c = tile_color(t, palette);
                        let bc = Color::new(c.r * 0.6, c.g * 0.6, c.b * 0.6, c.a);
                        draw_rectangle(tx as f32 * TILE_SIZE - sx, ty as f32 * TILE_SIZE - sy, TILE_SIZE, TILE_SIZE, bc);
                    }
                }
            }
        }
    }

    for ty in sty..ety {
        for tx in stx..etx {
            if tx >= 0 && ty >= 0 {
                let t = level.tile_at(tx as usize, ty as usize);
                if t != 0 {
                    let c = tile_color(t, palette);
                    let px = tx as f32 * TILE_SIZE - sx;
                    let py = ty as f32 * TILE_SIZE - sy;
                    draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, c);
                    draw_rectangle_lines(px, py, TILE_SIZE, TILE_SIZE, 1.0,
                        Color::new(c.r * 0.8, c.g * 0.8, c.b * 0.8, 0.3));
                }
            }
        }
    }
}

pub fn draw_player(player: &Player, sprites: &SpriteSheet, sx: f32, sy: f32) {
    if !player.alive { return; }
    if player.invincible_timer > 0.0 && (player.invincible_timer * 10.0) as i32 % 2 == 0 { return; }
    let px = player.x - sx;
    let py = player.y - sy;
    let si = player.sprite_index();
    if si < sprites.num_sprites() {
        if player.facing_right { sprites.draw(si, px, py); }
        else { sprites.draw_flipped(si, px + sprites.sprite_width(si) as f32, py); }
    } else {
        let c = if player.player_idx == 0 { Color::new(1.0, 0.2, 0.2, 1.0) } else { Color::new(0.2, 0.2, 1.0, 1.0) };
        draw_rectangle(px, py, 12.0, 16.0, c);
    }
}

pub fn draw_bombs(bombs: &[Bomb], sprites: &SpriteSheet, sx: f32, sy: f32, time: f32) {
    for b in bombs {
        let px = b.x - sx;
        let py = b.y - sy;
        if b.exploding {
            let r = b.bomb_type.radius() * (1.0 - b.explosion_timer / 0.5);
            let a = b.explosion_timer / 0.5;
            draw_circle(px + 8.0, py + 8.0, r, Color::new(1.0, 0.5 * a, 0.0, a));
            draw_circle(px + 8.0, py + 8.0, r * 0.6, Color::new(1.0, 1.0, 0.0, a));
        } else {
            let flash = b.timer < 1.0 && (time * 8.0).sin() > 0.0;
            let si = b.bomb_type.sprite_index();
            if si < sprites.num_sprites() {
                if flash { sprites.draw_colored(si, px, py, YELLOW); }
                else { sprites.draw(si, px, py); }
            } else {
                let c = if flash { YELLOW } else { match b.bomb_type {
                    BombType::Small => Color::new(0.4, 0.4, 0.4, 1.0),
                    BombType::Medium => Color::new(0.6, 0.4, 0.2, 1.0),
                    BombType::Large => Color::new(0.8, 0.2, 0.2, 1.0),
                    BombType::Super => Color::new(0.2, 0.8, 0.2, 1.0),
                }};
                draw_circle(px + 8.0, py + 8.0, 6.0, c);
                draw_line(px + 8.0, py + 2.0, px + 12.0, py - 2.0, 1.0, ORANGE);
            }
        }
    }
}

pub fn draw_debris(debris: &[Debris], palette: &Palette, sx: f32, sy: f32) {
    for d in debris {
        let a = (d.life / 2.0).min(1.0);
        let c = palette.to_color_alpha(d.color, a);
        draw_rectangle(d.x - sx, d.y - sy, 2.0, 2.0, c);
    }
}

pub fn draw_monsters(monsters: &[Monster], sprites: &SpriteSheet, sx: f32, sy: f32) {
    for m in monsters {
        if !m.alive { continue; }
        let px = m.x - sx;
        let py = m.y - sy;
        let si = m.sprite_index();
        if si < sprites.num_sprites() {
            if m.facing_right { sprites.draw(si, px, py); }
            else { sprites.draw_flipped(si, px + sprites.sprite_width(si) as f32, py); }
        } else {
            let c = match m.monster_type {
                MonsterType::Walker => Color::new(0.8, 0.0, 0.8, 1.0),
                MonsterType::Chaser => Color::new(0.8, 0.0, 0.0, 1.0),
                MonsterType::Floater => Color::new(0.0, 0.8, 0.8, 1.0),
                MonsterType::Jumper => Color::new(0.8, 0.8, 0.0, 1.0),
            };
            draw_rectangle(px, py, 14.0, 14.0, c);
            draw_rectangle(px + 3.0, py + 3.0, 3.0, 3.0, WHITE);
            draw_rectangle(px + 9.0, py + 3.0, 3.0, 3.0, WHITE);
        }
    }
}

pub fn draw_powerups(powerups: &[Powerup], sprites: &SpriteSheet, sx: f32, sy: f32, time: f32) {
    for p in powerups {
        if !p.alive { continue; }
        let px = p.x - sx;
        let py = p.y - sy + (time * 3.0).sin() * 2.0;
        let si = p.powerup_type.sprite_index();
        if si < sprites.num_sprites() {
            sprites.draw(si, px, py);
        } else {
            let c = match p.powerup_type {
                PowerupType::Present => GOLD,
                PowerupType::HotDog => ORANGE,
                PowerupType::FirstAid => RED,
                PowerupType::YellowBombBox => YELLOW,
                PowerupType::GreenBombBox => GREEN,
                PowerupType::JollyCloud => SKYBLUE,
                PowerupType::BigDiamond => Color::new(0.0, 1.0, 1.0, 1.0),
            };
            draw_triangle(vec2(px + 8.0, py), vec2(px, py + 8.0), vec2(px + 16.0, py + 8.0), c);
            draw_triangle(vec2(px + 8.0, py + 16.0), vec2(px, py + 8.0), vec2(px + 16.0, py + 8.0), c);
        }
    }
}

pub fn draw_hud(players: &[Player], level: &Level, destr_pct: f32, fonts: &SpriteSheet, _palette: &Palette, _two_player: bool) {
    let hy = PLAY_HEIGHT;
    draw_rectangle(0.0, hy, SCREEN_W, HUD_HEIGHT, Color::new(0.0, 0.0, 0.3, 1.0));
    draw_line(0.0, hy, SCREEN_W, hy, 1.0, Color::new(0.3, 0.3, 0.8, 1.0));

    if let Some(p) = players.first() {
        // Lives indicators
        for i in 0..p.lives.max(0) as usize {
            draw_text_small(fonts, "a", 4.0 + i as f32 * 10.0, hy + 28.0, GREEN);
        }
        // Energy bar
        draw_rectangle_lines(4.0, hy + 4.0, 100.0, 10.0, 1.0, SKYBLUE);
        let fw = (p.energy / p.max_energy) * 98.0;
        draw_rectangle(5.0, hy + 5.0, fw, 8.0, YELLOW);
        // Score
        draw_text_small(fonts, &format!("{}", p.score), 112.0, hy + 12.0, WHITE);

        // Current bomb indicator
        let bc = match p.current_bomb {
            BombType::Small => Color::new(0.4, 0.4, 0.4, 1.0),
            BombType::Medium => Color::new(0.6, 0.4, 0.2, 1.0),
            BombType::Large => Color::new(0.8, 0.2, 0.2, 1.0),
            BombType::Super => Color::new(0.2, 0.8, 0.2, 1.0),
        };
        let cx = SCREEN_W / 2.0 - 30.0;
        draw_rectangle(cx, hy + 4.0, 12.0, 12.0, bc);
        draw_text_small(fonts, &format!("{:2}", p.bombs[p.current_bomb as usize]), cx + 14.0, hy + 12.0, WHITE);
    }

    // Bonus/destruction targets
    let bx = SCREEN_W / 2.0 + 10.0;
    draw_circle(bx + 4.0, hy + 8.0, 4.0, GOLD);
    draw_text_small(fonts, &format!("{:02}", level.bonus_target), bx + 12.0, hy + 10.0, WHITE);
    draw_text_small(fonts, "*", bx, hy + 24.0, ORANGE);
    draw_text_small(fonts, &format!("{:02}", level.destruction_pct), bx + 12.0, hy + 24.0, WHITE);
    draw_text_small(fonts, &format!("{}%", destr_pct as u32), bx + 34.0, hy + 24.0, YELLOW);
}

/// Draw text using the font sprite sheet.
/// The font file contains:
///   Sprites 0-25:  10x10 large/shadow font (a-z)
///   Sprites 26-51: 8x8 small font (a-z)
///   Sprites 52-61: 8x8 small font (0-9)
///   Sprites 62-67: 8x8 small font (punctuation: . , - = ! ')
/// Text input uses lowercase internally. The game's original strings use
/// custom encoding: ':' = '.', ';' = ',', '>' = '!', '?' = "'", '=' = '-',
/// '<' = some char. Our Rust port uses normal characters.
pub fn draw_text_small(fonts: &SpriteSheet, text: &str, x: f32, y: f32, color: Color) {
    let mut cx = x;
    for ch in text.chars() {
        if let Some(fi) = char_to_font_index(ch) {
            if fi < fonts.num_sprites() {
                fonts.draw_colored(fi, cx, y, color);
                cx += fonts.sprite_width(fi) as f32;
            } else {
                cx += 8.0;
            }
        } else {
            cx += 8.0; // space
        }
    }
}

fn char_to_font_index(ch: char) -> Option<usize> {
    let ch_lower = ch.to_ascii_lowercase();
    match ch_lower {
        'a'..='z' => Some(26 + (ch_lower as u8 - b'a') as usize),
        '0'..='9' => Some(52 + (ch_lower as u8 - b'0') as usize),
        '.' => Some(62),
        ',' => Some(63),
        '-' => Some(64),
        '=' => Some(64), // same as '-' in original
        '!' => Some(66),
        '\'' => Some(67),
        ':' => Some(62), // ':' = '.' in original encoding
        ';' => Some(63), // ';' = ',' in original encoding
        '>' => Some(66), // '>' = '!' in original encoding
        '?' => Some(67), // '?' = '\'' in original encoding
        '*' | '#' | '+' => Some(65), // misc symbol
        '%' => Some(65),
        ' ' => None,
        _ => None,
    }
}

pub fn draw_text_centered(fonts: &SpriteSheet, text: &str, y: f32, color: Color) {
    // Calculate text width considering variable-width sprites
    let tw: f32 = text.chars().map(|ch| {
        if let Some(fi) = char_to_font_index(ch) {
            if fi < fonts.num_sprites() { fonts.sprite_width(fi) as f32 } else { 8.0 }
        } else { 8.0 }
    }).sum();
    draw_text_small(fonts, text, (SCREEN_W - tw) / 2.0, y, color);
}

pub fn draw_title_screen(tex: &Texture2D) {
    draw_texture_ex(tex, 0.0, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(SCREEN_W, SCREEN_H)),
        ..Default::default()
    });
}

pub fn draw_menu(fonts: &SpriteSheet, english: bool) {
    draw_rectangle(20.0, 70.0, 280.0, 120.0, Color::new(0.0, 0.0, 0.0, 0.6));
    if english {
        draw_text_centered(fonts, "PRESS 1 FOR ONE PLAYER GAME.", 80.0, WHITE);
        draw_text_centered(fonts, "PRESS 2 FOR TWO PLAYERS GAME.", 92.0, WHITE);
        draw_text_centered(fonts, "I. INFOS.", 104.0, YELLOW);
        draw_text_centered(fonts, "Z. INSTRUCTIONS.", 116.0, YELLOW);
        draw_text_centered(fonts, "R. SHOW RECORDS.", 128.0, YELLOW);
        draw_text_centered(fonts, "L. ITALIANO.", 140.0, SKYBLUE);
        draw_text_centered(fonts, "ESC EXITS.", 156.0, Color::new(0.7, 0.7, 0.7, 1.0));
    } else {
        draw_text_centered(fonts, "PREMI 1 PER UN GIOCATORE.", 80.0, WHITE);
        draw_text_centered(fonts, "PREMI 2 PER DUE GIOCATORI.", 92.0, WHITE);
        draw_text_centered(fonts, "I. INFORMAZIONI.", 104.0, YELLOW);
        draw_text_centered(fonts, "Z. ISTRUZIONI.", 116.0, YELLOW);
        draw_text_centered(fonts, "R. VEDI RECORDS.", 128.0, YELLOW);
        draw_text_centered(fonts, "L. ENGLISH.", 140.0, SKYBLUE);
        draw_text_centered(fonts, "ESC PER USCIRE.", 156.0, Color::new(0.7, 0.7, 0.7, 1.0));
    }
}
