use crate::input::PlayerInput;
use crate::level::*;
use crate::monsters::*;
use crate::player::*;
use crate::sound::*;
use crate::renderer::{SCREEN_W, PLAY_HEIGHT};

const TILE_SIZE: f32 = 8.0;
const EXPLOSION_DURATION: f32 = 0.5;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    TitleScreen, MainMenu, Instructions, Records, Info,
    LevelIntro, Playing, LevelComplete, GameOver, FinalScore,
}

pub struct Game {
    pub state: GameState,
    pub english: bool,
    pub two_player: bool,
    pub current_level: usize,
    pub levels: Vec<Level>,
    pub monster_defs: Vec<MonsterDef>,
    pub players: Vec<Player>,
    pub monsters: Vec<Monster>,
    pub bombs: Vec<Bomb>,
    pub debris: Vec<Debris>,
    pub powerups: Vec<Powerup>,
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
    pub fn new(levels: Vec<Level>, monster_defs: Vec<MonsterDef>, records: Vec<crate::assets::HighScore>) -> Self {
        Game {
            state: GameState::TitleScreen, english: false, two_player: false,
            current_level: 0, levels, monster_defs,
            players: Vec::new(), monsters: Vec::new(), bombs: Vec::new(),
            debris: Vec::new(), powerups: Vec::new(),
            scroll_x: 0.0, scroll_y: 0.0,
            initial_solid_count: 0, current_destruction_pct: 0.0,
            state_timer: 0.0, game_time: 0.0,
            show_background: true, screen_width_factor: 1.0,
            records, sound: SoundManager::new(),
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
        let md = &self.monster_defs[li % self.monster_defs.len()];
        self.monsters = spawn_monsters(&self.levels[li], md, li);
        self.bombs.clear(); self.debris.clear(); self.powerups.clear();
        self.initial_solid_count = self.levels[li].count_solid_tiles();
        self.scroll_x = 0.0; self.scroll_y = 0.0;
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
            else { b.timer -= dt; if b.timer <= 0.0 { b.exploding = true; b.explosion_timer = EXPLOSION_DURATION; expl.push((b.x,b.y,b.bomb_type)); self.sound.play(SoundEffect::Explosion); } }
        }
        for (ex,ey,bt) in &expl { self.process_explosion(*ex,*ey,*bt,li); }
        self.bombs.retain(|b| !b.exploding || b.explosion_timer > 0.0);

        let ps: Vec<Player> = self.players.clone();
        for m in &mut self.monsters {
            m.update(&self.levels[li], &ps, dt);
            for p in &mut self.players {
                if m.collides_with_player(p) { p.take_damage(m.monster_type.damage() * dt * 30.0); }
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
                        PowerupType::YellowBombBox => { p.bombs[0]+=20; p.bombs[1]+=10; p.bombs[2]+=5; }
                        PowerupType::GreenBombBox => { p.has_super_bombs = true; p.bombs[3]+=5; }
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

        let cs = self.levels[li].count_solid_tiles();
        if self.initial_solid_count > 0 { self.current_destruction_pct = (1.0 - cs as f32/self.initial_solid_count as f32)*100.0; }

        let lev = &self.levels[li];
        let bm = self.players.iter().any(|p| p.bonuses_collected >= lev.bonus_target as u32);
        let dm = self.current_destruction_pct >= lev.destruction_pct as f32;
        if bm && dm { self.state = GameState::LevelComplete; self.state_timer = 3.0; self.sound.play(SoundEffect::LevelComplete); }

        if self.players.iter().all(|p| !p.alive && p.lives <= 0) { self.state = GameState::GameOver; self.state_timer = 5.0; }
    }

    fn process_explosion(&mut self, ex: f32, ey: f32, bt: BombType, li: usize) {
        let radius = bt.radius();
        let damage = bt.damage();
        let cx = ((ex+8.0)/TILE_SIZE) as i32;
        let cy = ((ey+8.0)/TILE_SIZE) as i32;
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
        for m in &mut self.monsters {
            let dx = m.x-ex; let dy = m.y-ey; let dist = (dx*dx+dy*dy).sqrt();
            if dist <= radius {
                let was = m.alive;
                m.take_damage(damage*(1.0-dist/radius));
                if was && !m.alive { self.powerups.push(Powerup::new(m.x, m.y, PowerupType::random())); }
            }
        }
        for p in &mut self.players {
            let dx = p.x-ex; let dy = p.y-ey; let dist = (dx*dx+dy*dy).sqrt();
            if dist <= radius { p.take_damage(damage*0.5*(1.0-dist/radius)); }
        }
    }
}
