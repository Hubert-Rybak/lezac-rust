//! Rendering: tiles, sprites, HUD, backgrounds

use crate::assets::{Background, Palette, SpriteSheet, TitleCard};
use crate::game::Collapsing;
use crate::level::Level;
use crate::monsters::{Monster, MonsterType, Powerup, PowerupType};
use crate::player::{Bomb, BombType, Debris, Player};
use macroquad::prelude::*;

pub const SCREEN_W: f32 = 320.0;
pub const SCREEN_H: f32 = 200.0;
pub const TILE_SIZE: f32 = 8.0;
pub const HUD_HEIGHT: f32 = 40.0;
pub const PLAY_HEIGHT: f32 = SCREEN_H - HUD_HEIGHT;
const EXPLOSION_DRAW_DURATION: f32 = 0.5;

/// miniquad's render-target texture is stored with opposite vertical origin on
/// native OpenGL vs WebGL, so the offscreen 320x200 buffer must be flipped when
/// blitting to the screen on native but not on the web (where it would render
/// upside down).
const FLIP_RENDER_TARGET_Y: bool = !cfg!(target_arch = "wasm32");

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
        Renderer {
            render_target: rt,
            camera: cam,
        }
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
        draw_texture_ex(
            &self.render_target.texture,
            ox,
            oy,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dw, dh)),
                flip_y: FLIP_RENDER_TARGET_Y,
                ..Default::default()
            },
        );
    }
}

pub fn tile_color(tile: u8, palette: &Palette) -> Color {
    if tile == 0 {
        return Color::new(0.0, 0.0, 0.0, 0.0);
    }
    let pi = match tile {
        1 => 0x33,
        2 => 0x31,
        3 => 0x30,
        4 => 0x04,
        5 => 0x38,
        6 => 0x06,
        7 => 0x07,
        8 => 0x08,
        9 => 0x35,
        0x0a => 0x32,
        0x0b => 0x34,
        0x0c => 0x0C,
        0x0d => 0x36,
        0x0e => 0x0E,
        0x0f => 0x0F,
        _ => tile,
    };
    palette.to_color(pi)
}

pub fn draw_background(bg: &Background, scroll_x: f32, _palette: &Palette) {
    let off = (scroll_x * 0.5) % bg.width as f32;
    let src_h = (PLAY_HEIGHT / SCREEN_H * bg.height as f32).min(bg.height as f32);
    draw_texture_ex(
        &bg.texture,
        -off,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(bg.width as f32, PLAY_HEIGHT)),
            source: Some(Rect::new(0.0, 0.0, bg.width as f32, src_h)),
            ..Default::default()
        },
    );
    if off > 0.0 {
        draw_texture_ex(
            &bg.texture,
            bg.width as f32 - off,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(bg.width as f32, PLAY_HEIGHT)),
                source: Some(Rect::new(0.0, 0.0, bg.width as f32, src_h)),
                ..Default::default()
            },
        );
    }
}

/// Draw level tiles by indexing BOMOMIMK.SPR with the tile value.
/// Each non-zero tile byte directly names a sprite in BOMOMIMK.SPR (per GAME_SPEC §7).
/// Falls back to a palette-indexed rectangle if the sprite index is missing.
pub fn draw_tiles(
    level: &Level,
    sx: f32,
    sy: f32,
    palette: &Palette,
    tiles: &SpriteSheet,
    _show_bg: bool,
) {
    let stx = (sx / TILE_SIZE) as i32;
    let sty = (sy / TILE_SIZE) as i32;
    let etx = stx + (SCREEN_W / TILE_SIZE) as i32 + 2;
    let ety = sty + (PLAY_HEIGHT / TILE_SIZE) as i32 + 2;

    for ty in sty..ety {
        for tx in stx..etx {
            if tx < 0 || ty < 0 {
                continue;
            }
            let t = level.tile_at(tx as usize, ty as usize);
            if t == 0 {
                continue;
            }
            let px = tx as f32 * TILE_SIZE - sx;
            let py = ty as f32 * TILE_SIZE - sy;
            let si = t as usize;
            if si < tiles.num_sprites() && tiles.sprite_width(si) > 0 {
                tiles.draw(si, px, py);
            } else {
                let c = tile_color(t, palette);
                draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, c);
            }
        }
    }
}

pub fn draw_player(player: &Player, sprites: &SpriteSheet, sx: f32, sy: f32) {
    if !player.alive {
        return;
    }
    if player.invincible_frames > 0 && (player.invincible_frames / 4).is_multiple_of(2) {
        return;
    }
    let px = player.x - sx;
    let py = player.y - sy;
    let si = player.sprite_index();
    if si < sprites.num_sprites() {
        sprites.draw(si, px, py);
    } else {
        let c = if player.player_idx == 0 {
            Color::new(1.0, 0.2, 0.2, 1.0)
        } else {
            Color::new(0.2, 0.2, 1.0, 1.0)
        };
        draw_rectangle(px, py, 12.0, 16.0, c);
    }
}

pub fn draw_bombs(bombs: &[Bomb], sprites: &SpriteSheet, sx: f32, sy: f32, time: f32) {
    for b in bombs {
        let px = b.x - sx;
        let py = b.y - sy;
        if b.exploding {
            let si = explosion_sprite_index(b.explosion_timer);
            if si < sprites.num_sprites() {
                let x = px + 8.0 - sprites.sprite_width(si) as f32 * 0.5;
                let y = py + 8.0 - sprites.sprite_height(si) as f32 * 0.5;
                sprites.draw(si, x, y);
            } else {
                let r = b.bomb_type.radius() * (1.0 - b.explosion_timer / EXPLOSION_DRAW_DURATION);
                let a = b.explosion_timer / EXPLOSION_DRAW_DURATION;
                draw_circle(px + 8.0, py + 8.0, r, Color::new(1.0, 0.5 * a, 0.0, a));
                draw_circle(px + 8.0, py + 8.0, r * 0.6, Color::new(1.0, 1.0, 0.0, a));
            }
        } else {
            let flash = b.timer < 1.0 && (time * 8.0).sin() > 0.0;
            let si = b.bomb_type.sprite_index();
            if si < sprites.num_sprites() {
                if flash {
                    sprites.draw_colored(si, px, py, YELLOW);
                } else {
                    sprites.draw(si, px, py);
                }
            } else {
                let c = if flash {
                    YELLOW
                } else {
                    match b.bomb_type {
                        BombType::Small => Color::new(0.4, 0.4, 0.4, 1.0),
                        BombType::Medium => Color::new(0.6, 0.4, 0.2, 1.0),
                        BombType::Large => Color::new(0.8, 0.2, 0.2, 1.0),
                        BombType::Super => Color::new(0.2, 0.8, 0.2, 1.0),
                    }
                };
                draw_circle(px + 8.0, py + 8.0, 6.0, c);
                draw_line(px + 8.0, py + 2.0, px + 12.0, py - 2.0, 1.0, ORANGE);
            }
        }
    }
}

fn explosion_sprite_index(remaining: f32) -> usize {
    let progress = 1.0 - (remaining / EXPLOSION_DRAW_DURATION).clamp(0.0, 1.0);
    40 + (progress * 4.0).floor().min(3.0) as usize
}

/// Draw falling rubble as overlay BOMOMIMK sprites at the crack-stage index.
pub fn draw_collapsing(collapsing: &[Collapsing], tiles: &SpriteSheet, sx: f32, sy: f32) {
    for c in collapsing {
        let px = c.tx as f32 * TILE_SIZE - sx;
        let py = c.ty as f32 * TILE_SIZE + c.y_offset - sy;
        let si = c.stage as usize;
        if si < tiles.num_sprites() {
            tiles.draw(si, px, py);
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
        if !m.alive {
            continue;
        }
        let px = m.x - sx;
        let py = m.y - sy;
        let si = m.sprite_index();
        if si < sprites.num_sprites() {
            if m.facing_right {
                sprites.draw(si, px, py);
            } else {
                sprites.draw_flipped(si, px + sprites.sprite_width(si) as f32, py);
            }
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
        if !p.alive {
            continue;
        }
        let px = p.x - sx;
        let py = p.y - sy + (time * 3.0).sin() * 2.0;
        let si = p.powerup_type.sprite_index();
        if si < sprites.num_sprites() {
            sprites.draw(si, px, py);
        } else {
            let c = match p.powerup_type {
                PowerupType::Present => GOLD,
                PowerupType::BonusToken => GOLD,
                PowerupType::HotDog => ORANGE,
                PowerupType::FirstAid => RED,
                PowerupType::Invincibility => Color::new(1.0, 1.0, 1.0, 1.0),
                PowerupType::YellowBombBox => YELLOW,
                PowerupType::GreenBombBox => GREEN,
                PowerupType::JollyCloud => SKYBLUE,
                PowerupType::BigDiamond => Color::new(0.0, 1.0, 1.0, 1.0),
            };
            draw_triangle(
                vec2(px + 8.0, py),
                vec2(px, py + 8.0),
                vec2(px + 16.0, py + 8.0),
                c,
            );
            draw_triangle(
                vec2(px + 8.0, py + 16.0),
                vec2(px, py + 8.0),
                vec2(px + 16.0, py + 8.0),
                c,
            );
        }
    }
}

pub fn draw_hud(
    players: &[Player],
    level: &Level,
    destr_pct: f32,
    fonts: &SpriteSheet,
    player_sprites: &SpriteSheet,
    _palette: &Palette,
    two_player: bool,
) {
    let hy = PLAY_HEIGHT;
    draw_rectangle(
        0.0,
        hy,
        SCREEN_W,
        HUD_HEIGHT,
        Color::new(0.0, 0.0, 0.3, 1.0),
    );
    draw_line(0.0, hy, SCREEN_W, hy, 1.0, Color::new(0.3, 0.3, 0.8, 1.0));

    if two_player {
        for (i, p) in players.iter().take(2).enumerate() {
            draw_compact_player_hud(p, i, hy + 2.0 + i as f32 * 18.0, fonts, player_sprites);
        }
        draw_level_targets(
            level,
            destr_pct,
            fonts,
            player_sprites,
            SCREEN_W - 68.0,
            hy + 4.0,
        );
        return;
    }

    if let Some(p) = players.first() {
        // Sprite 89 is the "LIVES" label (21x7); fall back silently if absent.
        let lives_label = 89;
        if lives_label < player_sprites.num_sprites() {
            player_sprites.draw(lives_label, 4.0, hy + 18.0);
        }
        // Sprite 90: heart/life icon (12x10), repeated once per life.
        let life_sprite = 90;
        let lw = if life_sprite < player_sprites.num_sprites() {
            player_sprites.sprite_width(life_sprite) as f32 + 1.0
        } else {
            10.0
        };
        for i in 0..p.lives.max(0) as usize {
            let lx = 4.0 + i as f32 * lw;
            if life_sprite < player_sprites.num_sprites() {
                player_sprites.draw(life_sprite, lx, hy + 28.0);
            } else {
                draw_text_small(fonts, "a", lx, hy + 28.0, GREEN);
            }
        }

        // Energy: sprite 79 left endcap, 80 fill, 81 right endcap (best-effort).
        // If the sprite slots aren't where we expect we just draw a coloured bar.
        let bar_x = 50.0;
        let bar_y = hy + 4.0;
        let bar_w = 60.0;
        if 79 < player_sprites.num_sprites() {
            player_sprites.draw(79, bar_x, bar_y);
        } else {
            draw_rectangle_lines(bar_x, bar_y, bar_w, 6.0, 1.0, SKYBLUE);
        }
        let fw = (p.energy / p.max_energy) * (bar_w - 2.0);
        draw_rectangle(bar_x + 1.0, bar_y + 1.0, fw, 4.0, YELLOW);

        // Score
        draw_text_small(fonts, &format!("{:06}", p.score), 50.0, hy + 14.0, WHITE);

        // Current bomb indicator: original uses one-based bomb type + 0x39.
        let cx = SCREEN_W / 2.0 - 30.0;
        let bomb_sprite = p.current_bomb.sprite_index();
        if bomb_sprite < player_sprites.num_sprites() {
            player_sprites.draw(bomb_sprite, cx, hy + 2.0);
        } else {
            let bc = match p.current_bomb {
                BombType::Small => Color::new(0.4, 0.4, 0.4, 1.0),
                BombType::Medium => Color::new(0.6, 0.4, 0.2, 1.0),
                BombType::Large => Color::new(0.8, 0.2, 0.2, 1.0),
                BombType::Super => Color::new(0.2, 0.8, 0.2, 1.0),
            };
            draw_rectangle(cx, hy + 4.0, 12.0, 12.0, bc);
        }
        draw_text_small(
            fonts,
            &format!("{:2}", p.bombs[p.current_bomb as usize]),
            cx + 18.0,
            hy + 12.0,
            WHITE,
        );
    }

    draw_level_targets(
        level,
        destr_pct,
        fonts,
        player_sprites,
        SCREEN_W / 2.0 + 14.0,
        hy + 4.0,
    );
}

fn draw_compact_player_hud(
    player: &Player,
    idx: usize,
    y: f32,
    fonts: &SpriteSheet,
    player_sprites: &SpriteSheet,
) {
    let label = if idx == 0 { "P1" } else { "P2" };
    draw_text_small(
        fonts,
        label,
        4.0,
        y + 1.0,
        if idx == 0 { RED } else { SKYBLUE },
    );
    draw_text_small(fonts, &format!("{:06}", player.score), 24.0, y + 1.0, WHITE);

    let bar_x = 78.0;
    let bar_w = 38.0;
    draw_rectangle_lines(bar_x, y + 2.0, bar_w, 6.0, 1.0, SKYBLUE);
    let fw = (player.energy / player.max_energy).clamp(0.0, 1.0) * (bar_w - 2.0);
    draw_rectangle(bar_x + 1.0, y + 3.0, fw, 4.0, YELLOW);

    let bomb_x = 124.0;
    let bomb_sprite = player.current_bomb.sprite_index();
    if bomb_sprite < player_sprites.num_sprites() {
        player_sprites.draw(bomb_sprite, bomb_x, y - 1.0);
    }
    draw_text_small(
        fonts,
        &format!("{:2}", player.bombs[player.current_bomb as usize]),
        bomb_x + 14.0,
        y + 3.0,
        WHITE,
    );
    draw_text_small(
        fonts,
        &format!("L{}", player.lives.max(0)),
        154.0,
        y + 1.0,
        GREEN,
    );
}

fn draw_level_targets(
    level: &Level,
    destr_pct: f32,
    fonts: &SpriteSheet,
    player_sprites: &SpriteSheet,
    bx: f32,
    y: f32,
) {
    // Bonus/destruction targets — sprite 85/86 if available as decoration.
    if 85 < player_sprites.num_sprites() {
        player_sprites.draw(85, bx, y);
    } else {
        draw_circle(bx + 4.0, y + 4.0, 4.0, GOLD);
    }
    draw_text_small(
        fonts,
        &format!("{:02}", level.bonus_target),
        bx + 22.0,
        y + 4.0,
        WHITE,
    );
    if 86 < player_sprites.num_sprites() {
        player_sprites.draw(86, bx, y + 14.0);
    } else {
        draw_text_small(fonts, "*", bx, y + 20.0, ORANGE);
    }
    draw_text_small(
        fonts,
        &format!("{:02}", level.destruction_pct),
        bx + 22.0,
        y + 18.0,
        WHITE,
    );
    draw_text_small(
        fonts,
        &format!("{:02}", destr_pct as u32),
        bx + 44.0,
        y + 18.0,
        YELLOW,
    );
}

/// Draw text using the font sprite sheet.
/// The font file contains:
///   Sprites 0-25:  10x10 large/shadow font (a-z)
///   Sprites 26-51: 8x8 small font (a-z)
///   Sprites 52-61: 8x8 small font (0-9)
///   Sprites 62-67: 8x8 small font (punctuation: . : ; , ! ')
/// The original text routine takes a one-based first-letter sprite index.
/// Its shipped menu/info/high-score callers pass 0x1b, so normal UI text
/// starts at zero-based sprite 26 and uses the 8x8 font.
/// Text input uses lowercase internally. The game's original strings use
/// custom encoding: ':' = '.', ';' = ':', '=' = ',', '>' = '!', '?' = "'",
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

fn original_alnum_font_index(first_letter_sprite_one_based: u8, ch: char) -> Option<usize> {
    let first_letter = first_letter_sprite_one_based as usize - 1;
    let ch_lower = ch.to_ascii_lowercase();
    match ch_lower {
        'a'..='z' => Some(first_letter + (ch_lower as u8 - b'a') as usize),
        '0'..='9' => Some(first_letter + 26 + (ch_lower as u8 - b'0') as usize),
        _ => None,
    }
}

fn char_to_font_index(ch: char) -> Option<usize> {
    if let Some(idx) = original_alnum_font_index(0x1b, ch) {
        return Some(idx);
    }

    match ch {
        '.' => Some(62),
        ':' => Some(63),
        ';' => Some(64),
        ',' => Some(65),
        '=' => Some(65), // original encoded comma
        '!' => Some(66),
        '\'' => Some(67),
        '>' => Some(66), // original encoded '!'
        '?' => Some(67), // original encoded apostrophe
        ' ' => None,
        _ => None,
    }
}

pub fn draw_text_centered(fonts: &SpriteSheet, text: &str, y: f32, color: Color) {
    // Calculate text width considering variable-width sprites
    let tw: f32 = text
        .chars()
        .map(|ch| {
            if let Some(fi) = char_to_font_index(ch) {
                if fi < fonts.num_sprites() {
                    fonts.sprite_width(fi) as f32
                } else {
                    8.0
                }
            } else {
                8.0
            }
        })
        .sum();
    draw_text_small(fonts, text, (SCREEN_W - tw) / 2.0, y, color);
}

/// Draw the CARO.CAR title card. It's 132x64, drawn centered horizontally at
/// x=94 (per the original game binary), near the top of the screen.
/// Mask off the left and right of the playfield so the visible width
/// matches `factor` (1.0 = full 320 px, 0.5 = half). Per GAME_SPEC §11.4.
pub fn draw_screen_width_mask(factor: f32) {
    if factor >= 0.999 {
        return;
    }
    let visible = (SCREEN_W * factor).round();
    let bar = ((SCREEN_W - visible) * 0.5).max(0.0);
    if bar <= 0.0 {
        return;
    }
    draw_rectangle(0.0, 0.0, bar, PLAY_HEIGHT, BLACK);
    draw_rectangle(SCREEN_W - bar, 0.0, bar, PLAY_HEIGHT, BLACK);
}

pub fn draw_title_screen(card: &TitleCard) {
    let dx = (SCREEN_W - card.width as f32) / 2.0;
    let dy = 12.0;
    draw_texture_ex(
        &card.texture,
        dx,
        dy,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(card.width as f32, card.height as f32)),
            ..Default::default()
        },
    );
}

pub fn draw_menu(fonts: &SpriteSheet, english: bool) {
    draw_rectangle(20.0, 70.0, 280.0, 120.0, Color::new(0.0, 0.0, 0.0, 0.6));
    if english {
        draw_text_centered(fonts, "PRESS 1 FOR ONE PLAYER GAME.", 80.0, WHITE);
        draw_text_centered(fonts, "PRESS 2 FOR TWO PLAYERS GAME.", 92.0, WHITE);
        draw_text_centered(fonts, "I: INFOS.", 104.0, YELLOW);
        draw_text_centered(fonts, "Z: INSTRUCTIONS.", 116.0, YELLOW);
        draw_text_centered(fonts, "R: SHOW RECORDS.", 128.0, YELLOW);
        draw_text_centered(fonts, "L: ITALIANO.", 140.0, SKYBLUE);
        draw_text_centered(fonts, "ESC EXITS.", 156.0, Color::new(0.7, 0.7, 0.7, 1.0));
    } else {
        draw_text_centered(fonts, "PREMI 1 PER UN GIOCATORE.", 80.0, WHITE);
        draw_text_centered(fonts, "PREMI 2 PER DUE GIOCATORI.", 92.0, WHITE);
        draw_text_centered(fonts, "I: INFORMAZIONI.", 104.0, YELLOW);
        draw_text_centered(fonts, "Z: ISTRUZIONI.", 116.0, YELLOW);
        draw_text_centered(fonts, "R: VEDI RECORDS.", 128.0, YELLOW);
        draw_text_centered(fonts, "L: ENGLISH.", 140.0, SKYBLUE);
        draw_text_centered(
            fonts,
            "ESC PER USCIRE.",
            156.0,
            Color::new(0.7, 0.7, 0.7, 1.0),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explosion_sprite_sequence_uses_original_prova_frames() {
        assert_eq!(explosion_sprite_index(0.5), 40);
        assert_eq!(explosion_sprite_index(0.375), 41);
        assert_eq!(explosion_sprite_index(0.25), 42);
        assert_eq!(explosion_sprite_index(0.125), 43);
        assert_eq!(explosion_sprite_index(0.0), 43);
    }

    #[test]
    fn small_text_mapping_matches_original_one_based_font_offset() {
        assert_eq!(original_alnum_font_index(0x1b, 'a'), Some(26));
        assert_eq!(original_alnum_font_index(0x1b, 'z'), Some(51));
        assert_eq!(original_alnum_font_index(0x1b, '0'), Some(52));
        assert_eq!(original_alnum_font_index(0x1b, '9'), Some(61));

        assert_eq!(char_to_font_index('A'), Some(26));
        assert_eq!(char_to_font_index('Z'), Some(51));
        assert_eq!(char_to_font_index('0'), Some(52));
        assert_eq!(char_to_font_index('9'), Some(61));
    }

    #[test]
    fn large_font_offset_maps_to_decorative_letters() {
        assert_eq!(original_alnum_font_index(1, 'a'), Some(0));
        assert_eq!(original_alnum_font_index(1, 'z'), Some(25));
        assert_eq!(original_alnum_font_index(1, '0'), Some(26));
        assert_eq!(original_alnum_font_index(1, '9'), Some(35));
    }

    #[test]
    fn small_punctuation_mapping_matches_font_sprite_shapes() {
        assert_eq!(char_to_font_index('.'), Some(62));
        assert_eq!(char_to_font_index(':'), Some(63));
        assert_eq!(char_to_font_index(';'), Some(64));
        assert_eq!(char_to_font_index(','), Some(65));
        assert_eq!(char_to_font_index('='), Some(65));
        assert_eq!(char_to_font_index('!'), Some(66));
        assert_eq!(char_to_font_index('\''), Some(67));
    }
}
