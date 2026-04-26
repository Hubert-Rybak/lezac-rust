use crate::input::PlayerInput;
use crate::level::{Level, TILE_CEIL_MAX, TILE_SOLID_MAX};

const TILE_SIZE: f32 = 8.0;
/// Physics constants match GAME_SPEC §13. Internally we keep float pixels per
/// 70-Hz frame; values are derived from the original 8.8 fixed-point ints.
const GRAVITY: f32 = 0x40 as f32 / 256.0;          // 0.25 px/frame²
const JUMP_VEL: f32 = -(0x350 as f32) / 256.0;     // -3.3125 px/frame
const MAX_FALL: f32 = 0x7FF as f32 / 256.0;        // ~8 px/frame
const MOVE_ACCEL: f32 = 0x40 as f32 / 256.0;       // 0.25 px/frame²
const MAX_MOVE: f32 = 0x400 as f32 / 256.0;        // 4 px/frame
const FRICTION: f32 = 0x30 as f32 / 256.0;         // ~0.19 px/frame²
const PW: f32 = 12.0;
const PH: f32 = 16.0;
/// Special tile values (per GAME_SPEC §3.2 / §11.1).
const TILE_DROP_THROUGH: u8 = 0x27;
const TILE_TELEPORT: u8 = 0x45;

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
    /// Counts down a few frames during which gravity collisions are skipped
    /// so the player can drop through 0x27 platforms.
    pub drop_through_timer: u8,
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
            drop_through_timer: 0,
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
            // Acceleration / friction model (GAME_SPEC §3.2).
            if input.left { self.vx -= MOVE_ACCEL; self.facing_right = false; }
            else if input.right { self.vx += MOVE_ACCEL; self.facing_right = true; }
            else if self.vx > FRICTION { self.vx -= FRICTION; }
            else if self.vx < -FRICTION { self.vx += FRICTION; }
            else { self.vx = 0.0; }
            self.vx = self.vx.clamp(-MAX_MOVE, MAX_MOVE);
        }

        if input.jump && self.on_ground { self.vy = JUMP_VEL; self.on_ground = false; }

        // Drop-through and teleport on down-key (per §3.2 / §11.1).
        if input.down && self.on_ground {
            let cx = ((self.x + PW * 0.5) / TILE_SIZE) as i32;
            let cy = ((self.y + PH) / TILE_SIZE) as i32;
            if cx >= 0 && cy >= 0 {
                let below = level.tile_at(cx as usize, cy as usize);
                if below == TILE_DROP_THROUGH {
                    self.drop_through_timer = 6;
                    self.on_ground = false;
                    self.y += 1.0;
                } else if below == TILE_TELEPORT {
                    if let Some((tx, ty)) = find_teleport_target(level, cx as usize, cy as usize) {
                        self.x = tx as f32 * TILE_SIZE;
                        self.y = (ty as f32 * TILE_SIZE) - PH;
                        self.vx = 0.0; self.vy = 0.0;
                    }
                }
            }
        }

        if input.fire {
            let bi = self.current_bomb as usize;
            if self.bombs[bi] > 0 { self.bombs[bi] -= 1; place_bomb = true; }
        }

        self.vy += GRAVITY;
        if self.vy > MAX_FALL { self.vy = MAX_FALL; }
        if self.drop_through_timer > 0 { self.drop_through_timer -= 1; }

        // Horizontal: bounce off walls (-vx/2).
        let nx = self.x + self.vx;
        if !self.collides(nx, self.y, level) { self.x = nx; }
        else { self.vx = -self.vx * 0.5; }

        // Vertical: bounce a little off the ground (-vy/4); back off ceiling.
        let ny = self.y + self.vy;
        let going_down = self.vy > 0.0;
        let drop = self.drop_through_timer > 0 && going_down;
        if drop || !self.collides(self.x, ny, level) {
            self.y = ny; self.on_ground = false;
        } else if going_down {
            self.on_ground = true;
            self.y = ((self.y + PH) / TILE_SIZE).ceil() * TILE_SIZE - PH;
            let bounce = -self.vy * 0.25;
            self.vy = if bounce.abs() < 0.25 { 0.0 } else { bounce };
        } else {
            self.y = ((self.y / TILE_SIZE).floor() + 1.0) * TILE_SIZE;
            self.vy = 0.0;
        }

        let mx = (level.width as f32 * TILE_SIZE) - PW;
        let my = (level.height as f32 * TILE_SIZE) - PH;
        self.x = self.x.clamp(0.0, mx);
        self.y = self.y.clamp(0.0, my);

        self.anim_timer += dt;
        let anim_step = ((4.0 - self.vx.abs()) * 0.04).max(0.05);
        if self.anim_timer > anim_step {
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
                let v = level.tile_at(tx as usize, ty as usize);
                // 0x27 / 0x45 are activated by the down-key, not by collision.
                if v == TILE_DROP_THROUGH || v == TILE_TELEPORT { continue; }
                if v != 0 && v <= TILE_SOLID_MAX { return true; }
                // (0x4D..0x52) ceiling-only band only blocks the head.
                if v > TILE_SOLID_MAX && v <= TILE_CEIL_MAX && ty < b { return true; }
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

/// Pair-less teleporters: jump to the next 0x45 tile we find. The original
/// game wires pairs through the platform table; without that information,
/// "next match" still produces functional warping for level testing.
fn find_teleport_target(level: &Level, sx: usize, sy: usize) -> Option<(usize, usize)> {
    let total = level.width * level.height;
    let start = sy * level.width + sx;
    for i in 1..total {
        let idx = (start + i) % total;
        if level.tiles[idx] == TILE_TELEPORT {
            return Some((idx % level.width, idx / level.width));
        }
    }
    None
}
