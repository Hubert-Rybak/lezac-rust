use crate::assets::{save_records, HighScore};
use crate::input::PlayerInput;
use crate::level::*;
use crate::monsters::*;
use crate::original_rng::OriginalRng;
use crate::player::*;
use crate::renderer::{PLAY_HEIGHT, SCREEN_W};
use crate::sound::*;

const TILE_SIZE: f32 = 8.0;
const PLAYER_W: f32 = 12.0;
const PLAYER_H: f32 = 16.0;
const EXPLOSION_DURATION: f32 = 0.5;
const DEATH_RESTART_WAIT: f32 = 230.0 / 70.0;
const LEVEL_COMPLETE_DELAY: f32 = 100.0 / 70.0;
const TILE_BONUS_MIN: u8 = 0x13;
const TILE_BONUS_MAX: u8 = 0x1d;
const MIN_VISIBLE_TILE_COLUMNS: u16 = 0x15;
const MAX_VISIBLE_TILE_COLUMNS: u16 = 0x28;
/// 70 Hz frames between crack stages 0x76 → 0x77 → 0x78 → 0x79 → cleared.
const COLLAPSE_FRAME_TIME: f32 = 4.0 / 70.0;
/// Per-frame gravity for falling rubble (in pixels/frame at 70 Hz).
const RUBBLE_GRAVITY: f32 = 0x40 as f32 / 256.0;
const RUBBLE_MAX_FALL: f32 = 0x7FF as f32 / 256.0;
const MONSTER_CONTACT_DAMAGE_PER_FRAME: f32 = 1.0;
const ORIGINAL_MOTION_TRIG_SCALE: f64 = 256.0;
pub const INFO_PAGE_COUNT: usize = 3;
pub const INSTRUCTIONS_PAGE_COUNT: usize = 4;

fn original_motion_random_words(
    fields: &[MonsterMotionRuntimeFields],
    original_rng: &mut OriginalRng,
) -> Vec<(i16, i16)> {
    fields
        .iter()
        .map(|field| {
            if field.uses_absolute_velocity() {
                (0, 0)
            } else {
                (
                    original_rng.next_word() as i16,
                    original_rng.next_word() as i16,
                )
            }
        })
        .collect()
}

fn original_motion_trig_words() -> [i16; 128] {
    // Original `FUN_1000_26e8` builds 128 six-byte Turbo Pascal real entries
    // at 0x7bda, and `FUN_1000_432a` converts them back to integer offsets via
    // the Pascal real helpers. The table shape and phase pairing are pinned;
    // exact TP-real rounding remains an artifact gap.
    let mut words = [0; 128];
    let len = words.len() as f64;
    for (phase, word) in words.iter_mut().enumerate() {
        let radians = phase as f64 * std::f64::consts::TAU / len;
        *word = (radians.sin() * ORIGINAL_MOTION_TRIG_SCALE).round() as i16;
    }
    words
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    TitleScreen,
    MainMenu,
    Instructions,
    Records,
    Info,
    LevelIntro,
    Playing,
    LevelComplete,
    GameOver,
    HighScoreEntry,
    FinalScore,
}

/// A piece of falling rubble. Removed from the level grid (so collisions and
/// destruction% see it as gone) and rendered as an overlay sprite that cycles
/// through crack stages 0x76→0x79 while accumulating vertical velocity per
/// §11.2. Lands when it hits a solid tile beneath, or vanishes off the map.
#[derive(Clone, Copy)]
pub struct Collapsing {
    pub tx: usize,
    pub ty: usize,
    pub stage: u8, // 0x76, 0x77, 0x78, 0x79
    pub timer: f32,
    pub vy: f32,
    pub y_offset: f32,
}

#[derive(Clone, Copy)]
pub struct PendingHighScore {
    pub score: u32,
    pub level: u8,
}

pub struct Game {
    pub state: GameState,
    pub english: bool,
    pub two_player: bool,
    pub current_level: usize,
    pub levels: Vec<Level>,
    pub monster_defs: Vec<MonsterTemplate>,
    pub monster_motion_runtime_fields: Vec<MonsterMotionRuntimeFields>,
    pub players: Vec<Player>,
    pub monsters: Vec<Monster>,
    pub monster_spawn_controllers: Vec<MonsterSpawn>,
    pub bombs: Vec<Bomb>,
    pub debris: Vec<Debris>,
    pub powerups: Vec<Powerup>,
    pub collapsing: Vec<Collapsing>,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub initial_destruction_tile_count: usize,
    pub current_destruction_pct: f32,
    pub state_timer: f32,
    pub game_time: f32,
    pub frame_counter: u16,
    pub show_background: bool,
    pub screen_width_factor: f32,
    pub records: Vec<HighScore>,
    pub record_path: String,
    pub pending_high_score: Option<PendingHighScore>,
    pub high_score_name: String,
    pub last_level_destruction_bonus: u32,
    pub last_level_bomb_bonuses: Vec<u32>,
    level_completion_bonuses_awarded: bool,
    pub original_rng: OriginalRng,
    pub info_page: usize,
    pub instructions_page: usize,
    pub sound: SoundManager,
    pub p1_start_x: f32,
    pub p1_start_y: f32,
    pub p2_start_x: f32,
    pub p2_start_y: f32,
}

impl Game {
    pub fn new(
        levels: Vec<Level>,
        monster_defs: Vec<MonsterTemplate>,
        records: Vec<HighScore>,
        record_path: String,
        sound: SoundManager,
    ) -> Self {
        Game {
            state: GameState::TitleScreen,
            english: false,
            two_player: false,
            current_level: 0,
            levels,
            monster_motion_runtime_fields: monster_motion_runtime_fields_from_templates(
                &monster_defs,
            ),
            monster_defs,
            players: Vec::new(),
            monsters: Vec::new(),
            monster_spawn_controllers: Vec::new(),
            bombs: Vec::new(),
            debris: Vec::new(),
            powerups: Vec::new(),
            collapsing: Vec::new(),
            scroll_x: 0.0,
            scroll_y: 0.0,
            initial_destruction_tile_count: 0,
            current_destruction_pct: 0.0,
            state_timer: 0.0,
            game_time: 0.0,
            frame_counter: 0,
            show_background: true,
            screen_width_factor: 1.0,
            records,
            record_path,
            pending_high_score: None,
            high_score_name: String::new(),
            last_level_destruction_bonus: 0,
            last_level_bomb_bonuses: Vec::new(),
            level_completion_bonuses_awarded: false,
            original_rng: OriginalRng::from_system_time(),
            info_page: 0,
            instructions_page: 0,
            sound,
            p1_start_x: 40.0,
            p1_start_y: 40.0,
            p2_start_x: 80.0,
            p2_start_y: 40.0,
        }
    }

    pub fn start_game(&mut self, two_player: bool) {
        self.two_player = two_player;
        self.current_level = 0;
        self.players.clear();
        self.players.push(Player::new(0.0, 0.0, 0));
        if two_player {
            self.players.push(Player::new(0.0, 0.0, 1));
        }
        self.start_level();
        self.state = GameState::LevelIntro;
        self.state_timer = 3.0;
    }

    pub fn start_level(&mut self) {
        let li = self.current_level;
        if li >= self.levels.len() {
            self.state = GameState::FinalScore;
            return;
        }
        self.levels[li].reset();
        self.find_start_positions();
        if let Some(p) = self.players.get_mut(0) {
            p.bonuses_collected = 0;
            p.respawn(self.p1_start_x, self.p1_start_y);
        }
        if let Some(p) = self.players.get_mut(1) {
            p.bonuses_collected = 0;
            p.respawn(self.p2_start_x, self.p2_start_y);
        }
        self.monsters.clear();
        self.monster_spawn_controllers = self.levels[li].monsters.clone();
        self.monster_motion_runtime_fields =
            monster_motion_runtime_fields_from_templates(&self.monster_defs);
        self.bombs.clear();
        self.debris.clear();
        self.powerups.clear();
        self.collapsing.clear();
        // Pre-spawn only non-start records from the seven-byte level entity
        // table. The shipped data uses raw[6] as a flag byte: values 1/2 are
        // consumed by find_start_positions, and non-start records carry 0.
        for b in &self.levels[li].bonuses {
            if b.player_start_mask() != 0 {
                continue;
            }
            let tx = b.x_px() as f32;
            let ty = b.y_px() as f32 - 8.0;
            let pt = PowerupType::from_bonus_id(b.spawned_bonus_id());
            let mut p = Powerup::new(tx, ty, pt).level_goal();
            p.vy = 0.0;
            self.powerups.push(p);
        }
        self.initial_destruction_tile_count =
            self.levels[li].initial_destruction_tile_count as usize;
        self.current_destruction_pct = 0.0;
        self.last_level_destruction_bonus = 0;
        self.last_level_bomb_bonuses.clear();
        self.level_completion_bonuses_awarded = false;
        self.scroll_x = self.levels[li].scroll_x as f32 * TILE_SIZE;
        self.scroll_y = self.levels[li].scroll_y as f32 * TILE_SIZE;
    }

    fn find_start_positions(&mut self) {
        let lev = &self.levels[self.current_level];
        self.p1_start_x = 40.0;
        self.p1_start_y = 40.0;
        self.p2_start_x = 80.0;
        self.p2_start_y = 40.0;
        let mut found_p1 = false;
        let mut found_p2 = false;
        for spawn in &lev.bonuses {
            if !found_p1 && spawn.starts_player(0) {
                self.p1_start_x = spawn.x_px() as f32;
                self.p1_start_y = spawn.y_px() as f32;
                found_p1 = true;
            }
            if !found_p2 && spawn.starts_player(1) {
                self.p2_start_x = spawn.x_px() as f32;
                self.p2_start_y = spawn.y_px() as f32;
                found_p2 = true;
            }
        }
        if found_p1 || found_p2 {
            return;
        }
        for x in 2..lev.width / 4 {
            for y in 1..lev.height.saturating_sub(2) {
                if !lev.is_solid(x, y) && y > 0 && !lev.is_solid(x, y - 1) && lev.is_solid(x, y + 1)
                {
                    self.p1_start_x = x as f32 * TILE_SIZE;
                    self.p1_start_y = (y as f32 * TILE_SIZE) - 16.0;
                    self.p2_start_x = (x + 4) as f32 * TILE_SIZE;
                    self.p2_start_y = (y as f32 * TILE_SIZE) - 16.0;
                    return;
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.game_time += dt;
        self.advance_frame_counter();
        match self.state {
            GameState::TitleScreen => {
                if macroquad::input::get_last_key_pressed().is_some() {
                    self.state = GameState::MainMenu;
                }
            }
            GameState::MainMenu => {
                use macroquad::input::*;
                if is_key_pressed(KeyCode::Key1) {
                    self.sound.play(SoundEffect::MenuSelect);
                    self.start_game(false);
                } else if is_key_pressed(KeyCode::Key2) {
                    self.sound.play(SoundEffect::MenuSelect);
                    self.start_game(true);
                } else if is_key_pressed(KeyCode::L) {
                    self.sound.play(SoundEffect::MenuSelect);
                    self.english = !self.english;
                } else if is_key_pressed(KeyCode::I) {
                    self.sound.play(SoundEffect::MenuSelect);
                    self.info_page = 0;
                    self.state = GameState::Info;
                } else if is_key_pressed(KeyCode::Z) {
                    self.sound.play(SoundEffect::MenuSelect);
                    self.instructions_page = 0;
                    self.state = GameState::Instructions;
                } else if is_key_pressed(KeyCode::R) {
                    self.sound.play(SoundEffect::MenuSelect);
                    self.state = GameState::Records;
                } else if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
            }
            GameState::Info => {
                if macroquad::input::get_last_key_pressed().is_some() {
                    self.sound.play(SoundEffect::MenuSelect);
                    self.info_page += 1;
                    if self.info_page >= INFO_PAGE_COUNT {
                        self.info_page = 0;
                        self.state = GameState::MainMenu;
                    }
                }
            }
            GameState::Instructions => {
                if macroquad::input::get_last_key_pressed().is_some() {
                    self.sound.play(SoundEffect::MenuSelect);
                    self.instructions_page += 1;
                    if self.instructions_page >= INSTRUCTIONS_PAGE_COUNT {
                        self.instructions_page = 0;
                        self.state = GameState::MainMenu;
                    }
                }
            }
            GameState::Records => {
                if macroquad::input::get_last_key_pressed().is_some() {
                    self.sound.play(SoundEffect::MenuSelect);
                    self.state = GameState::MainMenu;
                }
            }
            GameState::LevelIntro => {
                self.state_timer -= dt;
                if self.state_timer <= 0.0 {
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => self.update_playing(dt),
            GameState::LevelComplete => {
                self.state_timer -= dt;
                if self.state_timer <= 0.0 {
                    self.current_level += 1;
                    if self.current_level >= 7 {
                        self.finish_run();
                    } else {
                        self.start_level();
                        self.state = GameState::LevelIntro;
                        self.state_timer = 3.0;
                    }
                }
            }
            GameState::GameOver => {
                self.state_timer -= dt;
                if self.state_timer <= 0.0 || macroquad::input::get_last_key_pressed().is_some() {
                    self.finish_run();
                }
            }
            GameState::HighScoreEntry => self.update_high_score_entry(),
            GameState::FinalScore => {
                self.state_timer -= dt;
                if self.state_timer <= 0.0 || macroquad::input::get_last_key_pressed().is_some() {
                    self.state = GameState::MainMenu;
                }
            }
        }
    }

    fn finish_run(&mut self) {
        if let Some(pending) = self.high_score_candidate() {
            self.pending_high_score = Some(pending);
            self.high_score_name.clear();
            self.state = GameState::HighScoreEntry;
        } else {
            self.state = GameState::FinalScore;
            self.state_timer = 5.0;
        }
    }

    fn advance_frame_counter(&mut self) {
        self.frame_counter = self.frame_counter.wrapping_add(1);
    }

    fn high_score_candidate(&self) -> Option<PendingHighScore> {
        let score = self.players.iter().map(|p| p.score).max().unwrap_or(0);
        high_score_rank(&self.records, score)?;
        Some(PendingHighScore {
            score,
            level: (self.current_level + 1).min(8) as u8,
        })
    }

    fn update_high_score_entry(&mut self) {
        use macroquad::input::{get_char_pressed, is_key_pressed, KeyCode};
        while let Some(ch) = get_char_pressed() {
            if self.high_score_name.len() >= 8 {
                continue;
            }
            if let Some(c) = normalize_high_score_name_char(ch) {
                self.high_score_name.push(c);
            }
        }
        if is_key_pressed(KeyCode::Backspace) {
            self.high_score_name.pop();
        }
        if is_key_pressed(KeyCode::Enter) {
            self.sound.play(SoundEffect::HighScoreConfirm);
            self.submit_pending_high_score();
        }
    }

    fn submit_pending_high_score(&mut self) {
        let Some(pending) = self.pending_high_score.take() else {
            self.state = GameState::FinalScore;
            self.state_timer = 5.0;
            return;
        };
        let name = self.high_score_name.trim_end().to_string();
        insert_high_score(
            &mut self.records,
            HighScore {
                score: pending.score,
                level: pending.level,
                name,
            },
        );
        if !self.record_path.is_empty() {
            if let Err(err) = save_records(&self.record_path, &self.records) {
                eprintln!(
                    "Failed to save high scores to {}: {}",
                    self.record_path, err
                );
            }
        }
        self.state = GameState::FinalScore;
        self.state_timer = 5.0;
    }

    fn update_playing(&mut self, dt: f32) {
        let li = self.current_level;
        if li >= self.levels.len() {
            return;
        }
        let i1 = PlayerInput::read_player1();
        let i2 = if self.two_player {
            PlayerInput::read_player2()
        } else {
            PlayerInput::default()
        };
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Escape) {
            std::process::exit(0);
        }
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::S) {
            self.show_background = !self.show_background;
        }
        // §11.4: E narrows the visible width, R widens it (single-player only).
        if !self.two_player {
            if macroquad::input::is_key_pressed(macroquad::input::KeyCode::E) {
                self.narrow_screen_width_once();
            }
            if macroquad::input::is_key_pressed(macroquad::input::KeyCode::R) {
                self.widen_screen_width_once();
            }
        }

        let inputs = [i1, i2];
        let remaining_bonuses = self.remaining_required_bonus_source_count(li);
        let can_continue_level =
            Self::can_continue_level(&self.levels[li], &self.players, remaining_bonuses);
        let mut nb = Vec::new();
        for (i, p) in self.players.iter_mut().enumerate() {
            if !p.alive {
                if p.respawn_timer <= 0.0 && p.lives > 0 && inputs[i].fire && can_continue_level {
                    Self::continue_player_at_death_position(p);
                } else {
                    p.respawn_timer -= dt;
                }
                continue;
            }
            let starts_jump = inputs[i].jump && p.on_ground;
            let update_result = p.update(&inputs[i], &self.levels[li], dt);
            if update_result.placed_bomb {
                nb.push(Bomb::new_with_velocity(
                    p.x,
                    p.y + 8.0,
                    p.current_bomb,
                    i,
                    p.vx,
                    p.vy,
                ));
                self.sound.play(SoundEffect::PlaceBomb);
            }
            if update_result.teleported {
                self.sound.play(SoundEffect::Teleport);
            }
            if let Some(action_ref) = update_result.platform_action_ref {
                if self.levels[li].apply_platform_action(action_ref) {
                    self.sound.play(SoundEffect::PlatformAction);
                }
            }
            if starts_jump && p.vy < 0.0 {
                self.sound.play(SoundEffect::Jump);
            }
        }
        if Self::should_restart_dead_level(&self.players) {
            self.start_level();
            self.state = GameState::LevelIntro;
            self.state_timer = 3.0;
            return;
        }
        self.bombs.extend(nb);

        let mut expl = Vec::new();
        for b in &mut self.bombs {
            if b.exploding {
                b.explosion_timer -= dt;
            } else {
                b.update_arming_motion(&self.levels[li], dt);
                b.timer -= dt;
                if b.timer <= 0.0 {
                    b.exploding = true;
                    b.explosion_timer = EXPLOSION_DURATION;
                    expl.push((b.x, b.y, b.bomb_type, b.owner));
                    self.sound.play(SoundEffect::Explosion);
                }
            }
        }
        for (ex, ey, bt, owner) in &expl {
            self.process_explosion(*ex, *ey, *bt, *owner, li);
        }
        self.bombs
            .retain(|b| !b.exploding || b.explosion_timer > 0.0);
        self.update_collapsing(li, dt);
        self.update_original_spawn_controllers();

        let ps: Vec<Player> = self.players.clone();
        let mut monster_hit_counts = vec![0_u8; self.players.len()];
        let monster_motion_anchor_offsets = self
            .monsters
            .first()
            .map(|monster| monster.motion_anchor_offsets.clone())
            .unwrap_or_default();
        if !self.monster_motion_runtime_fields.is_empty()
            && self
                .monsters
                .iter()
                .any(Monster::uses_original_motion_update)
        {
            let random_words = original_motion_random_words(
                &self.monster_motion_runtime_fields,
                &mut self.original_rng,
            );
            let trig_words = original_motion_trig_words();
            preprocess_monster_motion_runtime_fields(
                &mut self.monster_motion_runtime_fields,
                &monster_motion_anchor_offsets,
                &random_words,
                &trig_words,
            );
        }
        let frame_counter = self.frame_counter as u32;
        for monster_idx in 0..self.monsters.len() {
            let mut state6_death_wakeup_key = None;
            let mut death_cleanup_effect = None;
            {
                let m = &mut self.monsters[monster_idx];
                match m.advance_original_death_countdown(frame_counter) {
                    OriginalDeathCountdownResult::Active => {
                        // Original death-transition objects only tick their countdown here.
                    }
                    OriginalDeathCountdownResult::Cleaned { effect_count } => {
                        death_cleanup_effect = Some((effect_count, m.x, m.y));
                    }
                    OriginalDeathCountdownResult::Inactive if m.uses_original_motion_update() => {
                        let (motion_sequence_fields, motion_sequence_reverse) =
                            resolve_motion_sequence_fields_from_runtime_records(
                                &self.monster_motion_runtime_fields,
                                m.motion_sequence_ids,
                            );
                        m.motion_sequence_reverse = motion_sequence_reverse;
                        m.update_with_preprocessed_original_motion_fields(
                            &self.levels[li],
                            &ps,
                            dt,
                            motion_sequence_fields,
                        );
                    }
                    OriginalDeathCountdownResult::Inactive
                        if m.uses_original_state5_countdown() =>
                    {
                        m.advance_original_state5_countdown(frame_counter);
                    }
                    OriginalDeathCountdownResult::Inactive => {
                        if m.apply_state6_random_impulse(
                            &self.levels[li],
                            &ps,
                            frame_counter,
                            &mut self.original_rng,
                        ) {
                            self.sound.play(SoundEffect::State6Impulse);
                        }
                        if m.update(&self.levels[li], &ps, dt) {
                            state6_death_wakeup_key = Some(m.state6_death_wakeup_key);
                        }
                    }
                }
                for (player_idx, p) in self.players.iter().enumerate() {
                    if m.collides_with_player(p) {
                        monster_hit_counts[player_idx] =
                            monster_hit_counts[player_idx].saturating_add(1);
                    }
                }
            }
            if let Some((effect_count, x, y)) = death_cleanup_effect {
                self.spawn_original_death_cleanup_effects(effect_count, x, y);
            }
            if let Some(wakeup_key) = state6_death_wakeup_key {
                self.apply_state6_dependent_death_transitions(monster_idx, wakeup_key);
                self.sound.play(SoundEffect::State6Death);
            }
        }
        for (player_idx, hits) in monster_hit_counts.into_iter().enumerate() {
            if hits != 0 {
                if let Some(p) = self.players.get_mut(player_idx) {
                    Self::play_damage_sound(
                        &self.sound,
                        Self::apply_monster_contact_damage(p, hits),
                    );
                }
            }
        }

        for pu in &mut self.powerups {
            pu.update(&self.levels[li], dt);
        }
        self.collect_tile_bonuses(li);
        let mut bonus_pus: Vec<Powerup> = Vec::new();
        for pu in &mut self.powerups {
            for p in &mut self.players {
                if pu.collides_with_player(p) {
                    Self::apply_powerup_to_player(
                        p,
                        pu.powerup_type,
                        pu.counts_for_level_goal,
                        &mut bonus_pus,
                        &mut self.original_rng,
                    );
                    pu.alive = false;
                    self.sound.play(SoundEffect::Pickup);
                }
            }
        }
        self.powerups.extend(bonus_pus);
        self.powerups.retain(|p| p.alive);

        for d in &mut self.debris {
            d.x += d.vx;
            d.y += d.vy;
            d.vy += 0.15;
            d.life -= dt;
        }
        self.debris.retain(|d| d.life > 0.0);

        if let Some((tx, ty)) = Self::scroll_target(&self.players, self.two_player) {
            self.scroll_x += (tx - self.scroll_x) * 0.1;
            self.scroll_y += (ty - self.scroll_y) * 0.1;
            let mx = (self.levels[li].width as f32 * TILE_SIZE) - SCREEN_W;
            let my = (self.levels[li].height as f32 * TILE_SIZE) - PLAY_HEIGHT;
            self.scroll_x = self.scroll_x.clamp(0.0, mx.max(0.0));
            self.scroll_y = self.scroll_y.clamp(0.0, my.max(0.0));
        }

        let cs = self.levels[li].count_destruction_tiles() as usize;
        if self.initial_destruction_tile_count > 0 {
            self.current_destruction_pct =
                (1.0 - cs as f32 / self.initial_destruction_tile_count as f32) * 100.0;
        }

        let lev = &self.levels[li];
        if Self::completion_requirements_available(lev, &self.players, self.current_destruction_pct)
        {
            self.award_level_completion_bonuses(li);
            self.state = GameState::LevelComplete;
            self.state_timer = LEVEL_COMPLETE_DELAY;
            self.sound.play(SoundEffect::LevelComplete);
        }

        if self.players.iter().all(|p| !p.alive && p.lives <= 0) {
            self.state = GameState::GameOver;
            self.state_timer = 5.0;
        }
    }

    fn apply_state6_dependent_death_transitions(&mut self, source_idx: usize, wakeup_key: u16) {
        for (dependent_idx, dependent) in self.monsters.iter_mut().enumerate() {
            if dependent_idx != source_idx && dependent.depends_on_state6_death_key(wakeup_key) {
                let countdown = 40 + self.original_rng.gen_mod(10) as u8;
                dependent.apply_original_death_transition(countdown);
            }
        }
    }

    fn spawn_original_death_cleanup_effects(&mut self, effect_count: u8, x: f32, y: f32) {
        for _ in 0..effect_count {
            let (vx, vy) = Self::original_debris_velocity(&mut self.original_rng);
            self.debris.push(Debris {
                x,
                y,
                vx,
                vy,
                color: 0x0d,
                life: 1.25,
            });
        }
    }

    fn update_original_spawn_controllers(&mut self) {
        for controller in &mut self.monster_spawn_controllers {
            if let Some(event) =
                controller.advance_original_spawn_controller(&mut self.original_rng)
            {
                self.monsters
                    .push(Self::monster_from_original_spawn_event(event));
            }
        }
    }

    fn monster_from_original_spawn_event(event: OriginalSpawnControllerEvent) -> Monster {
        let request = event.allocation_request;
        let object_id = request.template_selector;
        let mt = MonsterType::from_id(object_id);
        let x = request.x_px as f32;
        let y = request.y_px as f32 - 16.0;
        let mut monster = Monster::new(x, y, mt, mt.sprite_base(), mt.speed(), mt.damage(), 0);
        let allocation_call = request.original_object_allocation_call();

        monster.object_id = object_id;
        monster.original_state = request.allocation_param;
        monster.original_vitality_byte = event.runtime_fields.vitality;
        monster.health = f32::from(event.runtime_fields.vitality);
        monster.original_animation_seed = request.animation_seed;
        monster.original_animation_mode = request.animation_seed.mode;
        monster.original_motion = OriginalMotionState {
            x_px: request.x_px as i16,
            y_px: request.y_px as i16 - 16,
            x_fraction: 0,
            y_fraction: 0,
            x_velocity_word: allocation_call.param_5_x_velocity,
            y_velocity_word: allocation_call.param_6_y_velocity,
        };
        monster
    }

    fn process_explosion(&mut self, ex: f32, ey: f32, bt: BombType, owner: usize, li: usize) {
        let radius = bt.radius();
        let damage = bt.damage();
        let cx = ((ex + 8.0) / TILE_SIZE) as i32;
        let cy = ((ey + 8.0) / TILE_SIZE) as i32;

        // GAME_SPEC §4.4: read the attribute at the bomb tile. If it's a
        // building group (< 0x4000) flood-fill all connected tiles sharing
        // that attribute and clear them. Then fall back to a radius blast for
        // any remaining tiles inside the explosion sphere.
        if cx >= 0
            && cy >= 0
            && (cx as usize) < self.levels[li].width
            && (cy as usize) < self.levels[li].height
        {
            let attr = self.levels[li].attr_at(cx as usize, cy as usize);
            if attr != 0 && attr < 0x4000 {
                self.flood_fill_destroy(li, cx as usize, cy as usize, attr, owner);
            }
        }

        let tr = (radius / TILE_SIZE) as i32 + 1;
        for ty in (cy - tr)..=(cy + tr) {
            for tx in (cx - tr)..=(cx + tr) {
                if tx < 0 || ty < 0 {
                    continue;
                }
                let dx = (tx - cx) as f32 * TILE_SIZE;
                let dy = (ty - cy) as f32 * TILE_SIZE;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= radius {
                    let old = self.levels[li].tile_at(tx as usize, ty as usize);
                    if old != 0 {
                        self.count_required_tile_destroyed(li, owner, old);
                        self.levels[li].set_tile(tx as usize, ty as usize, 0);
                        for _ in 0..3 {
                            let (vx, vy) = Self::original_debris_velocity(&mut self.original_rng);
                            self.debris.push(Debris {
                                x: tx as f32 * TILE_SIZE + 4.0,
                                y: ty as f32 * TILE_SIZE + 4.0,
                                vx,
                                vy,
                                color: old,
                                life: 1.25,
                            });
                        }
                    }
                }
            }
        }
        let mut kills: Vec<(f32, f32)> = Vec::new();
        for m in &mut self.monsters {
            let dx = m.x - ex;
            let dy = m.y - ey;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= radius {
                let was = m.alive;
                m.take_damage(damage * (1.0 - dist / radius));
                if was && !m.alive {
                    kills.push((m.x, m.y));
                }
            }
        }
        for (mx, my) in kills {
            self.powerups.push(Powerup::new(
                mx,
                my,
                PowerupType::random_from_original_rng(&mut self.original_rng),
            ));
            if let Some(p) = self.players.get_mut(owner) {
                p.score = p.score.saturating_add(500);
            }
        }
        for p in &mut self.players {
            let dx = p.x - ex;
            let dy = p.y - ey;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= radius {
                Self::play_damage_sound(
                    &self.sound,
                    p.take_damage(damage * 0.5 * (1.0 - dist / radius)),
                );
            }
        }
    }

    /// 4-connected flood fill over a building group: every reachable tile
    /// whose attribute equals `attr` is replaced with crack stage 0x76 and
    /// queued for collapse animation. Mirrors the rectangle-expansion
    /// algorithm in FUN_1000_370e (with §11.2 crack states).
    fn flood_fill_destroy(&mut self, li: usize, sx: usize, sy: usize, attr: u16, _owner: usize) {
        let (w, h) = {
            let lev = &self.levels[li];
            (lev.width, lev.height)
        };
        if self.levels[li].attr_at(sx, sy) != attr {
            return;
        }
        let mut stack: Vec<(usize, usize)> = Vec::with_capacity(64);
        stack.push((sx, sy));
        let mut destroyed = 0u32;
        while let Some((x, y)) = stack.pop() {
            let idx = y * w + x;
            let lev = &mut self.levels[li];
            if lev.attrs[idx] != attr || lev.tiles[idx] == 0 {
                continue;
            }
            let old = lev.tiles[idx];
            // Clear from the level immediately so collisions and destruction%
            // see it as gone; the overlay carries the visual until it lands.
            lev.tiles[idx] = 0;
            self.count_required_tile_destroyed(li, _owner, old);
            self.collapsing.push(Collapsing {
                tx: x,
                ty: y,
                stage: 0x76,
                timer: COLLAPSE_FRAME_TIME,
                vy: 0.0,
                y_offset: 0.0,
            });
            destroyed += 1;
            for _ in 0..2 {
                let (vx, vy) = Self::original_debris_velocity(&mut self.original_rng);
                self.debris.push(Debris {
                    x: x as f32 * TILE_SIZE + 4.0,
                    y: y as f32 * TILE_SIZE + 4.0,
                    vx,
                    vy,
                    color: old,
                    life: 1.0,
                });
            }
            if x + 1 < w {
                stack.push((x + 1, y));
            }
            if x > 0 {
                stack.push((x - 1, y));
            }
            if y + 1 < h {
                stack.push((x, y + 1));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
        }
        if destroyed != 0 {
            self.sound.play(SoundEffect::Collapse);
        }
    }

    fn original_debris_velocity(original_rng: &mut OriginalRng) -> (f32, f32) {
        let vy_word = original_rng.gen_centered(600);
        let vx_word = original_rng.gen_centered(600);
        (vx_word as f32 / 256.0, vy_word as f32 / 256.0)
    }

    fn original_jolly_cloud_x_offset(original_rng: &mut OriginalRng) -> f32 {
        original_rng.gen_centered(80) as f32
    }

    fn count_required_tile_destroyed(&mut self, li: usize, owner: usize, old_tile: u8) {
        if old_tile == self.levels[li].destruction_tile {
            if let Some(player) = self.players.get_mut(owner) {
                player.bonuses_collected = player.bonuses_collected.saturating_add(1);
            }
        }
    }

    fn visible_tile_columns(&self) -> u16 {
        ((self.screen_width_factor * MAX_VISIBLE_TILE_COLUMNS as f32).round() as u16)
            .clamp(MIN_VISIBLE_TILE_COLUMNS, MAX_VISIBLE_TILE_COLUMNS)
    }

    fn set_visible_tile_columns(&mut self, columns: u16) {
        let columns = columns.clamp(MIN_VISIBLE_TILE_COLUMNS, MAX_VISIBLE_TILE_COLUMNS);
        self.screen_width_factor = columns as f32 / MAX_VISIBLE_TILE_COLUMNS as f32;
    }

    fn narrow_screen_width_once(&mut self) {
        let columns = self.visible_tile_columns();
        self.set_visible_tile_columns(columns.saturating_sub(1));
    }

    fn widen_screen_width_once(&mut self) {
        let columns = self.visible_tile_columns();
        self.set_visible_tile_columns(columns.saturating_add(1));
    }

    /// Advance collapse animation and falling physics. Each piece accumulates
    /// gravity and sub-pixel vertical offset; the crack stage cycles 0x76→0x79
    /// over time. The piece is removed when it lands on a solid tile, drops
    /// off the bottom of the level, or finishes its crack cycle in mid-air.
    fn update_collapsing(&mut self, li: usize, dt: f32) {
        if self.collapsing.is_empty() {
            return;
        }
        let lev_h = self.levels[li].height;
        let mut still_alive = Vec::with_capacity(self.collapsing.len());
        for mut c in self.collapsing.drain(..) {
            // Stage timer advances the visible crack value.
            c.timer -= dt;
            if c.timer <= 0.0 && c.stage < 0x79 {
                c.stage += 1;
                c.timer = COLLAPSE_FRAME_TIME;
            }
            // Apply gravity; clamp to terminal velocity.
            c.vy = (c.vy + RUBBLE_GRAVITY).min(RUBBLE_MAX_FALL);
            c.y_offset += c.vy;
            let cur_row = c.ty + (c.y_offset / TILE_SIZE) as usize;
            let next_row = cur_row + 1;
            let landed = next_row >= lev_h || self.levels[li].is_solid(c.tx, next_row);
            // Damage entities standing in the falling cell. Per-frame at 70 Hz,
            // so the constants are tuned for ~7 dps to players, ~28 dps to
            // monsters when overlap persists.
            let py = c.ty as f32 * TILE_SIZE + c.y_offset;
            let px = c.tx as f32 * TILE_SIZE;
            for p in &mut self.players {
                if p.alive
                    && p.x + 12.0 > px
                    && p.x < px + TILE_SIZE
                    && p.y + 16.0 > py
                    && p.y < py + TILE_SIZE
                {
                    Self::play_damage_sound(&self.sound, p.take_damage(0.1));
                }
            }
            for m in &mut self.monsters {
                if m.alive
                    && m.x + 14.0 > px
                    && m.x < px + TILE_SIZE
                    && m.y + 14.0 > py
                    && m.y < py + TILE_SIZE
                {
                    m.take_damage(0.4);
                }
            }
            if landed {
                continue;
            }
            if c.stage >= 0x79 && c.timer <= 0.0 {
                continue;
            }
            still_alive.push(c);
        }
        self.collapsing = still_alive;
    }

    fn continue_player_at_death_position(player: &mut Player) {
        let (x, y) = (player.x, player.y);
        player.apply_continue_bomb_minimums();
        player.respawn(x, y);
    }

    fn collect_tile_bonuses(&mut self, li: usize) {
        let mut pickups = Vec::new();
        for (player_idx, p) in self.players.iter().enumerate() {
            if !p.alive {
                continue;
            }
            let left = (p.x / TILE_SIZE).floor() as i32;
            let right = ((p.x + PLAYER_W - 1.0) / TILE_SIZE).floor() as i32;
            let top = (p.y / TILE_SIZE).floor() as i32;
            let bottom = ((p.y + PLAYER_H - 1.0) / TILE_SIZE).floor() as i32;
            for ty in top..=bottom {
                for tx in left..=right {
                    if tx < 0 || ty < 0 {
                        continue;
                    }
                    let tile = self.levels[li].tile_at(tx as usize, ty as usize);
                    if (TILE_BONUS_MIN..=TILE_BONUS_MAX).contains(&tile) {
                        self.levels[li].set_tile(tx as usize, ty as usize, 0);
                        pickups.push((
                            player_idx,
                            PowerupType::from_bonus_id(tile - TILE_BONUS_MIN),
                        ));
                    }
                }
            }
        }

        let mut bonus_pus = Vec::new();
        for (player_idx, powerup_type) in pickups {
            if let Some(p) = self.players.get_mut(player_idx) {
                Self::apply_powerup_to_player(
                    p,
                    powerup_type,
                    true,
                    &mut bonus_pus,
                    &mut self.original_rng,
                );
                self.sound.play(SoundEffect::Pickup);
            }
        }
        self.powerups.extend(bonus_pus);
    }

    fn apply_powerup_to_player(
        p: &mut Player,
        powerup_type: PowerupType,
        counts_for_level_goal: bool,
        bonus_pus: &mut Vec<Powerup>,
        original_rng: &mut OriginalRng,
    ) {
        p.score = p.score.saturating_add(powerup_type.points());
        if counts_for_level_goal {
            p.bonuses_collected += 1;
        }
        if let Some(effect_id) = powerup_type.original_effect_id() {
            let effect = original_pickup_effect(effect_id).expect("mapped effect id is covered");
            if let Some(vitality) = effect.vitality {
                match vitality {
                    OriginalVitalityEffect::Set(value) => p.energy = f32::from(value),
                    OriginalVitalityEffect::AddCapped { amount, cap } => {
                        p.energy = (p.energy + f32::from(amount)).min(f32::from(cap));
                    }
                }
            }
            if let Some(frames) = effect.invincibility_frames {
                p.invincible_frames = frames;
            }
            if let Some(bomb_box) = effect.bomb_box {
                let medium = original_rng.gen_mod(bomb_box.medium_roll_mod) as i32
                    + i32::from(bomb_box.medium_add_base);
                let large = original_rng.gen_mod(bomb_box.large_roll_mod) as i32
                    + i32::from(bomb_box.large_add_base);
                if let Some(super_roll_mod) = bomb_box.super_roll_mod {
                    let super_bombs = original_rng.gen_mod(super_roll_mod) as i32
                        + i32::from(bomb_box.super_add_base);
                    p.apply_green_bomb_box(medium, large, super_bombs);
                } else {
                    p.apply_yellow_bomb_box(medium, large);
                }
            }
            return;
        }
        if powerup_type == PowerupType::JollyCloud {
            for j in 0..3 {
                let x_offset = Self::original_jolly_cloud_x_offset(original_rng);
                bonus_pus.push(Powerup::new(
                    p.x + x_offset,
                    p.y - 30.0 - j as f32 * 20.0,
                    PowerupType::Present,
                ));
            }
        }
    }

    fn play_damage_sound(sound: &SoundManager, outcome: DamageOutcome) {
        match outcome {
            DamageOutcome::None => {}
            DamageOutcome::Hurt => sound.play(SoundEffect::Hurt),
            DamageOutcome::Die => sound.play(SoundEffect::Die),
        }
    }

    fn apply_monster_contact_damage(player: &mut Player, hits: u8) -> DamageOutcome {
        player.take_damage(MONSTER_CONTACT_DAMAGE_PER_FRAME * hits as f32)
    }

    fn remaining_collectible_bonus_count(&self) -> u32 {
        self.powerups
            .iter()
            .filter(|p| p.alive && p.counts_for_level_goal)
            .count() as u32
    }

    fn remaining_required_bonus_source_count(&self, li: usize) -> u32 {
        self.remaining_collectible_bonus_count()
            .saturating_add(self.levels[li].count_destruction_tiles())
    }

    fn collected_required_bonus_count(players: &[Player]) -> u32 {
        players
            .iter()
            .map(|p| p.bonuses_collected)
            .fold(0_u32, u32::saturating_add)
    }

    fn can_continue_level(level: &Level, players: &[Player], remaining_bonuses: u32) -> bool {
        Self::collected_required_bonus_count(players) + remaining_bonuses
            >= level.bonus_target as u32
    }

    fn destroyed_destruction_tile_count(&self, li: usize) -> u32 {
        self.initial_destruction_tile_count
            .saturating_sub(self.levels[li].count_destruction_tiles() as usize) as u32
    }

    fn level_completion_bomb_bonus(player: &Player) -> u32 {
        let medium = player.bombs[1].max(0) as u32;
        let large = player.bombs[2].max(0) as u32;
        let super_bombs = player.bombs[3].max(0) as u32;
        medium
            .saturating_mul(100)
            .saturating_add(large.saturating_mul(500))
            .saturating_add(super_bombs.saturating_mul(2000))
    }

    fn award_level_completion_bonuses(&mut self, li: usize) {
        if self.level_completion_bonuses_awarded {
            return;
        }
        self.level_completion_bonuses_awarded = true;
        let destruction_bonus = self.destroyed_destruction_tile_count(li).saturating_mul(10);
        self.last_level_destruction_bonus = destruction_bonus;
        self.last_level_bomb_bonuses = self
            .players
            .iter()
            .map(Self::level_completion_bomb_bonus)
            .collect();
        for (player, bomb_bonus) in self
            .players
            .iter_mut()
            .zip(self.last_level_bomb_bonuses.iter().copied())
        {
            player.score = player
                .score
                .saturating_add(destruction_bonus)
                .saturating_add(bomb_bonus);
        }
    }

    fn completion_requirements_available(
        level: &Level,
        players: &[Player],
        destruction_pct: f32,
    ) -> bool {
        Self::collected_required_bonus_count(players) >= level.bonus_target as u32
            && destruction_pct >= level.destruction_pct as f32
    }

    fn scroll_target(players: &[Player], two_player: bool) -> Option<(f32, f32)> {
        if two_player {
            let mut count = 0.0;
            let mut sx = 0.0;
            let mut sy = 0.0;
            for p in players.iter().take(2).filter(|p| p.alive) {
                count += 1.0;
                sx += p.x;
                sy += p.y;
            }
            if count > 0.0 {
                return Some((
                    sx / count - SCREEN_W / 2.0 + 6.0,
                    sy / count - PLAY_HEIGHT / 2.0 + 8.0,
                ));
            }
            return None;
        }

        players
            .first()
            .filter(|p| p.alive)
            .map(|p| (p.x - SCREEN_W / 2.0 + 6.0, p.y - PLAY_HEIGHT / 2.0 + 8.0))
    }

    fn should_restart_dead_level(players: &[Player]) -> bool {
        !players.is_empty()
            && players.iter().all(|p| !p.alive)
            && players.iter().any(|p| p.lives > 0)
            && players
                .iter()
                .all(|p| p.lives <= 0 || p.respawn_timer <= -DEATH_RESTART_WAIT)
    }
}

pub fn high_score_rank(records: &[HighScore], score: u32) -> Option<usize> {
    if records.len() < 7 {
        return Some(
            records
                .iter()
                .position(|r| score > r.score)
                .unwrap_or(records.len()),
        );
    }
    records.iter().position(|r| score > r.score)
}

pub fn insert_high_score(records: &mut Vec<HighScore>, record: HighScore) {
    let rank = high_score_rank(records, record.score);
    if let Some(idx) = rank {
        records.insert(idx, record);
        records.truncate(7);
    }
}

fn normalize_high_score_name_char(ch: char) -> Option<char> {
    let upper = ch.to_ascii_uppercase();
    if upper.is_ascii_uppercase() || upper == ' ' {
        Some(upper.to_ascii_lowercase())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_level() -> Level {
        let tiles = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ];
        Level {
            width: 8,
            height: 4,
            destruction_tile: 1,
            bonus_target: 1,
            destruction_pct: 50,
            attrs: vec![0; 32],
            orig_tiles: tiles.clone(),
            initial_destruction_tile_count: tiles.iter().filter(|&&t| t == 1).count() as u32,
            tiles,
            scroll_x: 0,
            scroll_y: 0,
            monsters: Vec::new(),
            bonuses: Vec::new(),
            platforms: Vec::new(),
        }
    }

    #[test]
    fn state6_death_transitions_matching_dependents_with_original_rng_countdown() {
        let seed = 0x1234_5678;
        let mut expected = OriginalRng::new(seed);
        let expected_countdown = 40 + expected.gen_mod(10) as u8;
        let mut game = Game::new(
            vec![test_level()],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.original_rng = OriginalRng::new(seed);

        let mut source = Monster::new(0.0, 0.0, MonsterType::Walker, 0, 0.0, 0.0, 0);
        source.object_id = 0x0e;
        source.state6_death_wakeup_key = 4;
        let mut matching = Monster::new(8.0, 0.0, MonsterType::Walker, 0, 0.0, 0.0, 0);
        matching.object_id = 0x1f;
        matching.dependent_death_key = 4;
        let mut non_matching = Monster::new(16.0, 0.0, MonsterType::Walker, 0, 0.0, 0.0, 0);
        non_matching.object_id = 0x1f;
        non_matching.dependent_death_key = 6;
        game.monsters = vec![source, matching, non_matching];

        game.apply_state6_dependent_death_transitions(0, 4);

        assert_eq!(game.original_rng.seed(), expected.seed());
        assert_eq!(game.monsters[1].object_id, 0x0e);
        assert_eq!(game.monsters[1].original_state, 2);
        assert_eq!(game.monsters[1].original_death_timer, expected_countdown);
        assert_eq!(game.monsters[1].original_animation_mode, 0);
        assert!(!game.monsters[1].alive);
        assert_eq!(game.monsters[2].object_id, 0x1f);
        assert_eq!(game.monsters[2].original_death_timer, 0);
        assert!(game.monsters[2].alive);
    }

    #[test]
    fn original_death_cleanup_effects_consume_original_debris_rng() {
        let seed = 0x1357_2468;
        let mut expected_rng = OriginalRng::new(seed);
        let expected_velocities = [
            Game::original_debris_velocity(&mut expected_rng),
            Game::original_debris_velocity(&mut expected_rng),
            Game::original_debris_velocity(&mut expected_rng),
        ];
        let mut game = Game::new(
            vec![test_level()],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.original_rng = OriginalRng::new(seed);

        game.spawn_original_death_cleanup_effects(3, 24.0, 32.0);

        assert_eq!(game.original_rng.seed(), expected_rng.seed());
        assert_eq!(game.debris.len(), 3);
        for (debris, (vx, vy)) in game.debris.iter().zip(expected_velocities) {
            assert_eq!((debris.x, debris.y), (24.0, 32.0));
            assert_eq!((debris.vx, debris.vy), (vx, vy));
            assert_eq!(debris.color, 0x0d);
            assert_eq!(debris.life, 1.25);
        }
    }

    #[test]
    fn start_level_restores_level_scoped_state() {
        let mut game = Game::new(
            vec![test_level()],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.players.push(Player::new(0.0, 0.0, 0));
        game.players[0].bonuses_collected = 7;
        game.levels[0].tiles[16] = 0;
        game.current_destruction_pct = 42.0;

        game.start_level();

        assert_eq!(game.levels[0].tiles, game.levels[0].orig_tiles);
        assert_eq!(game.players[0].bonuses_collected, 0);
        assert_eq!(game.current_destruction_pct, 0.0);
    }

    #[test]
    fn update_playing_advances_original_spawn_controllers_into_monsters() {
        let mut level = test_level();
        level.bonus_target = 99;
        level.destruction_pct = 100;
        let mut raw = [0; 30];
        raw[0x00..0x02].copy_from_slice(&0x0150_u16.to_le_bytes());
        raw[0x02..0x04].copy_from_slice(&0x00a8_u16.to_le_bytes());
        raw[0x08] = 1;
        raw[0x09] = 2;
        raw[0x0a] = 3;
        raw[0x0b] = 4;
        raw[0x0c..0x0e].copy_from_slice(&10_u16.to_le_bytes());
        raw[0x0e..0x10].copy_from_slice(&5_u16.to_le_bytes());
        raw[0x10..0x12].copy_from_slice(&20_u16.to_le_bytes());
        raw[0x12..0x14].copy_from_slice(&7_u16.to_le_bytes());
        raw[0x14..0x16].copy_from_slice(&30_u16.to_le_bytes());
        raw[0x16..0x18].copy_from_slice(&11_u16.to_le_bytes());
        raw[0x18] = 4;
        raw[0x19] = 3;
        raw[0x1a] = 3;
        raw[0x1b] = 1;
        raw[0x1c] = 0x3c;
        raw[0x1d] = 2;
        level.monsters.push(MonsterSpawn { raw });

        let seed = 0x0bad_cafe;
        let mut expected_rng = OriginalRng::new(seed);
        expected_rng.gen_mod(5);
        expected_rng.gen_mod(7);
        expected_rng.gen_mod(11);
        let expected_vitality = 4 + expected_rng.gen_mod(3) as u8;

        let mut game = Game::new(
            vec![level],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.players.push(Player::new(0.0, 0.0, 0));
        game.original_rng = OriginalRng::new(seed);
        game.start_level();

        assert!(game.monsters.is_empty());
        assert_eq!(game.monster_spawn_controllers.len(), 1);

        game.update_original_spawn_controllers();

        assert_eq!(game.original_rng.seed(), expected_rng.seed());
        assert_eq!(game.monsters.len(), 1);
        assert_eq!(game.monsters[0].object_id, 4);
        assert_eq!(game.monsters[0].original_state, 3);
        assert_eq!(game.monsters[0].original_vitality_byte, expected_vitality);
        assert_eq!(game.monsters[0].health, f32::from(expected_vitality));
        assert_eq!(
            game.monsters[0].original_animation_seed,
            MonsterAnimationSeed::from_original_setup(1, 2, 0x38, 0x36)
        );
        assert_eq!(game.monster_spawn_controllers[0].original_spawn_count(), 1);
        assert_eq!(game.monster_spawn_controllers[0].original_spawn_budget(), 2);
        assert_eq!(
            game.monster_spawn_controllers[0].original_spawn_timer(),
            0x3c
        );
    }

    #[test]
    fn start_level_uses_original_seven_byte_player_start_records() {
        let mut level = test_level();
        level.bonuses.push(BonusSpawn {
            raw: [0, 0, 0x18, 0x01, 0xa8, 0x00, 2],
        });
        level.bonuses.push(BonusSpawn {
            raw: [0, 0, 0x68, 0x00, 0xa8, 0x00, 1],
        });
        let mut game = Game::new(
            vec![level],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.sound.enabled = false;
        game.players.push(Player::new(0.0, 0.0, 0));
        game.players.push(Player::new(0.0, 0.0, 1));

        game.start_level();

        assert_eq!(
            (game.players[0].x, game.players[0].y),
            (0x0068 as f32, 0x00a8 as f32)
        );
        assert_eq!(
            (game.players[1].x, game.players[1].y),
            (0x0118 as f32, 0x00a8 as f32)
        );
    }

    #[test]
    fn start_level_places_level_bonus_from_decoded_pixel_fields() {
        let mut level = test_level();
        level.bonuses.push(BonusSpawn {
            raw: [0, 0, 0x18, 0x01, 0xa8, 0x00, 0],
        });
        let mut game = Game::new(
            vec![level],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.sound.enabled = false;
        game.players.push(Player::new(0.0, 0.0, 0));

        game.start_level();

        assert_eq!(game.powerups.len(), 1);
        assert_eq!(game.powerups[0].x, 0x0118 as f32);
        assert_eq!(game.powerups[0].y, 0x00a8 as f32 - 8.0);
        assert!(game.powerups[0].counts_for_level_goal);
        assert!(matches!(
            game.powerups[0].powerup_type,
            PowerupType::Present
        ));
    }

    #[test]
    fn start_level_does_not_spawn_player_start_records_as_powerups() {
        let mut level = test_level();
        level.bonuses.push(BonusSpawn {
            raw: [0, 0, 0x18, 0x01, 0xa8, 0x00, 2],
        });
        level.bonuses.push(BonusSpawn {
            raw: [0, 0, 0x68, 0x00, 0xa8, 0x00, 1],
        });
        let mut game = Game::new(
            vec![level],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.players.push(Player::new(0.0, 0.0, 0));
        game.players.push(Player::new(0.0, 0.0, 1));

        game.start_level();

        assert!(game.powerups.is_empty());
    }

    #[test]
    fn continue_after_death_respawns_at_death_position() {
        let mut player = Player::new(120.0, 64.0, 0);
        player.bombs = [0, 0, 0, 0];
        player.die();
        player.x = 136.0;
        player.y = 72.0;

        Game::continue_player_at_death_position(&mut player);

        assert!(player.alive);
        assert_eq!((player.x, player.y), (136.0, 72.0));
        assert_eq!(player.energy, player.max_energy);
        assert_eq!(player.bombs, [100, 10, 2, 0]);
    }

    #[test]
    fn continue_requires_enough_available_bonuses_to_finish_level() {
        let mut level = test_level();
        level.bonus_target = 2;
        let mut player = Player::new(0.0, 0.0, 0);

        assert!(!Game::can_continue_level(
            &level,
            std::slice::from_ref(&player),
            0
        ));

        assert!(Game::can_continue_level(
            &level,
            std::slice::from_ref(&player),
            2
        ));

        player.bonuses_collected = 1;
        assert!(!Game::can_continue_level(&level, &[player], 0));
    }

    #[test]
    fn completion_bonus_requirement_uses_collected_bonuses_only() {
        let mut level = test_level();
        level.bonus_target = 2;
        level.destruction_pct = 50;
        let mut player = Player::new(0.0, 0.0, 0);
        player.bonuses_collected = 1;

        assert!(!Game::completion_requirements_available(
            &level,
            &[player],
            50.0
        ));

        let mut player = Player::new(0.0, 0.0, 0);
        player.bonuses_collected = 2;
        assert!(Game::completion_requirements_available(
            &level,
            &[player],
            50.0
        ));
    }

    #[test]
    fn flood_fill_destruction_does_not_score_until_level_completion() {
        let mut level = test_level();
        level.attrs[16] = 7;
        let mut game = Game::new(
            vec![level],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.sound.enabled = false;
        game.players.push(Player::new(0.0, 0.0, 0));
        game.initial_destruction_tile_count =
            game.levels[0].initial_destruction_tile_count as usize;
        game.players[0].score = 100;

        game.flood_fill_destroy(0, 0, 2, 7, 0);

        assert_eq!(game.players[0].score, 100);
        assert_eq!(game.destroyed_destruction_tile_count(0), 1);
        assert_eq!(game.players[0].bonuses_collected, 1);
    }

    #[test]
    fn level_completion_awards_original_destruction_and_bomb_bonuses() {
        let mut game = Game::new(
            vec![test_level()],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.players.push(Player::new(0.0, 0.0, 0));
        game.initial_destruction_tile_count =
            game.levels[0].initial_destruction_tile_count as usize;
        game.levels[0].tiles[16] = 0;
        game.levels[0].tiles[17] = 0;
        game.players[0].bombs = [99, 2, 3, 1];

        game.award_level_completion_bonuses(0);

        assert_eq!(game.last_level_destruction_bonus, 20);
        assert_eq!(game.last_level_bomb_bonuses, vec![3700]);
        assert_eq!(game.players[0].score, 3720);

        game.award_level_completion_bonuses(0);
        assert_eq!(game.players[0].score, 3720);
    }

    #[test]
    fn completion_requirements_do_not_depend_on_high_tile_overlap() {
        let mut level = test_level();
        level.tiles[1] = 0x6d;
        let mut player = Player::new(0.0, 0.0, 0);
        player.bonuses_collected = 1;

        assert!(Game::completion_requirements_available(
            &level,
            &[player.clone()],
            50.0
        ));
    }

    #[test]
    fn two_player_scroll_target_tracks_alive_player_center() {
        let p1 = Player::new(100.0, 40.0, 0);
        let p2 = Player::new(220.0, 80.0, 1);

        assert_eq!(
            Game::scroll_target(&[p1.clone(), p2.clone()], true),
            Some((6.0, -12.0))
        );

        assert_eq!(Game::scroll_target(&[p1, p2], false), Some((-54.0, -32.0)));
    }

    #[test]
    fn two_player_scroll_target_ignores_dead_players() {
        let p1 = Player::new(100.0, 40.0, 0);
        let mut p2 = Player::new(220.0, 80.0, 1);
        p2.alive = false;

        assert_eq!(Game::scroll_target(&[p1, p2], true), Some((-54.0, -32.0)));
    }

    #[test]
    fn screen_width_adjustment_matches_original_tile_column_bounds() {
        let mut game = Game::new(
            vec![test_level()],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );

        assert_eq!(game.visible_tile_columns(), MAX_VISIBLE_TILE_COLUMNS);

        game.narrow_screen_width_once();
        assert_eq!(game.visible_tile_columns(), MAX_VISIBLE_TILE_COLUMNS - 1);

        for _ in 0..100 {
            game.narrow_screen_width_once();
        }
        assert_eq!(game.visible_tile_columns(), MIN_VISIBLE_TILE_COLUMNS);
        assert_eq!(
            game.screen_width_factor,
            MIN_VISIBLE_TILE_COLUMNS as f32 / MAX_VISIBLE_TILE_COLUMNS as f32
        );

        game.widen_screen_width_once();
        assert_eq!(game.visible_tile_columns(), MIN_VISIBLE_TILE_COLUMNS + 1);

        for _ in 0..100 {
            game.widen_screen_width_once();
        }
        assert_eq!(game.visible_tile_columns(), MAX_VISIBLE_TILE_COLUMNS);
        assert_eq!(game.screen_width_factor, 1.0);
    }

    #[test]
    fn frame_counter_wraps_like_original_word() {
        let mut game = Game::new(
            vec![test_level()],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.frame_counter = u16::MAX;

        game.advance_frame_counter();

        assert_eq!(game.frame_counter, 0);
    }

    #[test]
    fn remaining_bonus_count_uses_live_powerups_not_static_level_table() {
        let mut game = Game::new(
            vec![test_level()],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.powerups
            .push(Powerup::new(0.0, 0.0, PowerupType::Present).level_goal());
        game.powerups
            .push(Powerup::new(0.0, 0.0, PowerupType::Present));
        let mut collected = Powerup::new(0.0, 0.0, PowerupType::Present);
        collected.counts_for_level_goal = true;
        collected.alive = false;
        game.powerups.push(collected);

        assert_eq!(game.remaining_collectible_bonus_count(), 1);
    }

    #[test]
    fn only_level_goal_pickups_count_toward_required_bonuses() {
        let mut player = Player::new(0.0, 0.0, 0);
        let dropped = Powerup::new(0.0, 0.0, PowerupType::Present);
        let level_goal = Powerup::new(0.0, 0.0, PowerupType::Present).level_goal();

        if dropped.counts_for_level_goal {
            player.bonuses_collected += 1;
        }
        assert_eq!(player.bonuses_collected, 0);

        if level_goal.counts_for_level_goal {
            player.bonuses_collected += 1;
        }
        assert_eq!(player.bonuses_collected, 1);
    }

    #[test]
    fn tile_bonus_pickup_clears_tile_and_counts_for_level_goal() {
        let mut level = test_level();
        level.tiles[9] = TILE_BONUS_MIN + 2;
        level.orig_tiles = level.tiles.clone();
        let mut game = Game::new(
            vec![level],
            Vec::new(),
            Vec::new(),
            String::new(),
            SoundManager::new(),
        );
        game.sound.enabled = false;
        game.players.push(Player::new(8.0, 8.0, 0));
        game.players[0].energy = 10.0;

        game.collect_tile_bonuses(0);

        assert_eq!(game.levels[0].tile_at(1, 1), 0);
        assert_eq!(game.players[0].bonuses_collected, 1);
        assert_eq!(game.players[0].energy, game.players[0].max_energy);
        assert_eq!(game.players[0].score, PowerupType::FirstAid.points());
    }

    #[test]
    fn powerup_score_awards_saturate_at_u32_max() {
        let mut player = Player::new(0.0, 0.0, 0);
        player.score = u32::MAX - 1;
        let mut spawned = Vec::new();
        let mut original_rng = OriginalRng::new(0);

        Game::apply_powerup_to_player(
            &mut player,
            PowerupType::BigDiamond,
            false,
            &mut spawned,
            &mut original_rng,
        );

        assert_eq!(player.score, u32::MAX);
    }

    #[test]
    fn bomb_box_powerups_use_original_rng_ranges() {
        let seed = 0x1234_5678;
        let mut expected = OriginalRng::new(seed);
        let yellow_medium = expected.gen_mod(10) as i32 + 1;
        let yellow_large = expected.gen_mod(4) as i32 + 1;

        let mut rng = OriginalRng::new(seed);
        let mut player = Player::new(0.0, 0.0, 0);
        let mut spawned = Vec::new();
        Game::apply_powerup_to_player(
            &mut player,
            PowerupType::YellowBombBox,
            false,
            &mut spawned,
            &mut rng,
        );

        assert_eq!(player.bombs, [200, yellow_medium, yellow_large, 0]);
        assert_eq!(rng.seed(), expected.seed());

        let seed = 0x8765_4321;
        let mut expected = OriginalRng::new(seed);
        let green_medium = expected.gen_mod(0x0d) as i32 + 1;
        let green_large = expected.gen_mod(5) as i32 + 2;
        let green_super = expected.gen_mod(2) as i32 + 1;

        let mut rng = OriginalRng::new(seed);
        let mut player = Player::new(0.0, 0.0, 0);
        Game::apply_powerup_to_player(
            &mut player,
            PowerupType::GreenBombBox,
            false,
            &mut spawned,
            &mut rng,
        );

        assert_eq!(player.bombs, [200, green_medium, green_large, green_super]);
        assert!(player.has_super_bombs);
        assert_eq!(rng.seed(), expected.seed());
    }

    #[test]
    fn debris_velocity_uses_original_rng_range() {
        let seed = 0x2468_ace0;
        let mut expected = OriginalRng::new(seed);
        let expected_vy = (expected.gen_mod(600) as i32 - 300) as f32 / 256.0;
        let expected_vx = (expected.gen_mod(600) as i32 - 300) as f32 / 256.0;

        let mut rng = OriginalRng::new(seed);
        let (vx, vy) = Game::original_debris_velocity(&mut rng);

        assert_eq!(vx, expected_vx);
        assert_eq!(vy, expected_vy);
        assert_eq!(rng.seed(), expected.seed());
        assert!((-300.0 / 256.0..=299.0 / 256.0).contains(&vx));
        assert!((-300.0 / 256.0..=299.0 / 256.0).contains(&vy));
    }

    #[test]
    fn monster_contact_damage_matches_original_hit_counter_ticks() {
        let mut player = Player::new(0.0, 0.0, 0);
        player.energy = 10.0;

        assert_eq!(
            Game::apply_monster_contact_damage(&mut player, 2),
            DamageOutcome::Hurt
        );

        assert_eq!(player.energy, 8.0);
    }

    #[test]
    fn jolly_cloud_offsets_use_original_rng_range() {
        let seed = 0x0bad_f00d;
        let mut expected = OriginalRng::new(seed);
        let expected_offset = (expected.gen_mod(80) as i32 - 40) as f32;

        let mut rng = OriginalRng::new(seed);
        let offset = Game::original_jolly_cloud_x_offset(&mut rng);

        assert_eq!(offset, expected_offset);
        assert_eq!(rng.seed(), expected.seed());
        assert!((-40.0..=39.0).contains(&offset));
    }

    #[test]
    fn original_motion_random_words_consume_rng_for_bounded_records_only() {
        let bounded = MonsterMotionRuntimeFields {
            anchor_index: 0,
            secondary_anchor_index: 0,
            phase_step: 0,
            limit_or_sentinel: 10,
            angle_phase: 0,
            base_x_word: 0,
            base_y_word: 0,
            x_word: 0,
            y_word: 0,
            random_y_base: 0,
        };
        let absolute = MonsterMotionRuntimeFields {
            limit_or_sentinel: -1,
            ..bounded
        };
        let seed = 0x1234_5678;
        let mut expected = OriginalRng::new(seed);
        let expected_words = [
            (expected.next_word() as i16, expected.next_word() as i16),
            (0, 0),
            (expected.next_word() as i16, expected.next_word() as i16),
        ];

        let mut rng = OriginalRng::new(seed);
        let words = original_motion_random_words(&[bounded, absolute, bounded], &mut rng);

        assert_eq!(words, expected_words);
        assert_eq!(rng.seed(), expected.seed());
    }

    #[test]
    fn original_motion_trig_words_use_128_phase_quadrants() {
        let trig = original_motion_trig_words();

        assert_eq!(trig[0x00], 0);
        assert_eq!(trig[0x20], ORIGINAL_MOTION_TRIG_SCALE as i16);
        assert_eq!(trig[0x40], 0);
        assert_eq!(trig[0x60], -(ORIGINAL_MOTION_TRIG_SCALE as i16));
        assert_eq!(trig.len(), 128);
        for phase in 0..0x40 {
            assert_eq!(trig[phase], trig[0x40 - phase]);
        }
        for phase in 0..0x40 {
            assert_eq!(trig[phase], -trig[(phase + 0x40) & 0x7f]);
        }
    }

    #[test]
    fn high_score_rank_accepts_scores_that_make_the_table() {
        let records = vec![
            HighScore {
                score: 70_000,
                level: 1,
                name: "a".to_string(),
            },
            HighScore {
                score: 3000,
                level: 1,
                name: "b".to_string(),
            },
            HighScore {
                score: 1000,
                level: 1,
                name: "c".to_string(),
            },
        ];

        assert_eq!(high_score_rank(&records, 65_536), Some(1));
        assert_eq!(high_score_rank(&records, 4000), Some(1));
        assert_eq!(high_score_rank(&records, 500), Some(3));

        let full = vec![
            HighScore {
                score: 7000,
                level: 1,
                name: "a".to_string(),
            },
            HighScore {
                score: 6000,
                level: 1,
                name: "b".to_string(),
            },
            HighScore {
                score: 5000,
                level: 1,
                name: "c".to_string(),
            },
            HighScore {
                score: 4000,
                level: 1,
                name: "d".to_string(),
            },
            HighScore {
                score: 3000,
                level: 1,
                name: "e".to_string(),
            },
            HighScore {
                score: 2000,
                level: 1,
                name: "f".to_string(),
            },
            HighScore {
                score: 1000,
                level: 1,
                name: "g".to_string(),
            },
        ];
        assert_eq!(high_score_rank(&full, 2500), Some(5));
        assert_eq!(high_score_rank(&full, 1000), None);
    }

    #[test]
    fn insert_high_score_keeps_records_sorted_and_capped() {
        let mut records = vec![
            HighScore {
                score: 7000,
                level: 1,
                name: "a".to_string(),
            },
            HighScore {
                score: 6000,
                level: 1,
                name: "b".to_string(),
            },
            HighScore {
                score: 5000,
                level: 1,
                name: "c".to_string(),
            },
            HighScore {
                score: 4000,
                level: 1,
                name: "d".to_string(),
            },
            HighScore {
                score: 3000,
                level: 1,
                name: "e".to_string(),
            },
            HighScore {
                score: 2000,
                level: 1,
                name: "f".to_string(),
            },
            HighScore {
                score: 1000,
                level: 1,
                name: "g".to_string(),
            },
        ];

        insert_high_score(
            &mut records,
            HighScore {
                score: 4500,
                level: 3,
                name: "new".to_string(),
            },
        );

        assert_eq!(records.len(), 7);
        assert_eq!(
            records[3],
            HighScore {
                score: 4500,
                level: 3,
                name: "new".to_string()
            }
        );
        assert_eq!(records.last().unwrap().score, 2000);
    }

    #[test]
    fn high_score_name_entry_matches_original_letter_space_filter() {
        assert_eq!(normalize_high_score_name_char('A'), Some('a'));
        assert_eq!(normalize_high_score_name_char('z'), Some('z'));
        assert_eq!(normalize_high_score_name_char(' '), Some(' '));
        assert_eq!(normalize_high_score_name_char('1'), None);
        assert_eq!(normalize_high_score_name_char('-'), None);
    }

    #[test]
    fn dead_level_restarts_after_wait_when_lives_remain() {
        let mut player = Player::new(0.0, 0.0, 0);
        player.die();
        player.respawn_timer = -DEATH_RESTART_WAIT;

        assert!(Game::should_restart_dead_level(&[player]));
    }

    #[test]
    fn dead_level_does_not_restart_while_a_player_is_active_or_out_of_lives() {
        let active = Player::new(0.0, 0.0, 0);
        let mut dead = Player::new(0.0, 0.0, 0);
        dead.die();
        dead.respawn_timer = -DEATH_RESTART_WAIT;

        assert!(!Game::should_restart_dead_level(&[active, dead.clone()]));

        dead.lives = 0;
        assert!(!Game::should_restart_dead_level(&[dead]));
    }

    #[test]
    fn level_complete_delay_matches_original_100_frames() {
        assert_eq!(LEVEL_COMPLETE_DELAY, 100.0 / 70.0);
    }
}
