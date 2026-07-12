//! Larax & Zaco - Rust Port
//! Original game by Stefano Zanobi (Zanobi Software, 1996)
#![allow(dead_code)]

mod assets;
mod game;
mod input;
mod level;
mod monsters;
mod original_rng;
mod player;
mod renderer;
mod sound;
mod touch;

use assets::*;
use game::*;
use level::*;
use macroquad::prelude::*;
use renderer::*;

const INFO_PAGES_EN: &[&[&str]] = &[
    &[
        "LARAX E ZACO 1.0 SHAREWARE VERSION",
        "",
        "IF YOU ENJOY THIS GAME",
        "AND YOU ARE WILLING TO KEEP IT",
        "PLEASE: REGISTER YOUR COPY",
        "SENDING A DONATION TO",
        ".",
        "STEFANO ZANOBI V.GRAMSCI 47",
        "00015 MONTEROTONDO ROMA",
        "ITALY",
        "",
        "FOR DONATIONS OF AT LEAST",
        "12 DOLLARS",
    ],
    &[
        "I WILL SEND YOU AN UPGRADED VERSION",
        "OF THE GAME.",
        "",
        "NEXT VERSIONS WILL CONTAIN",
        "MANY FUNNY PLAYING LEVELS!",
        "DIGITAL GRAPHICS!",
        "NEW DESTRUCTIVE WEAPONS",
        "AND GADGETS!",
        "NEW MONSTERS, NEW GUARDIANS",
        "AND MUCH MORE...",
    ],
    &[
        "IF YOU HAVE ANY SUGGESTION",
        "CONTACT THE AUTHOR.",
        "",
        "SUPPORT HANDMADE SOFTWARE !!",
    ],
];

const INFO_PAGES_IT: &[&[&str]] = &[
    &[
        "LARAX E ZACO VERSIONE 1.0 SHAREWARE",
        "",
        "SE QUESTO GIOCO VI DIVERTE",
        "E DECIDETE DI TENERLO",
        "SIETE PREGATI DI",
        "REGISTRARE LA VOSTRA COPIA",
        "INVIANDO UN CONTRIBUTO A",
        ".",
        "STEFANO ZANOBI V.GRAMSCI 47",
        "00015 MONTEROTONDO ROMA",
        "",
        "CON UN CONTRIBUTO MINIMO DI",
        "LIRE 20000",
    ],
    &[
        "RICEVERETE LA VERSIONE AGGIORNATA",
        "DEL GIOCO.",
        "",
        "LE PROSSIME VERSIONI CONTERRANNO",
        "PIU' LIVELLI DI GIOCO!",
        "GRAFICA DIGITALIZZATA!",
        "NUOVE ARMI E NUOVI GADGETS PER",
        "PLACARE OGNI BRAMA DI DISTRUZIONE!",
        "NUOVI MOSTRI, NUOVI GUARDIANI E",
        "ALTRO ANCORA!",
    ],
    &[
        "PER CHIARIMENTI O SUGGERIMENTI",
        "CONTATTARE L'AUTORE",
        "",
        "SUPPORTATE IL SOFTWARE ARTIGIANALE!",
    ],
];

const INSTRUCTION_PAGES_EN: &[&[&str]] = &[
    &[
        "KEYS",
        "",
        "         PLAYER1        PLAYER2",
        "",
        "LEFT    :    Z           ARROWS",
        "RIGHT   :    X              ,",
        "DOWN    :    C              ,",
        "JUMP    :    M              ,",
        "FIRE    :    N              0",
        "ESC QUITS GAME",
    ],
    &[
        "PRESS LEFT AND RIGHT KEYS TOGETHER:",
        "TO CHANGE YOUR WEAPON.",
        "E,R: SET PLAYING SCREEN WIDTH,",
        "IN ONE PLAYER GAME.",
        "S: TOGGLES BACKGROUND.",
        "PRESS DOWN ON SPECIAL PLATFORMS",
        "OR TELEPODS TO ACTIVATE THEM.",
    ],
    &[
        "",
        "YOU MUST COLLECT BONUSES AND",
        "DESTROY BUILDINGS TO PROCEED.",
        "THE CENTRAL WINDOW WILL TELL",
        "YOU THE NUMBER OF BONUS TO",
        "COLLECT AND THE PERCENTAGE",
        "OF BUILDINGS YOU MUST DESTROY",
        "TO COMPLETE THE LEVEL.",
    ],
    &[
        "YOU WILL START WITH THREE LIVES,",
        "WHEN YOU LOOSE ONE, REMEMBER",
        "THAT YOU CAN DECIDE",
        "TO CONTINUE, BY PRESSING FIRE OR",
        "TO RESTART THAT LEVEL FROM START,",
        "WAITING A FEW SECONDS.",
        "YOU CAN NOT REENTER A LEVEL IF",
        "THERE ARE NOT ENOUGH BONUSES",
        "TO COMPLETE IT.",
    ],
];

const INSTRUCTION_PAGES_IT: &[&[&str]] = &[
    &[
        "TASTI",
        "",
        "          GIOC.1          GIOC.2",
        "",
        "SINISTRA:    Z           FRECCE",
        "DESTRA  :    X              ,",
        "SCENDI  :    C              ,",
        "SALTA   :    M              ,",
        "FUOCO   :    N              0",
        "ESC ABBANDONA LA PARTITA",
    ],
    &[
        "PER CAMBIARE BOMBA:",
        "PREMI SINISTRA E DESTRA INSIEME.",
        "E,R: REGOLA LA LARGHEZZA DELLO",
        "SCHERMO, CON UNO SCHERMO SOLO.",
        "S: DISATTIVA O ATTIVA LO SFONDO.",
        "IL TASTO SCENDI ATTIVA",
        "LE PIATTAFORME SPECIALI.",
    ],
    &[
        "",
        "PER PROCEDERE NEL GIOCO DOVETE",
        "RACCOGLIERE BONUS E, SOPRATTUTTO,",
        "FAR SALTARE IN ARIA OGNI COSA:",
        "IL RIQUADRO CENTRALE VI INDICA",
        "IL NUMERO MINIMO DI BONUS DA",
        "RACCOGLIERE E LA PERCENTUALE DELLE",
        "COSTRUZIONI DA DISTRUGGERE PER",
        "COMPLETARE IL QUADRO.",
    ],
    &[
        "AVETE TRE VITE A DISPOSIZIONE",
        "QUANDO NE PERDETE UNA",
        "POTETE DECIDERE",
        "SE CONTINUARE PREMENDO FIRE O",
        "SE RIINIZIARE QUEL LIVELLO DA CAPO",
        "ATTENDENDO QUALCHE SECONDO.",
        "NON POTRETE RIENTRARE IN GIOCO SE",
        "NON CI SONO PIU'ABBASTANZA BONUS",
        "PER TERMINARE IL LIVELLO.",
    ],
];

fn draw_text_page(fonts: &SpriteSheet, lines: &[&str]) {
    let start_y = 10.0;
    for (i, line) in lines.iter().enumerate() {
        draw_text_centered(fonts, line, start_y + i as f32 * 10.0, WHITE);
    }
    draw_text_centered(
        fonts,
        "PRESS ANY KEY",
        188.0,
        Color::new(0.5, 0.5, 0.5, 1.0),
    );
}

fn format_high_score_entry_name(name: &str) -> String {
    let mut out = String::with_capacity(8);
    for ch in name.chars().take(8) {
        out.push(ch);
    }
    while out.len() < 8 {
        out.push('.');
    }
    out
}

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Larax & Zaco".to_owned(),
        window_width: 960,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    };
    // miniquad defaults to WebGL1, but its render-target path (used for our
    // 320x200 offscreen buffer) calls WebGL2-only GL functions, which leaves the
    // framebuffer non-functional and the page black on the web. Request WebGL2,
    // which every target browser — including iOS Safari 15+ — supports.
    conf.platform.webgl_version = macroquad::miniquad::conf::WebGLVersion::WebGL2;
    conf
}

#[macroquad::main(window_conf)]
async fn main() {
    #[cfg(target_arch = "wasm32")]
    let ap = "assets";
    #[cfg(not(target_arch = "wasm32"))]
    let ap = if std::path::Path::new("assets/BOMPAL.PAL").exists() {
        "assets"
    } else if std::path::Path::new("BOMPAL.PAL").exists() {
        "."
    } else {
        eprintln!("Cannot find game assets! Place original data files in assets/ directory.");
        std::process::exit(1);
    };

    let base_palette = Palette::load(&format!("{}/BOMPAL.PAL", ap));
    let mut palette = base_palette.clone();
    let player_sprites = SpriteSheet::load(&format!("{}/PROVA.SPR", ap), &palette);
    let misc_sprites = SpriteSheet::load(&format!("{}/BOMOMIMK.SPR", ap), &palette);
    let fonts = SpriteSheet::load(&format!("{}/FONTS.SPR", ap), &palette);
    let background = Background::load(&format!("{}/SFONLEF.ZBG", ap), &palette);
    let title_tex = load_title_screen(&format!("{}/CARO.CAR", ap), &palette);
    let levels = load_levels(&format!("{}/LIVELS.SCH", ap));
    let monster_defs = load_monster_defs(&format!("{}/GRAN.MST", ap));
    let record_path = format!("{}/RECS.DAT", ap);
    let records = load_records(&record_path);
    let sound = sound::SoundManager::load(&format!("{}/PROEFS.SON", ap)).await;
    let renderer = Renderer::new();
    let mut game = Game::new(levels, monster_defs, records, record_path, sound);

    /// 70 Hz fixed step matches the original VGA vsync timing (GAME_SPEC §8.1).
    const STEP: f32 = 1.0 / 70.0;
    let mut accum = 0.0_f32;
    let mut frame_counter = 0_u16;
    let mut palette_phase = 0_u8;

    loop {
        touch::update();
        accum += get_frame_time().min(0.25);
        // Cap to a few steps to avoid spiral-of-death after a stall.
        let mut steps = 0;
        while accum >= STEP && steps < 6 {
            frame_counter = advance_original_frame_word(frame_counter);
            game.update(STEP);
            if should_refresh_palette(frame_counter) {
                palette = base_palette.animated(palette_phase);
                player_sprites.update_palette(&palette);
                misc_sprites.update_palette(&palette);
                fonts.update_palette(&palette);
                title_tex.update_palette(&palette);
                background.update_palette(palette_phase);
                palette_phase = next_palette_cycle_phase(palette_phase);
            }
            accum -= STEP;
            steps += 1;
        }
        if steps == 6 {
            accum = 0.0;
        }

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
                let pages = if game.english {
                    INFO_PAGES_EN
                } else {
                    INFO_PAGES_IT
                };
                draw_text_page(&fonts, pages[game.info_page.min(pages.len() - 1)]);
            }
            GameState::Instructions => {
                clear_background(Color::new(0.0, 0.0, 0.15, 1.0));
                let pages = if game.english {
                    INSTRUCTION_PAGES_EN
                } else {
                    INSTRUCTION_PAGES_IT
                };
                draw_text_page(&fonts, pages[game.instructions_page.min(pages.len() - 1)]);
            }
            GameState::Records => {
                clear_background(Color::new(0.0, 0.0, 0.15, 1.0));
                draw_text_centered(&fonts, "RECORDS", 20.0, YELLOW);
                draw_text_centered(&fonts, "NAME     LEV SCORE", 38.0, SKYBLUE);
                for (i, rec) in game.records.iter().enumerate() {
                    let s = format!(
                        "{}. {:8} {:2} {:6}",
                        i + 1,
                        rec.name.to_ascii_uppercase(),
                        rec.level,
                        rec.score
                    );
                    draw_text_centered(&fonts, &s, 58.0 + i as f32 * 16.0, WHITE);
                }
                draw_text_centered(
                    &fonts,
                    "PRESS ANY KEY",
                    180.0,
                    Color::new(0.5, 0.5, 0.5, 1.0),
                );
            }
            GameState::LevelIntro => {
                clear_background(BLACK);
                let msg = if game.english {
                    format!("NOW ENTERING LEVEL {}", game.current_level + 1)
                } else {
                    format!("PREPARATI PER IL LIVELLO {}", game.current_level + 1)
                };
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
                    draw_tiles(
                        &game.levels[li],
                        game.scroll_x,
                        game.scroll_y,
                        &palette,
                        &misc_sprites,
                        game.show_background,
                    );
                    draw_powerups(
                        &game.powerups,
                        &misc_sprites,
                        game.scroll_x,
                        game.scroll_y,
                        game.game_time,
                    );
                    draw_bombs(
                        &game.bombs,
                        &player_sprites,
                        game.scroll_x,
                        game.scroll_y,
                        game.game_time,
                    );
                    draw_monsters(&game.monsters, &misc_sprites, game.scroll_x, game.scroll_y);
                    for p in &game.players {
                        draw_player(p, &player_sprites, game.scroll_x, game.scroll_y);
                    }
                    draw_collapsing(
                        &game.collapsing,
                        &misc_sprites,
                        game.scroll_x,
                        game.scroll_y,
                    );
                    draw_debris(&game.debris, &palette, game.scroll_x, game.scroll_y);
                    draw_screen_width_mask(game.screen_width_factor);
                    draw_hud(
                        &game.players,
                        &game.levels[li],
                        game.current_destruction_pct,
                        &fonts,
                        &player_sprites,
                        &palette,
                        game.two_player,
                    );
                    for p in &game.players {
                        if !p.alive && p.lives > 0 && p.respawn_timer <= 0.0 {
                            let m = if game.english {
                                "PRESS FIRE TO CONTINUE"
                            } else {
                                "PREMI FUOCO"
                            };
                            draw_text_centered(&fonts, m, 80.0, RED);
                        }
                    }
                    touch::draw_controls(&fonts);
                }
            }
            GameState::LevelComplete => {
                clear_background(Color::new(0.0, 0.15, 0.0, 1.0));
                let m = if game.english {
                    "LEVEL COMPLETED"
                } else {
                    "LIVELLO COMPLETATO"
                };
                draw_text_centered(&fonts, m, 80.0, GREEN);
                let destruction_line = if game.english {
                    format!("DESTRUCTION BONUS: {}", game.last_level_destruction_bonus)
                } else {
                    format!("BONUS DISTRUZIONE: {}", game.last_level_destruction_bonus)
                };
                draw_text_centered(&fonts, &destruction_line, 98.0, WHITE);
                for (i, p) in game.players.iter().enumerate() {
                    if let Some(bonus) = game.last_level_bomb_bonuses.get(i) {
                        let bomb_bonus_line = if game.english {
                            format!("PLAYER {} BONUS: {}", i + 1, bonus)
                        } else {
                            format!("GIOCATORE {} BONUS: {}", i + 1, bonus)
                        };
                        draw_text_centered(
                            &fonts,
                            &bomb_bonus_line,
                            116.0 + i as f32 * 32.0,
                            SKYBLUE,
                        );
                    }
                    let score_line = if game.english {
                        format!("PLAYER {} SCORE: {}", i + 1, p.score)
                    } else {
                        format!("GIOCATORE {} PUNTEGGIO: {}", i + 1, p.score)
                    };
                    draw_text_centered(&fonts, &score_line, 128.0 + i as f32 * 32.0, YELLOW);
                }
            }
            GameState::GameOver => {
                clear_background(Color::new(0.2, 0.0, 0.0, 1.0));
                draw_text_centered(&fonts, "GAME OVER", 80.0, RED);
                for (i, p) in game.players.iter().enumerate() {
                    let score_line = if game.english {
                        format!("P{} SCORE: {}", i + 1, p.score)
                    } else {
                        format!("G{} PUNTEGGIO: {}", i + 1, p.score)
                    };
                    draw_text_centered(&fonts, &score_line, 110.0 + i as f32 * 16.0, WHITE);
                }
            }
            GameState::HighScoreEntry => {
                clear_background(Color::new(0.0, 0.0, 0.15, 1.0));
                draw_text_centered(&fonts, "NEW HIGH SCORE", 46.0, YELLOW);
                if let Some(pending) = game.pending_high_score {
                    draw_text_centered(&fonts, &format!("SCORE {}", pending.score), 70.0, WHITE);
                    draw_text_centered(&fonts, &format!("LEVEL {}", pending.level), 86.0, SKYBLUE);
                }
                draw_text_centered(&fonts, "ENTER YOUR NAME", 116.0, WHITE);
                draw_text_centered(
                    &fonts,
                    &format_high_score_entry_name(&game.high_score_name),
                    138.0,
                    GREEN,
                );
                draw_text_centered(&fonts, "PRESS ENTER", 170.0, Color::new(0.5, 0.5, 0.5, 1.0));
            }
            GameState::FinalScore => {
                clear_background(Color::new(0.0, 0.0, 0.15, 1.0));
                let m = if game.english {
                    "FINAL SCORE"
                } else {
                    "PUNTEGGIO FINALE"
                };
                draw_text_centered(&fonts, m, 60.0, YELLOW);
                for (i, p) in game.players.iter().enumerate() {
                    let l = if game.english {
                        format!("PLAYER {}", i + 1)
                    } else {
                        format!("GIOCATORE {}", i + 1)
                    };
                    draw_text_centered(&fonts, &l, 90.0 + i as f32 * 40.0, SKYBLUE);
                    draw_text_centered(
                        &fonts,
                        &format!("{}", p.score),
                        106.0 + i as f32 * 40.0,
                        WHITE,
                    );
                }
                draw_text_centered(
                    &fonts,
                    "PRESS ANY KEY",
                    180.0,
                    Color::new(0.5, 0.5, 0.5, 1.0),
                );
            }
        }
        renderer.end();
        next_frame().await;
    }
}

fn advance_original_frame_word(frame_counter: u16) -> u16 {
    frame_counter.wrapping_add(1)
}

fn should_refresh_palette(frame_counter: u16) -> bool {
    frame_counter.is_multiple_of(5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_text_page_counts_match_menu_state_paging() {
        assert_eq!(INFO_PAGES_EN.len(), INFO_PAGE_COUNT);
        assert_eq!(INFO_PAGES_IT.len(), INFO_PAGE_COUNT);
        assert_eq!(INSTRUCTION_PAGES_EN.len(), INSTRUCTIONS_PAGE_COUNT);
        assert_eq!(INSTRUCTION_PAGES_IT.len(), INSTRUCTIONS_PAGE_COUNT);
    }

    #[test]
    fn original_text_pages_fit_mode_13h_width() {
        for page in INFO_PAGES_EN
            .iter()
            .chain(INFO_PAGES_IT)
            .chain(INSTRUCTION_PAGES_EN)
            .chain(INSTRUCTION_PAGES_IT)
        {
            for line in *page {
                assert!(
                    line.chars().count() <= 40,
                    "line is too wide for 320px 8x8 text: {line}"
                );
            }
        }
    }

    #[test]
    fn instruction_pages_preserve_extracted_blank_page_markers() {
        assert_eq!(INSTRUCTION_PAGES_EN[2][0], "");
        assert_eq!(INSTRUCTION_PAGES_IT[2][0], "");
    }

    #[test]
    fn italian_instruction_spacing_matches_extracted_original_text() {
        assert!(INSTRUCTION_PAGES_IT[3].contains(&"NON CI SONO PIU'ABBASTANZA BONUS"));
        assert!(!INSTRUCTION_PAGES_IT[3].contains(&"NON CI SONO PIU' ABBASTANZA BONUS"));
    }

    #[test]
    fn high_score_entry_name_renders_original_period_buffer() {
        assert_eq!(format_high_score_entry_name(""), "........");
        assert_eq!(format_high_score_entry_name("abc"), "abc.....");
        assert_eq!(format_high_score_entry_name("abcdefgh"), "abcdefgh");
        assert_eq!(format_high_score_entry_name("abcdefghi"), "abcdefgh");
    }

    #[test]
    fn palette_refresh_uses_incremented_original_frame_word() {
        let mut frame_counter = 0_u16;
        let mut refreshes = Vec::new();
        for _ in 0..10 {
            frame_counter = advance_original_frame_word(frame_counter);
            if should_refresh_palette(frame_counter) {
                refreshes.push(frame_counter);
            }
        }

        assert_eq!(refreshes, vec![5, 10]);

        frame_counter = u16::MAX;
        frame_counter = advance_original_frame_word(frame_counter);
        assert_eq!(frame_counter, 0);
        assert!(should_refresh_palette(frame_counter));
    }
}
