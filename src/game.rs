use crate::input::PlayerInput;
use crate::level::*;
use crate::monsters::*;
use crate::player::*;
use crate::sound::*;
use crate::renderer::{SCREEN_W, PLAY_HEIGHT};

const TILE_SIZE: f32 = 8.0;
const EXPLOSION_DURATION: f32 = 0.5;
/// 70 Hz frames between crack stages 0x76 → 0x77 → 0x78 → 0x79 → cleared.
const COLLAPSE_FRAME_TIME: f32 = 4.0 / 70.0;
/// Per-frame gravity for falling rubble (in pixels/frame at 70 Hz).
const RUBBLE_GRAVITY: f32 = 0x40 as f32 / 256.0;
const RUBBLE_MAX_FALL: f32 = 0x7FF as f32 / 256.0;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    TitleScreen, MainMenu, Instructions, Records, Info,
    LevelIntro, Playing, LevelComplete, GameOver, FinalScore,
}

/// A piece of falling rubble. Removed from the level grid (so collisions and
/// destruction% see it as gone) and rendered as an overlay sprite that cycles
/// through crack stages 0x76→0x79 while accumulating vertical velocity per
/// §11.2. Lands when it hits a solid tile beneath, or vanishes off the map.
#[derive(Clone, Copy)]
pub struct Collapsing {
    pub tx: usize,
    pub ty: usize,
    pub stage: u8,        // 0x76, 0x77, 0x78, 0x79
    pub timer: f32,
    pub vy: f32,
    pub y_offset: f32,
}

pub struct Game {
    pub state: GameState,
    pub english: bool,
    pub two_player: bool,
    pub current_level: usize,
    pub levels: Vec<Level>,
    pub monster_defs: Vec<MonsterTemplate>,
    pub players: Vec<Player>,
    pub monsters: Vec<Monster>,
    pub bombs: Vec<Bomb>,
    pub debris: Vec<Debris>,
    pub powerups: Vec<Powerup>,
    pub collapsing: Vec<Collapsing>,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub initial_solid_count: usize,
    pub current_destruction_pct: f32,
    pub state_timer: f32,
    pub game_time: f32,
    pub show_background: bool,
    pub screen_width_factor: f32,
    pub records: Vec<crate::assets::HighScore>,
    pub sound: SoundManager,
    pub p1_start_x: f32, pub p1_start_y: f32,
    pub p2_start_x: f32, pub p2_start_y: f32,
}

impl Game {
    pub fn new(levels: Vec<Level>, monster_defs: Vec<MonsterTemplate>, records: Vec<crate::assets::HighScore>, sound: SoundManager) -> Self {
        Game {
            state: GameState::TitleScreen, english: false, two_player: false,
            current_level: 0, levels, monster_defs,
            players: Vec::new(), monsters: Vec::new(), bombs: Vec::new(),
            debris: Vec::new(), powerups: Vec::new(), collapsing: Vec::new(),
            scroll_x: 0.0, scroll_y: 0.0,
            initial_solid_count: 0, current_destruction_pct: 0.0,
            state_timer: 0.0, game_time: 0.0,
            show_background: true, screen_width_factor: 1.0,
            records, sound,
            p1_start_x: 40.0, p1_start_y: 40.0,
            p2_start_x: 80.0, p2_start_y: 40.0,
        }
    }

    pub fn start_game(&mut self, two_player: bool) {
        self.two_player = two_player;
        self.current_level = 0;
        self.players.clear();
        self.players.push(Player::new(0.0, 0.0, 0));
        if two_player { self.players.push(Player::new(0.0, 0.0, 1)); }
        self.start_level();
        self.state = GameState::LevelIntro;
        self.state_timer = 3.0;
    }

    pub fn start_level(&mut self) {
        let li = self.current_level;
        if li >= self.levels.len() { self.state = GameState::FinalScore; return; }
        self.find_start_positions();
        if let Some(p) = self.players.get_mut(0) { p.respawn(self.p1_start_x, self.p1_start_y); }
        if let Some(p) = self.players.get_mut(1) { p.respawn(self.p2_start_x, self.p2_start_y); }
        self.monsters = spawn_monsters(&self.levels[li], &self.monster_defs, li);
        self.bombs.clear(); self.debris.clear(); self.powerups.clear();
        self.collapsing.clear();
        // Pre-spawn pickups from the level's bonus table (per LIVELS.SCH §1.3
        // and bonus types §6.4). Each record's raw[0]/raw[2] are tile coords
        // and raw[4] is the bonus id.
        for b in &self.levels[li].bonuses {
            let tx = b.raw[0] as f32 * TILE_SIZE;
            let ty = (b.raw[2] as f32 * TILE_SIZE) - 8.0;
            let pt = PowerupType::from_bonus_id(b.raw[4]);
            let mut p = Powerup::new(tx, ty, pt);
            p.vy = 0.0;
            self.powerups.push(p);
        }
        self.initial_solid_count = self.levels[li].initial_variant_count as usize;
        self.scroll_x = self.levels[li].scroll_x as f32 * TILE_SIZE;
        self.scroll_y = self.levels[li].scroll_y as f32 * TILE_SIZE;
    }

    fn find_start_positions(&mut self) {
        let lev = &self.levels[self.current_level];
        for x in 2..lev.width/4 {
            for y in 1..lev.height.saturating_sub(2) {
                if !lev.is_solid(x, y) && y > 0 && !lev.is_solid(x, y-1) && lev.is_solid(x, y+1) {
                    self.p1_start_x = x as f32 * TILE_SIZE;
                    self.p1_start_y = (y as f32 * TILE_SIZE) - 16.0;
                    self.p2_start_x = (x+4) as f32 * TILE_SIZE;
                    self.p2_start_y = (y as f32 * TILE_SIZE) - 16.0;
                    return;
                }
            }
        }
        self.p1_start_x = 40.0; self.p1_start_y = 40.0;
        self.p2_start_x = 80.0; self.p2_start_y = 40.0;
    }

    pub fn update(&mut self, dt: f32) {
        self.game_time += dt;
        match self.state {
            GameState::TitleScreen => {
                if macroquad::input::get_last_key_pressed().is_some() { self.state = GameState::MainMenu; }
            }
            GameState::MainMenu => {
                use macroquad::input::*;
                if is_key_pressed(KeyCode::Key1) { self.start_game(false); }
                else if is_key_pressed(KeyCode::Key2) { self.start_game(true); }
                else if is_key_pressed(KeyCode::L) { self.english = !self.english; }
                else if is_key_pressed(KeyCode::I) { self.state = GameState::Info; }
                else if is_key_pressed(KeyCode::Z) { self.state = GameState::Instructions; }
                else if is_key_pressed(KeyCode::R) { self.state = GameState::Records; }
                else if is_key_pressed(KeyCode::Escape) { std::process::exit(0); }
            }
            GameState::Instructions | GameState::Records | GameState::Info => {
                if macroquad::input::get_last_key_pressed().is_some() { self.state = GameState::MainMenu; }
            }
            GameState::LevelIntro => {
                self.state_timer -= dt;
                if self.state_timer <= 0.0 { self.state = GameState::Playing; }
            }
            GameState::Playing => self.update_playing(dt),
            GameState::LevelComplete => {
                self.state_timer -= dt;
                if self.state_timer <= 0.0 {
                    self.current_level += 1;
                    if self.current_level >= 7 { self.state = GameState::FinalScore; self.state_timer = 5.0; }
                    else { self.start_level(); self.state = GameState::LevelIntro; self.state_timer = 3.0; }
                }
            }
            GameState::GameOver => {
                self.state_timer -= dt;
                if self.state_timer <= 0.0 || macroquad::input::get_last_key_pressed().is_some() {
                    self.state = GameState::FinalScore; self.state_timer = 5.0;
                }
            }
            GameState::FinalScore => {
                self.state_timer -= dt;
                if self.state_timer <= 0.0 || macroquad::input::get_last_key_pressed().is_some() {
                    self.state = GameState::MainMenu;
                }
            }
        }
    }

    fn update_playing(&mut self, dt: f32) {
        let li = self.current_level;
        if li >= self.levels.len() { return; }
        let i1 = PlayerInput::read_player1();
        let i2 = if self.two_player { PlayerInput::read_player2() } else { PlayerInput::default() };
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Escape) { self.state = GameState::MainMenu; return; }
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::S) { self.show_background = !self.show_background; }
        // §11.4: E narrows the visible width, R widens it (single-player only).
        if !self.two_player {
            if macroquad::input::is_key_pressed(macroquad::input::KeyCode::E) {
                self.screen_width_factor = (self.screen_width_factor - 0.1).max(0.5);
            }
            if macroquad::input::is_key_pressed(macroquad::input::KeyCode::R) {
                self.screen_width_factor = (self.screen_width_factor + 0.1).min(1.0);
            }
        }

        let inputs = [i1, i2];
        let mut nb = Vec::new();
        for (i, p) in self.players.iter_mut().enumerate() {
            if !p.alive {
                if p.respawn_timer <= 0.0 && p.lives > 0 && inputs[i].fire {
                    let (sx,sy) = if i==0 { (self.p1_start_x,self.p1_start_y) } else { (self.p2_start_x,self.p2_start_y) };
                    p.respawn(sx, sy);
                } else { p.respawn_timer -= dt; }
                continue;
            }
            if p.update(&inputs[i], &self.levels[li], dt) {
                nb.push(Bomb::new(p.x, p.y, p.current_bomb, i));
                self.sound.play(SoundEffect::PlaceBomb);
            }
        }
        self.bombs.extend(nb);

        let mut expl = Vec::new();
        for b in &mut self.bombs {
            if b.exploding { b.explosion_timer -= dt; }
            else { b.timer -= dt; if b.timer <= 0.0 { b.exploding = true; b.explosion_timer = EXPLOSION_DURATION; expl.push((b.x,b.y,b.bomb_type,b.owner)); self.sound.play(SoundEffect::Explosion); } }
        }
        for (ex,ey,bt,owner) in &expl { self.process_explosion(*ex,*ey,*bt,*owner,li); }
        self.bombs.retain(|b| !b.exploding || b.explosion_timer > 0.0);
        self.update_collapsing(li, dt);

        let ps: Vec<Player> = self.players.clone();
        for m in &mut self.monsters {
            m.update(&self.levels[li], &ps, dt);
            for p in &mut self.players {
                if m.collides_with_player(p) { p.take_damage(m.damage * dt * 30.0); }
            }
        }

        for pu in &mut self.powerups { pu.update(&self.levels[li], dt); }
        let mut bonus_pus: Vec<Powerup> = Vec::new();
        for pu in &mut self.powerups {
            for p in &mut self.players {
                if pu.collides_with_player(p) {
                    p.score += pu.powerup_type.points();
                    p.bonuses_collected += 1;
                    match pu.powerup_type {
                        PowerupType::HotDog => { p.energy = (p.energy + p.max_energy/3.0).min(p.max_energy); }
                        PowerupType::FirstAid => { p.energy = p.max_energy; }
                        // Bonus type 4: 46 frames at 70 Hz ≈ 0.66 seconds.
                        PowerupType::Invincibility => { p.invincible_timer = p.invincible_timer.max(46.0 / 70.0); }
                        PowerupType::YellowBombBox => {
                            // Bonus type 5: random qty of random types.
                            for _ in 0..3 {
                                let slot = macroquad::rand::gen_range(0u32, 4) as usize;
                                let qty = macroquad::rand::gen_range(2i32, 8);
                                p.bombs[slot] += qty;
                            }
                        }
                        PowerupType::GreenBombBox => {
                            // Bonus type 6: larger supply incl. super bombs.
                            p.has_super_bombs = true;
                            p.bombs[0]+=15; p.bombs[1]+=10; p.bombs[2]+=8; p.bombs[3]+=5;
                        }
                        PowerupType::JollyCloud => {
                            for j in 0..3 {
                                bonus_pus.push(Powerup::new(p.x + macroquad::rand::gen_range(-40.0f32,40.0), p.y-30.0-j as f32*20.0, PowerupType::Present));
                            }
                        }
                        _ => {}
                    }
                    pu.alive = false;
                    self.sound.play(SoundEffect::Pickup);
                }
            }
        }
        self.powerups.extend(bonus_pus);
        self.powerups.retain(|p| p.alive);

        for d in &mut self.debris { d.x += d.vx; d.y += d.vy; d.vy += 0.15; d.life -= dt; }
        self.debris.retain(|d| d.life > 0.0);

        if let Some(p) = self.players.first() {
            if p.alive {
                let tx = p.x - SCREEN_W/2.0+6.0;
                let ty = p.y - PLAY_HEIGHT/2.0+8.0;
                self.scroll_x += (tx - self.scroll_x)*0.1;
                self.scroll_y += (ty - self.scroll_y)*0.1;
                let mx = (self.levels[li].width as f32*TILE_SIZE)-SCREEN_W;
                let my = (self.levels[li].height as f32*TILE_SIZE)-PLAY_HEIGHT;
                self.scroll_x = self.scroll_x.clamp(0.0, mx.max(0.0));
                self.scroll_y = self.scroll_y.clamp(0.0, my.max(0.0));
            }
        }

        let cs = self.levels[li].count_variant_tiles() as usize;
        if self.initial_solid_count > 0 { self.current_destruction_pct = (1.0 - cs as f32/self.initial_solid_count as f32)*100.0; }

        let lev = &self.levels[li];
        let bm = self.players.iter().any(|p| p.bonuses_collected >= lev.bonus_target as u32);
        let dm = self.current_destruction_pct >= lev.destruction_pct as f32;
        if bm && dm { self.state = GameState::LevelComplete; self.state_timer = 3.0; self.sound.play(SoundEffect::LevelComplete); }

        if self.players.iter().all(|p| !p.alive && p.lives <= 0) { self.state = GameState::GameOver; self.state_timer = 5.0; }
    }

    fn process_explosion(&mut self, ex: f32, ey: f32, bt: BombType, owner: usize, li: usize) {
        let radius = bt.radius();
        let damage = bt.damage();
        let cx = ((ex+8.0)/TILE_SIZE) as i32;
        let cy = ((ey+8.0)/TILE_SIZE) as i32;

        // GAME_SPEC §4.4: read the attribute at the bomb tile. If it's a
        // building group (< 0x4000) flood-fill all connected tiles sharing
        // that attribute and clear them. Then fall back to a radius blast for
        // any remaining tiles inside the explosion sphere.
        if cx >= 0 && cy >= 0 && (cx as usize) < self.levels[li].width && (cy as usize) < self.levels[li].height {
            let attr = self.levels[li].attr_at(cx as usize, cy as usize);
            if attr != 0 && attr < 0x4000 {
                self.flood_fill_destroy(li, cx as usize, cy as usize, attr, owner);
            }
        }

        let tr = (radius/TILE_SIZE) as i32 + 1;
        for ty in (cy-tr)..=(cy+tr) {
            for tx in (cx-tr)..=(cx+tr) {
                if tx < 0 || ty < 0 { continue; }
                let dx = (tx-cx) as f32*TILE_SIZE;
                let dy = (ty-cy) as f32*TILE_SIZE;
                let dist = (dx*dx+dy*dy).sqrt();
                if dist <= radius {
                    let old = self.levels[li].tile_at(tx as usize, ty as usize);
                    if old != 0 {
                        self.levels[li].set_tile(tx as usize, ty as usize, 0);
                        for _ in 0..3 {
                            let a = macroquad::rand::gen_range(0.0f32, std::f32::consts::TAU);
                            let s = macroquad::rand::gen_range(1.0f32, 4.0);
                            self.debris.push(Debris { x: tx as f32*TILE_SIZE+4.0, y: ty as f32*TILE_SIZE+4.0, vx: a.cos()*s, vy: a.sin()*s-2.0, color: old, life: macroquad::rand::gen_range(0.5f32, 2.0) });
                        }
                    }
                }
            }
        }
        let mut kills: Vec<(f32, f32)> = Vec::new();
        for m in &mut self.monsters {
            let dx = m.x-ex; let dy = m.y-ey; let dist = (dx*dx+dy*dy).sqrt();
            if dist <= radius {
                let was = m.alive;
                m.take_damage(damage*(1.0-dist/radius));
                if was && !m.alive { kills.push((m.x, m.y)); }
            }
        }
        for (mx, my) in kills {
            self.powerups.push(Powerup::new(mx, my, PowerupType::random()));
            if let Some(p) = self.players.get_mut(owner) {
                p.score = p.score.saturating_add(500);
            }
        }
        for p in &mut self.players {
            let dx = p.x-ex; let dy = p.y-ey; let dist = (dx*dx+dy*dy).sqrt();
            if dist <= radius { p.take_damage(damage*0.5*(1.0-dist/radius)); }
        }
    }

    /// 4-connected flood fill over a building group: every reachable tile
    /// whose attribute equals `attr` is replaced with crack stage 0x76 and
    /// queued for collapse animation. Mirrors the rectangle-expansion
    /// algorithm in FUN_1000_370e (with §11.2 crack states).
    fn flood_fill_destroy(&mut self, li: usize, sx: usize, sy: usize, attr: u16, owner: usize) {
        let (w, h) = {
            let lev = &self.levels[li];
            (lev.width, lev.height)
        };
        if self.levels[li].attr_at(sx, sy) != attr { return; }
        let mut stack: Vec<(usize, usize)> = Vec::with_capacity(64);
        stack.push((sx, sy));
        let mut destroyed = 0u32;
        while let Some((x, y)) = stack.pop() {
            let idx = y * w + x;
            let lev = &mut self.levels[li];
            if lev.attrs[idx] != attr || lev.tiles[idx] == 0 { continue; }
            let old = lev.tiles[idx];
            // Clear from the level immediately so collisions and destruction%
            // see it as gone; the overlay carries the visual until it lands.
            lev.tiles[idx] = 0;
            self.collapsing.push(Collapsing {
                tx: x, ty: y,
                stage: 0x76, timer: COLLAPSE_FRAME_TIME,
                vy: 0.0, y_offset: 0.0,
            });
            destroyed += 1;
            for _ in 0..2 {
                let a = macroquad::rand::gen_range(0.0f32, std::f32::consts::TAU);
                let s = macroquad::rand::gen_range(1.0f32, 3.0);
                self.debris.push(Debris {
                    x: x as f32 * TILE_SIZE + 4.0,
                    y: y as f32 * TILE_SIZE + 4.0,
                    vx: a.cos()*s, vy: a.sin()*s - 2.0,
                    color: old, life: macroquad::rand::gen_range(0.5f32, 1.5),
                });
            }
            if x + 1 < w { stack.push((x + 1, y)); }
            if x > 0 { stack.push((x - 1, y)); }
            if y + 1 < h { stack.push((x, y + 1)); }
            if y > 0 { stack.push((x, y - 1)); }
        }
        if destroyed > 0 {
            if let Some(p) = self.players.get_mut(owner) {
                p.score = p.score.saturating_add(destroyed * 10);
            }
        }
    }

    /// Advance collapse animation and falling physics. Each piece accumulates
    /// gravity and sub-pixel vertical offset; the crack stage cycles 0x76→0x79
    /// over time. The piece is removed when it lands on a solid tile, drops
    /// off the bottom of the level, or finishes its crack cycle in mid-air.
    fn update_collapsing(&mut self, li: usize, dt: f32) {
        if self.collapsing.is_empty() { return; }
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
            let landed = next_row >= lev_h
                || self.levels[li].is_solid(c.tx, next_row);
            // Damage entities standing in the falling cell. Per-frame at 70 Hz,
            // so the constants are tuned for ~7 dps to players, ~28 dps to
            // monsters when overlap persists.
            let py = c.ty as f32 * TILE_SIZE + c.y_offset;
            let px = c.tx as f32 * TILE_SIZE;
            for p in &mut self.players {
                if p.alive
                    && p.x + 12.0 > px && p.x < px + TILE_SIZE
                    && p.y + 16.0 > py && p.y < py + TILE_SIZE {
                    p.take_damage(0.1);
                }
            }
            for m in &mut self.monsters {
                if m.alive
                    && m.x + 14.0 > px && m.x < px + TILE_SIZE
                    && m.y + 14.0 > py && m.y < py + TILE_SIZE {
                    m.take_damage(0.4);
                }
            }
            if landed { continue; }
            if c.stage >= 0x79 && c.timer <= 0.0 { continue; }
            still_alive.push(c);
        }
        self.collapsing = still_alive;
    }
}
