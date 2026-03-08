use crate::input::PlayerInput;
use crate::level::Level;

const TILE_SIZE: f32 = 8.0;
const GRAVITY: f32 = 0.4;
const JUMP_VEL: f32 = -4.5;
const MOVE_SPD: f32 = 1.5;
const MAX_FALL: f32 = 4.0;
const PW: f32 = 12.0;
const PH: f32 = 16.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BombType { Small = 0, Medium = 1, Large = 2, Super = 3 }
impl BombType {
    pub fn radius(&self) -> f32 { match self { Self::Small=>16.0, Self::Medium=>24.0, Self::Large=>32.0, Self::Super=>48.0 } }
    pub fn damage(&self) -> f32 { match self { Self::Small=>5.0, Self::Medium=>10.0, Self::Large=>20.0, Self::Super=>35.0 } }
    pub fn fuse_time(&self) -> f32 { match self { Self::Small=>2.0, Self::Medium=>2.5, Self::Large=>3.0, Self::Super=>3.5 } }
    pub fn next(&self) -> Self { match self { Self::Small=>Self::Medium, Self::Medium=>Self::Large, Self::Large=>Self::Super, Self::Super=>Self::Small } }
    pub fn sprite_index(&self) -> usize { *self as usize }
}

#[derive(Clone)]
pub struct Bomb {
    pub x: f32, pub y: f32,
    pub bomb_type: BombType,
    pub timer: f32, pub owner: usize,
    pub exploding: bool, pub explosion_timer: f32,
}
impl Bomb {
    pub fn new(x: f32, y: f32, bt: BombType, owner: usize) -> Self {
        Bomb { x, y, bomb_type: bt, timer: bt.fuse_time(), owner, exploding: false, explosion_timer: 0.0 }
    }
}

#[derive(Clone)]
pub struct Debris {
    pub x: f32, pub y: f32, pub vx: f32, pub vy: f32,
    pub color: u8, pub life: f32,
}

#[derive(Clone)]
pub struct Player {
    pub x: f32, pub y: f32, pub vx: f32, pub vy: f32,
    pub on_ground: bool, pub facing_right: bool,
    pub energy: f32, pub max_energy: f32,
    pub lives: i32, pub score: u32,
    pub bombs: [i32; 4],
    pub current_bomb: BombType,
    pub has_super_bombs: bool,
    pub bonuses_collected: u32,
    pub alive: bool, pub respawn_timer: f32,
    pub anim_frame: usize, pub anim_timer: f32,
    pub invincible_timer: f32,
    pub player_idx: usize,
}

impl Player {
    pub fn new(x: f32, y: f32, idx: usize) -> Self {
        Player {
            x, y, vx: 0.0, vy: 0.0,
            on_ground: false, facing_right: true,
            energy: 100.0, max_energy: 100.0,
            lives: 3, score: 0,
            bombs: [99, 10, 5, 0],
            current_bomb: BombType::Small,
            has_super_bombs: false,
            bonuses_collected: 0,
            alive: true, respawn_timer: 0.0,
            anim_frame: 0, anim_timer: 0.0,
            invincible_timer: 0.0,
            player_idx: idx,
        }
    }

    pub fn update(&mut self, input: &PlayerInput, level: &Level, dt: f32) -> bool {
        if !self.alive { self.respawn_timer -= dt; return false; }
        if self.invincible_timer > 0.0 { self.invincible_timer -= dt; }
        let mut place_bomb = false;

        if input.change_weapon() {
            self.current_bomb = self.current_bomb.next();
            if self.current_bomb == BombType::Super && !self.has_super_bombs {
                self.current_bomb = BombType::Small;
            }
        } else {
            if input.left { self.vx = -MOVE_SPD; self.facing_right = false; }
            else if input.right { self.vx = MOVE_SPD; self.facing_right = true; }
            else { self.vx = 0.0; }
        }

        if input.jump && self.on_ground { self.vy = JUMP_VEL; self.on_ground = false; }
        if input.fire {
            let bi = self.current_bomb as usize;
            if self.bombs[bi] > 0 { self.bombs[bi] -= 1; place_bomb = true; }
        }

        self.vy += GRAVITY;
        if self.vy > MAX_FALL { self.vy = MAX_FALL; }

        let nx = self.x + self.vx;
        if !self.collides(nx, self.y, level) { self.x = nx; } else { self.vx = 0.0; }

        let ny = self.y + self.vy;
        if !self.collides(self.x, ny, level) {
            self.y = ny; self.on_ground = false;
        } else {
            if self.vy > 0.0 {
                self.on_ground = true;
                self.y = ((self.y + PH) / TILE_SIZE).ceil() * TILE_SIZE - PH;
            }
            self.vy = 0.0;
        }

        let mx = (level.width as f32 * TILE_SIZE) - PW;
        let my = (level.height as f32 * TILE_SIZE) - PH;
        self.x = self.x.clamp(0.0, mx);
        self.y = self.y.clamp(0.0, my);

        self.anim_timer += dt;
        if self.anim_timer > 0.12 {
            self.anim_timer = 0.0;
            if self.vx.abs() > 0.1 { self.anim_frame = (self.anim_frame + 1) % 4; }
            else { self.anim_frame = 0; }
        }
        place_bomb
    }

    fn collides(&self, x: f32, y: f32, level: &Level) -> bool {
        let l = (x / TILE_SIZE) as i32;
        let r = ((x + PW - 1.0) / TILE_SIZE) as i32;
        let t = (y / TILE_SIZE) as i32;
        let b = ((y + PH - 1.0) / TILE_SIZE) as i32;
        for ty in t..=b {
            for tx in l..=r {
                if tx < 0 || ty < 0 { return true; }
                if level.is_solid(tx as usize, ty as usize) { return true; }
            }
        }
        false
    }

    pub fn take_damage(&mut self, amount: f32) {
        if self.invincible_timer > 0.0 { return; }
        self.energy -= amount;
        if self.energy <= 0.0 { self.energy = 0.0; self.die(); }
    }
    pub fn die(&mut self) { self.alive = false; self.lives -= 1; self.respawn_timer = 3.0; }
    pub fn respawn(&mut self, x: f32, y: f32) {
        self.alive = true; self.energy = self.max_energy;
        self.x = x; self.y = y; self.vx = 0.0; self.vy = 0.0;
        self.invincible_timer = 2.0;
    }
    pub fn sprite_index(&self) -> usize {
        let base = if self.player_idx == 0 { 0 } else { 20 };
        if !self.on_ground { base + 8 }
        else if self.vx.abs() > 0.1 { base + self.anim_frame }
        else { base }
    }
}
