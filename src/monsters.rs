use crate::level::{Level, MonsterTemplate};
use crate::player::Player;

const TILE_SIZE: f32 = 8.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MonsterType { Walker, Chaser, Floater, Jumper }
impl MonsterType {
    pub fn from_id(id: u8) -> Self {
        match id % 4 { 0 => Self::Walker, 1 => Self::Chaser, 2 => Self::Floater, _ => Self::Jumper }
    }
    pub fn speed(&self) -> f32 { match self { Self::Walker=>0.5, Self::Chaser=>0.7, Self::Floater=>0.4, Self::Jumper=>0.6 } }
    pub fn damage(&self) -> f32 { match self { Self::Walker=>0.5, Self::Chaser=>0.8, Self::Floater=>0.3, Self::Jumper=>0.6 } }
    pub fn health(&self) -> f32 { match self { Self::Walker=>20.0, Self::Chaser=>30.0, Self::Floater=>15.0, Self::Jumper=>25.0 } }
    pub fn sprite_base(&self) -> usize { match self { Self::Walker=>10, Self::Chaser=>20, Self::Floater=>30, Self::Jumper=>40 } }
}

#[derive(Clone)]
pub struct Monster {
    pub x: f32, pub y: f32, pub vx: f32, pub vy: f32,
    pub monster_type: MonsterType,
    pub health: f32, pub alive: bool, pub facing_right: bool,
    pub anim_frame: usize, pub anim_timer: f32,
    pub patrol_timer: f32,
    pub start_x: f32, pub start_y: f32,
    /// Sprite-sheet base index pulled from GRAN.MST (or fallback per type).
    pub sprite_base: usize,
    /// Per-frame movement speed in pixels (from GRAN.MST byte 1, fallback per type).
    pub speed: f32,
}

impl Monster {
    pub fn new(x: f32, y: f32, mt: MonsterType, sprite_base: usize, speed: f32) -> Self {
        Monster {
            x, y, vx: speed, vy: 0.0,
            monster_type: mt, health: mt.health(),
            alive: true, facing_right: true,
            anim_frame: 0, anim_timer: 0.0,
            patrol_timer: 0.0, start_x: x, start_y: y,
            sprite_base, speed,
        }
    }

    pub fn update(&mut self, level: &Level, players: &[Player], dt: f32) {
        if !self.alive { return; }
        self.anim_timer += dt;
        if self.anim_timer > 0.15 { self.anim_timer = 0.0; self.anim_frame = (self.anim_frame + 1) % 4; }

        match self.monster_type {
            MonsterType::Walker => {
                self.vy += 0.3; if self.vy > 3.0 { self.vy = 3.0; }
                let nx = self.x + self.vx;
                let tx = if self.vx > 0.0 { ((nx+15.0)/TILE_SIZE) as usize } else { (nx/TILE_SIZE) as usize };
                let ty = (self.y/TILE_SIZE) as usize;
                if level.is_solid(tx, ty) || level.is_solid(tx, ty+1) {
                    self.vx = -self.vx; self.facing_right = self.vx > 0.0;
                }
                let gx = if self.vx > 0.0 { ((self.x+15.0)/TILE_SIZE) as usize } else { (self.x/TILE_SIZE) as usize };
                let gy = ((self.y+16.0)/TILE_SIZE) as usize;
                if !level.is_solid(gx, gy) && self.vy <= 0.1 {
                    self.vx = -self.vx; self.facing_right = self.vx > 0.0;
                }
                self.x += self.vx;
                let ny = self.y + self.vy;
                let bty = ((ny+15.0)/TILE_SIZE) as usize;
                let ctx = ((self.x+8.0)/TILE_SIZE) as usize;
                if level.is_solid(ctx, bty) { self.y = (bty as f32*TILE_SIZE)-16.0; self.vy = 0.0; }
                else { self.y = ny; }
            }
            MonsterType::Chaser => {
                let mut nd = f32::MAX; let mut tx = self.x;
                for p in players {
                    if p.alive {
                        let d = (p.x-self.x)*(p.x-self.x) + (p.y-self.y)*(p.y-self.y);
                        if d < nd { nd = d; tx = p.x; }
                    }
                }
                let s = self.speed;
                if tx > self.x+2.0 { self.vx = s; self.facing_right = true; }
                else if tx < self.x-2.0 { self.vx = -s; self.facing_right = false; }
                else { self.vx = 0.0; }
                self.vy += 0.3; if self.vy > 3.0 { self.vy = 3.0; }
                self.x += self.vx;
                let ny = self.y + self.vy;
                let bty = ((ny+15.0)/TILE_SIZE) as usize;
                let ctx = ((self.x+8.0)/TILE_SIZE) as usize;
                if level.is_solid(ctx, bty) { self.y = (bty as f32*TILE_SIZE)-16.0; self.vy = 0.0; }
                else { self.y = ny; }
            }
            MonsterType::Floater => {
                self.patrol_timer += dt;
                self.x = self.start_x + (self.patrol_timer*1.5).sin()*40.0;
                self.y = self.start_y + (self.patrol_timer*2.0).cos()*20.0;
                self.facing_right = (self.patrol_timer*1.5).cos() > 0.0;
            }
            MonsterType::Jumper => {
                self.patrol_timer += dt;
                self.vy += 0.3; if self.vy > 3.0 { self.vy = 3.0; }
                if self.patrol_timer > 1.5 { self.patrol_timer = 0.0; self.vy = -3.5; }
                self.x += self.vx;
                let tx = if self.vx > 0.0 { ((self.x+15.0)/TILE_SIZE) as usize } else { (self.x/TILE_SIZE) as usize };
                let ty = ((self.y+8.0)/TILE_SIZE) as usize;
                if level.is_solid(tx, ty) { self.vx = -self.vx; self.facing_right = self.vx > 0.0; }
                let ny = self.y + self.vy;
                let bty = ((ny+15.0)/TILE_SIZE) as usize;
                let ctx = ((self.x+8.0)/TILE_SIZE) as usize;
                if level.is_solid(ctx, bty) { self.y = (bty as f32*TILE_SIZE)-16.0; self.vy = 0.0; }
                else { self.y = ny; }
            }
        }
        let mx = (level.width as f32*TILE_SIZE) - 16.0;
        let my = (level.height as f32*TILE_SIZE) - 16.0;
        self.x = self.x.clamp(0.0, mx);
        self.y = self.y.clamp(0.0, my);
    }

    pub fn take_damage(&mut self, amount: f32) { self.health -= amount; if self.health <= 0.0 { self.alive = false; } }
    pub fn sprite_index(&self) -> usize { self.sprite_base + self.anim_frame }
    pub fn collides_with_player(&self, p: &Player) -> bool {
        if !self.alive || !p.alive { return false; }
        self.x < p.x+12.0 && self.x+14.0 > p.x && self.y < p.y+16.0 && self.y+14.0 > p.y
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PowerupType { Present, HotDog, FirstAid, Invincibility, YellowBombBox, GreenBombBox, JollyCloud, BigDiamond }
impl PowerupType {
    pub fn random() -> Self {
        match macroquad::rand::gen_range(0u32, 8) {
            0 => Self::Present, 1 => Self::HotDog, 2 => Self::FirstAid,
            3 => Self::Invincibility, 4 => Self::YellowBombBox, 5 => Self::GreenBombBox,
            6 => Self::JollyCloud, _ => Self::BigDiamond,
        }
    }
    /// From a 7-byte BonusSpawn record (bonus type in raw[4]) per spec §6.4.
    pub fn from_bonus_id(b: u8) -> Self {
        match b {
            2 => Self::FirstAid,        // full energy
            3 => Self::HotDog,          // +33 energy
            4 => Self::Invincibility,   // 46 frames invuln
            5 => Self::YellowBombBox,   // random bombs
            6 => Self::GreenBombBox,    // larger bomb supply
            _ => Self::Present,
        }
    }
    pub fn points(&self) -> u32 {
        match self { Self::Present=>2000, Self::HotDog=>1500, Self::FirstAid=>1000,
            Self::Invincibility=>2500,
            Self::YellowBombBox=>3000, Self::GreenBombBox=>1000, Self::JollyCloud=>2000, Self::BigDiamond=>5000 }
    }
    pub fn sprite_index(&self) -> usize {
        match self { Self::Present=>50, Self::HotDog=>51, Self::FirstAid=>52,
            Self::Invincibility=>53,
            Self::YellowBombBox=>54, Self::GreenBombBox=>55, Self::JollyCloud=>56, Self::BigDiamond=>57 }
    }
}

#[derive(Clone)]
pub struct Powerup {
    pub x: f32, pub y: f32, pub vy: f32,
    pub powerup_type: PowerupType,
    pub alive: bool, pub timer: f32,
}
impl Powerup {
    pub fn new(x: f32, y: f32, pt: PowerupType) -> Self {
        Powerup { x, y, vy: -1.0, powerup_type: pt, alive: true, timer: 8.0 }
    }
    pub fn update(&mut self, level: &Level, dt: f32) {
        if !self.alive { return; }
        self.timer -= dt;
        if self.timer <= 0.0 { self.alive = false; return; }
        self.vy += 0.2; if self.vy > 2.0 { self.vy = 2.0; }
        let ny = self.y + self.vy;
        let bty = ((ny+15.0)/TILE_SIZE) as usize;
        let ctx = ((self.x+8.0)/TILE_SIZE) as usize;
        if level.is_solid(ctx, bty) { self.y = (bty as f32*TILE_SIZE)-16.0; self.vy = 0.0; }
        else { self.y = ny; }
    }
    pub fn collides_with_player(&self, p: &Player) -> bool {
        if !self.alive || !p.alive { return false; }
        self.x < p.x+12.0 && self.x+14.0 > p.x && self.y < p.y+16.0 && self.y+14.0 > p.y
    }
}

/// Spawn monsters from the level's monster table (per LIVELS.SCH spawn records).
/// Each spawn is 30 raw bytes; raw[0]/raw[2] are tile coords and raw[4] picks
/// a GRAN.MST template (sprite base + speed). We blend the template's
/// sprite_base/speed with the per-type defaults so unknown templates still
/// look reasonable.
pub fn spawn_monsters(level: &Level, templates: &[MonsterTemplate], _level_idx: usize) -> Vec<Monster> {
    let mut monsters = Vec::new();
    for s in &level.monsters {
        let tx = s.raw[0] as usize;
        let ty = s.raw[2] as usize;
        let tidx = s.raw[4] as usize;
        let mt = MonsterType::from_id(tidx as u8);
        let sx = tx as f32 * TILE_SIZE;
        let sy = (ty as f32 * TILE_SIZE) - 16.0;
        let (sprite_base, speed) = if !templates.is_empty() {
            let t = &templates[tidx % templates.len()];
            // GRAN.MST byte 1 is a 0..255 speed value; rescale to ~0.2..1.5 px/frame.
            let sp = (t.speed.max(1) as f32 / 8.0).clamp(0.2, 1.5);
            (t.sprite_base as usize, sp)
        } else {
            (mt.sprite_base(), mt.speed())
        };
        monsters.push(Monster::new(sx, sy, mt, sprite_base, speed));
    }
    monsters
}
