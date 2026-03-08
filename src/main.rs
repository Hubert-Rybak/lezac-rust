//! Larax & Zaco - Rust Port
//! Original game by Stefano Zanobi (Zanobi Software, 1996)

mod assets;
mod game;
mod input;
mod level;
mod monsters;
mod player;
mod renderer;
mod sound;

use macroquad::prelude::*;
use assets::*;
use game::*;
use level::*;
use renderer::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Larax & Zaco".to_owned(),
        window_width: 960,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let ap = if std::path::Path::new("assets/BOMPAL.PAL").exists() { "assets" }
    else if std::path::Path::new("BOMPAL.PAL").exists() { "." }
    else {
        eprintln!("Cannot find game assets! Place original data files in assets/ directory.");
        std::process::exit(1);
    };

    let palette = Palette::load(&format!("{}/BOMPAL.PAL", ap));
    let player_sprites = SpriteSheet::load(&format!("{}/PROVA.SPR", ap), &palette);
    let misc_sprites = SpriteSheet::load(&format!("{}/BOMOMIMK.SPR", ap), &palette);
    let fonts = SpriteSheet::load(&format!("{}/FONTS.SPR", ap), &palette);
    let background = Background::load(&format!("{}/SFONLEF.ZBG", ap), &palette);
    let title_tex = load_title_screen(&format!("{}/CARO.CAR", ap), &palette);
    let levels = load_levels(&format!("{}/LIVELS.SCH", ap));
    let monster_defs = load_monster_defs(&format!("{}/GRAN.MST", ap));
    let records = load_records(&format!("{}/RECS.DAT", ap));
    let renderer = Renderer::new();
    let mut game = Game::new(levels, monster_defs, records);

    loop {
        let dt = get_frame_time().min(0.05);
        game.update(dt);

        renderer.begin();
        match game.state {
            GameState::TitleScreen => {
                draw_title_screen(&title_tex);
                if (game.game_time * 2.0) as i32 % 2 == 0 {
                    draw_text_centered(&fonts, "PRESS ANY KEY", 180.0, WHITE);
                }
            }
            GameState::MainMenu => {
                draw_title_screen(&title_tex);
                draw_menu(&fonts, game.english);
            }
            GameState::Info => {
                clear_background(Color::new(0.0, 0.0, 0.15, 1.0));
                if game.english {
                    draw_text_centered(&fonts, "LARAX E ZACO 1.0 SHAREWARE VERSION", 20.0, YELLOW);
                    draw_text_centered(&fonts, "IF YOU ENJOY THIS GAME", 40.0, WHITE);
                    draw_text_centered(&fonts, "PLEASE: REGISTER YOUR COPY", 60.0, WHITE);
                    draw_text_centered(&fonts, "STEFANO ZANOBI V.GRAMSCI 47", 86.0, SKYBLUE);
                    draw_text_centered(&fonts, "00015 MONTEROTONDO ROMA ITALY", 98.0, SKYBLUE);
                    draw_text_centered(&fonts, "SUPPORT HANDMADE SOFTWARE", 130.0, GREEN);
                } else {
                    draw_text_centered(&fonts, "LARAX E ZACO VERSIONE 1.0 SHAREWARE", 20.0, YELLOW);
                    draw_text_centered(&fonts, "SE QUESTO GIOCO VI DIVERTE", 40.0, WHITE);
                    draw_text_centered(&fonts, "REGISTRARE LA VOSTRA COPIA", 60.0, WHITE);
                    draw_text_centered(&fonts, "STEFANO ZANOBI V.GRAMSCI 47", 86.0, SKYBLUE);
                    draw_text_centered(&fonts, "00015 MONTEROTONDO ROMA", 98.0, SKYBLUE);
                    draw_text_centered(&fonts, "SUPPORTATE IL SOFTWARE ARTIGIANALE", 130.0, GREEN);
                }
                draw_text_centered(&fonts, "PRESS ANY KEY", 180.0, Color::new(0.5, 0.5, 0.5, 1.0));
            }
            GameState::Instructions => {
                clear_background(Color::new(0.0, 0.0, 0.15, 1.0));
                if game.english {
                    draw_text_centered(&fonts, "KEYS", 10.0, YELLOW);
                    draw_text_centered(&fonts, "     PLAYER1     PLAYER2", 26.0, WHITE);
                    draw_text_small(&fonts, "LEFT  :  Z        ARROWS", 30.0, 42.0, WHITE);
                    draw_text_small(&fonts, "RIGHT :  X           =", 30.0, 54.0, WHITE);
                    draw_text_small(&fonts, "DOWN  :  C           =", 30.0, 66.0, WHITE);
                    draw_text_small(&fonts, "JUMP  :  M           =", 30.0, 78.0, WHITE);
                    draw_text_small(&fonts, "FIRE  :  N           0", 30.0, 90.0, WHITE);
                    draw_text_centered(&fonts, "ESC QUITS GAME", 106.0, ORANGE);
                    draw_text_small(&fonts, "LEFT*RIGHT: CHANGE WEAPON", 20.0, 122.0, SKYBLUE);
                    draw_text_small(&fonts, "E,R: SET SCREEN WIDTH", 20.0, 134.0, SKYBLUE);
                    draw_text_small(&fonts, "S: TOGGLES BACKGROUND", 20.0, 146.0, SKYBLUE);
                } else {
                    draw_text_centered(&fonts, "TASTI", 10.0, YELLOW);
                    draw_text_small(&fonts, "SINISTRA: Z      FRECCE", 30.0, 42.0, WHITE);
                    draw_text_small(&fonts, "DESTRA  : X         =", 30.0, 54.0, WHITE);
                    draw_text_small(&fonts, "SCENDI  : C         =", 30.0, 66.0, WHITE);
                    draw_text_small(&fonts, "SALTA   : M         =", 30.0, 78.0, WHITE);
                    draw_text_small(&fonts, "FUOCO   : N         0", 30.0, 90.0, WHITE);
                    draw_text_centered(&fonts, "ESC ABBANDONA LA PARTITA", 106.0, ORANGE);
                }
                draw_text_centered(&fonts, "PRESS ANY KEY", 180.0, Color::new(0.5, 0.5, 0.5, 1.0));
            }
            GameState::Records => {
                clear_background(Color::new(0.0, 0.0, 0.15, 1.0));
                draw_text_centered(&fonts, "HIGH SCORES", 20.0, YELLOW);
                for (i, rec) in game.records.iter().enumerate() {
                    let s = format!("{}. {:8} {:6}", i + 1, rec.name, rec.score);
                    draw_text_centered(&fonts, &s, 50.0 + i as f32 * 18.0, WHITE);
                }
                draw_text_centered(&fonts, "PRESS ANY KEY", 180.0, Color::new(0.5, 0.5, 0.5, 1.0));
            }
            GameState::LevelIntro => {
                clear_background(Color::new(0.15, 0.15, 0.0, 1.0));
                for y in (0..200).step_by(2) {
                    for x in (0..320).step_by(2) {
                        if (x + y) % 4 == 0 {
                            draw_rectangle(x as f32, y as f32, 1.0, 1.0, Color::new(0.2, 0.2, 0.0, 0.3));
                        }
                    }
                }
                let msg = if game.english { format!("NOW ENTERING LEVEL {}", game.current_level + 1) }
                          else { format!("PREPARATI PER IL LIVELLO {}", game.current_level + 1) };
                draw_text_centered(&fonts, &msg, 95.0, WHITE);
            }
            GameState::Playing => {
                if game.show_background {
                    draw_background(&background, game.scroll_x, &palette);
                } else {
                    clear_background(Color::new(0.0, 0.0, 0.1, 1.0));
                }
                let li = game.current_level;
                if li < game.levels.len() {
                    draw_tiles(&game.levels[li], game.scroll_x, game.scroll_y, &palette, game.show_background);
                    draw_powerups(&game.powerups, &misc_sprites, game.scroll_x, game.scroll_y, game.game_time);
                    draw_bombs(&game.bombs, &misc_sprites, game.scroll_x, game.scroll_y, game.game_time);
                    draw_monsters(&game.monsters, &misc_sprites, game.scroll_x, game.scroll_y);
                    for p in &game.players {
                        draw_player(p, &player_sprites, game.scroll_x, game.scroll_y);
                    }
                    draw_debris(&game.debris, &palette, game.scroll_x, game.scroll_y);
                    draw_hud(&game.players, &game.levels[li], game.current_destruction_pct, &fonts, &palette, game.two_player);
                    for p in &game.players {
                        if !p.alive && p.lives > 0 && p.respawn_timer <= 0.0 {
                            let m = if game.english { "PRESS FIRE TO CONTINUE" } else { "PREMI FUOCO" };
                            draw_text_centered(&fonts, m, 80.0, RED);
                        }
                    }
                }
            }
            GameState::LevelComplete => {
                clear_background(Color::new(0.0, 0.15, 0.0, 1.0));
                let m = if game.english { "LEVEL COMPLETED" } else { "LIVELLO COMPLETATO" };
                draw_text_centered(&fonts, m, 80.0, GREEN);
                for (i, p) in game.players.iter().enumerate() {
                    draw_text_centered(&fonts, &format!("PLAYER {} SCORE: {}", i+1, p.score), 110.0 + i as f32*16.0, YELLOW);
                }
            }
            GameState::GameOver => {
                clear_background(Color::new(0.2, 0.0, 0.0, 1.0));
                draw_text_centered(&fonts, "GAME OVER", 80.0, RED);
                for (i, p) in game.players.iter().enumerate() {
                    draw_text_centered(&fonts, &format!("P{} SCORE: {}", i+1, p.score), 110.0 + i as f32*16.0, WHITE);
                }
            }
            GameState::FinalScore => {
                clear_background(Color::new(0.0, 0.0, 0.15, 1.0));
                let m = if game.english { "FINAL SCORE" } else { "PUNTEGGIO FINALE" };
                draw_text_centered(&fonts, m, 60.0, YELLOW);
                for (i, p) in game.players.iter().enumerate() {
                    let l = if game.english { format!("PLAYER {}", i+1) } else { format!("GIOCATORE {}", i+1) };
                    draw_text_centered(&fonts, &l, 90.0 + i as f32*40.0, SKYBLUE);
                    draw_text_centered(&fonts, &format!("{}", p.score), 106.0 + i as f32*40.0, WHITE);
                }
                draw_text_centered(&fonts, "PRESS ANY KEY", 180.0, Color::new(0.5, 0.5, 0.5, 1.0));
            }
        }
        renderer.end();
        next_frame().await;
    }
}
