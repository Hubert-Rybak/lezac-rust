use crate::input::PlayerInput;
use crate::level::{Level, TILE_CEIL_MAX, TILE_PIXELS, TILE_SOLID_MAX};

const TILE_SIZE: f32 = 8.0;
/// Physics constants match GAME_SPEC §13. Internally we keep float pixels per
/// 70-Hz frame; values are derived from the original 8.8 fixed-point ints.
const GRAVITY: f32 = 0x40 as f32 / 256.0; // 0.25 px/frame²
const JUMP_VEL: f32 = -(0x350 as f32) / 256.0; // -3.3125 px/frame
const MAX_FALL: f32 = 0x7FF as f32 / 256.0; // ~8 px/frame
const LAND_BOUNCE_MIN: f32 = 0x641 as f32 / 256.0; // original landing bounce cutoff
const MOVE_ACCEL: f32 = 0x40 as f32 / 256.0; // 0.25 px/frame²
const MAX_MOVE: f32 = 0x400 as f32 / 256.0; // 4 px/frame
const FRICTION: f32 = 0x30 as f32 / 256.0; // ~0.19 px/frame²
const PW: f32 = 12.0;
const PH: f32 = 16.0;
const BOMB_ARM_TIME: f32 = 4.0 / 70.0;
const INVINCIBILITY_BONUS_FRAMES: u16 = 0x2e;
const RESPAWN_INVINCIBILITY_FRAMES: u16 = 2 * 70;
const DEATH_ANIMATION_TIME: f32 = 18.0 / 70.0;
/// Special tile values (per GAME_SPEC §3.2 / §11.1).
const TILE_DROP_THROUGH: u8 = 0x27;
const TILE_TELEPORT: u8 = 0x45;

fn original_capped_byte_add(current: u8, add: i8, cap: u8) -> u8 {
    let value = current.wrapping_add(add as u8);
    if cap < value {
        cap
    } else {
        value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OriginalVitalityEffect {
    Set(u8),
    AddCapped { amount: u8, cap: u8 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalBombBoxEffect {
    pub basic_bomb_timer: u8,
    pub medium_roll_mod: u16,
    pub medium_add_base: u8,
    pub large_roll_mod: u16,
    pub large_add_base: u8,
    pub super_roll_mod: Option<u16>,
    pub super_add_base: u8,
    pub marks_inventory_dirty: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalPickupEffect {
    pub vitality: Option<OriginalVitalityEffect>,
    pub invincibility_frames: Option<u16>,
    pub bomb_box: Option<OriginalBombBoxEffect>,
    pub sound_offset: u16,
    pub sound_priority: u8,
}

pub fn original_pickup_effect(effect_id: u8) -> Option<OriginalPickupEffect> {
    let mut effect = OriginalPickupEffect {
        vitality: None,
        invincibility_frames: None,
        bomb_box: None,
        sound_offset: 8,
        sound_priority: 5,
    };

    match effect_id {
        2 => effect.vitality = Some(OriginalVitalityEffect::Set(100)),
        3 => {
            effect.vitality = Some(OriginalVitalityEffect::AddCapped {
                amount: 0x21,
                cap: 100,
            })
        }
        4 => effect.invincibility_frames = Some(INVINCIBILITY_BONUS_FRAMES),
        5 => {
            effect.bomb_box = Some(OriginalBombBoxEffect {
                basic_bomb_timer: 200,
                medium_roll_mod: 10,
                medium_add_base: 1,
                large_roll_mod: 4,
                large_add_base: 1,
                super_roll_mod: None,
                super_add_base: 0,
                marks_inventory_dirty: true,
            })
        }
        6 => {
            effect.bomb_box = Some(OriginalBombBoxEffect {
                basic_bomb_timer: 200,
                medium_roll_mod: 0x0d,
                medium_add_base: 1,
                large_roll_mod: 5,
                large_add_base: 2,
                super_roll_mod: Some(2),
                super_add_base: 1,
                marks_inventory_dirty: true,
            })
        }
        _ => return None,
    }

    Some(effect)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BombType {
    Small = 0,
    Medium = 1,
    Large = 2,
    Super = 3,
}
impl BombType {
    pub fn radius(&self) -> f32 {
        match self {
            Self::Small => 16.0,
            Self::Medium => 24.0,
            Self::Large => 32.0,
            Self::Super => 48.0,
        }
    }
    pub fn damage(&self) -> f32 {
        match self {
            Self::Small => 5.0,
            Self::Medium => 10.0,
            Self::Large => 20.0,
            Self::Super => 35.0,
        }
    }
    pub fn fuse_time(&self) -> f32 {
        match self {
            Self::Small => 2.0,
            Self::Medium => 2.5,
            Self::Large => 3.0,
            Self::Super => 3.5,
        }
    }
    pub fn next(&self) -> Self {
        match self {
            Self::Small => Self::Medium,
            Self::Medium => Self::Large,
            Self::Large => Self::Super,
            Self::Super => Self::Small,
        }
    }
    pub fn sprite_index(&self) -> usize {
        58 + *self as usize
    }
}

#[derive(Clone)]
pub struct Bomb {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub bomb_type: BombType,
    pub timer: f32,
    pub owner: usize,
    pub arm_timer: f32,
    pub exploding: bool,
    pub explosion_timer: f32,
}
impl Bomb {
    pub fn new(x: f32, y: f32, bt: BombType, owner: usize) -> Self {
        Self::new_with_velocity(x, y, bt, owner, 0.0, 0.0)
    }

    pub fn new_with_velocity(x: f32, y: f32, bt: BombType, owner: usize, vx: f32, vy: f32) -> Self {
        Bomb {
            x,
            y,
            vx,
            vy,
            bomb_type: bt,
            timer: bt.fuse_time(),
            owner,
            arm_timer: BOMB_ARM_TIME,
            exploding: false,
            explosion_timer: 0.0,
        }
    }

    pub fn update_arming_motion(&mut self, level: &Level, dt: f32) {
        if self.exploding || self.arm_timer <= f32::EPSILON {
            self.arm_timer = 0.0;
            return;
        }
        self.arm_timer = (self.arm_timer - dt).max(0.0);
        if self.arm_timer <= f32::EPSILON {
            self.arm_timer = 0.0;
        }
        let nx = self.x + self.vx;
        let ny = self.y + self.vy;
        let tx = ((nx + 4.0) / TILE_SIZE) as i32;
        let ty = ((ny + 4.0) / TILE_SIZE) as i32;
        if tx >= 0 && ty >= 0 && !level.is_solid(tx as usize, ty as usize) {
            self.x = nx;
            self.y = ny;
        } else {
            self.vx = 0.0;
            self.vy = 0.0;
        }
    }
}

#[derive(Clone)]
pub struct Debris {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: u8,
    pub life: f32,
}

#[derive(Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub on_ground: bool,
    pub facing_right: bool,
    pub energy: f32,
    pub max_energy: f32,
    pub lives: i32,
    pub score: u32,
    pub bombs: [i32; 4],
    pub current_bomb: BombType,
    pub has_super_bombs: bool,
    pub bonuses_collected: u32,
    pub alive: bool,
    pub respawn_timer: f32,
    pub anim_frame: usize,
    pub anim_timer: f32,
    pub invincible_frames: u16,
    pub crouching: bool,
    pub player_idx: usize,
    /// Counts down a few frames during which gravity collisions are skipped
    /// so the player can drop through 0x27 platforms.
    pub drop_through_timer: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamageOutcome {
    None,
    Hurt,
    Die,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlayerUpdateResult {
    pub placed_bomb: bool,
    pub teleported: bool,
    pub platform_action_ref: Option<u16>,
}

impl Player {
    pub fn new(x: f32, y: f32, idx: usize) -> Self {
        Player {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            on_ground: false,
            facing_right: true,
            energy: 100.0,
            max_energy: 100.0,
            lives: 3,
            score: 0,
            bombs: [99, 0, 0, 0],
            current_bomb: BombType::Small,
            has_super_bombs: false,
            bonuses_collected: 0,
            alive: true,
            respawn_timer: 0.0,
            anim_frame: 0,
            anim_timer: 0.0,
            invincible_frames: 0,
            crouching: false,
            player_idx: idx,
            drop_through_timer: 0,
        }
    }

    pub fn update(&mut self, input: &PlayerInput, level: &Level, dt: f32) -> PlayerUpdateResult {
        if !self.alive {
            self.respawn_timer -= dt;
            return PlayerUpdateResult::default();
        }
        if self.invincible_frames > 0 {
            self.invincible_frames -= 1;
        }
        let mut result = PlayerUpdateResult::default();
        let mut wants_crouch = false;

        if input.change_weapon() {
            self.current_bomb = self.current_bomb.next();
            if self.current_bomb == BombType::Super && !self.has_super_bombs {
                self.current_bomb = BombType::Small;
            }
            // Treat weapon change as no-input so friction still decelerates.
            if self.vx > FRICTION {
                self.vx -= FRICTION;
            } else if self.vx < -FRICTION {
                self.vx += FRICTION;
            } else {
                self.vx = 0.0;
            }
        } else {
            // Acceleration / friction model (GAME_SPEC §3.2).
            if input.left {
                self.vx -= MOVE_ACCEL;
                self.facing_right = false;
            } else if input.right {
                self.vx += MOVE_ACCEL;
                self.facing_right = true;
            } else if self.vx > FRICTION {
                self.vx -= FRICTION;
            } else if self.vx < -FRICTION {
                self.vx += FRICTION;
            } else {
                self.vx = 0.0;
            }
        }
        self.vx = self.vx.clamp(-MAX_MOVE, MAX_MOVE);

        if input.jump && self.on_ground {
            self.vy = JUMP_VEL;
            self.on_ground = false;
        }

        // Drop-through and teleport on down-key (per §3.2 / §11.1).
        if input.down && self.on_ground {
            wants_crouch = true;
            let cx = ((self.x + PW * 0.5) / TILE_SIZE) as i32;
            let cy = ((self.y + PH) / TILE_SIZE) as i32;
            if cx >= 0 && cy >= 0 {
                let below = level.tile_at(cx as usize, cy as usize);
                if below == TILE_DROP_THROUGH {
                    wants_crouch = false;
                    let action_ref = level.attr_at(cx as usize, cy as usize) & 0x7fff;
                    if action_ref != 0 {
                        result.platform_action_ref = Some(action_ref);
                    }
                    self.drop_through_timer = 6;
                    self.on_ground = false;
                    self.y += 1.0;
                } else if below == TILE_TELEPORT {
                    if let Some((tx, ty)) = find_teleport_target_px(level, cx as usize, cy as usize)
                    {
                        wants_crouch = false;
                        self.x = tx as f32;
                        self.y = ty as f32;
                        self.vx = 0.0;
                        self.vy = 0.0;
                        result.teleported = true;
                    }
                }
            }
        }

        if input.fire {
            let bi = self.current_bomb as usize;
            if self.bombs[bi] > 0 {
                self.bombs[bi] -= 1;
                result.placed_bomb = true;
            }
        }

        self.vy += GRAVITY;
        if self.vy > MAX_FALL {
            self.vy = MAX_FALL;
        }
        if self.drop_through_timer > 0 {
            self.drop_through_timer -= 1;
        }

        // Horizontal: bounce off walls (-vx/2).
        let nx = self.x + self.vx;
        if !self.collides(nx, self.y, level) {
            self.x = nx;
        } else {
            self.vx = -self.vx * 0.5;
        }

        // Vertical: bounce a little off the ground (-vy/4); back off ceiling.
        let ny = self.y + self.vy;
        let going_down = self.vy > 0.0;
        let drop = self.drop_through_timer > 0 && going_down;
        if drop || !self.collides(self.x, ny, level) {
            self.y = ny;
            self.on_ground = false;
        } else if going_down {
            self.on_ground = true;
            self.y = ((self.y + PH) / TILE_SIZE).ceil() * TILE_SIZE - PH;
            self.vy = if self.vy < LAND_BOUNCE_MIN {
                0.0
            } else {
                -self.vy * 0.25
            };
        } else {
            self.y = ((self.y / TILE_SIZE).floor() + 1.0) * TILE_SIZE;
            self.vy = 0.0;
        }

        let mx = (level.width as f32 * TILE_SIZE) - PW;
        let my = (level.height as f32 * TILE_SIZE) - PH;
        self.x = self.x.clamp(0.0, mx);
        self.y = self.y.clamp(0.0, my);
        self.crouching = wants_crouch && self.on_ground;

        self.anim_timer += dt;
        let anim_step = ((4.0 - self.vx.abs()) * 0.04).max(0.05);
        if self.anim_timer > anim_step {
            self.anim_timer = 0.0;
            if self.vx.abs() > 0.1 {
                self.anim_frame = (self.anim_frame + 1) % 8;
            } else {
                self.anim_frame = 0;
            }
        }
        result
    }

    fn collides(&self, x: f32, y: f32, level: &Level) -> bool {
        let l = (x / TILE_SIZE) as i32;
        let r = ((x + PW - 1.0) / TILE_SIZE) as i32;
        let t = (y / TILE_SIZE) as i32;
        let b = ((y + PH - 1.0) / TILE_SIZE) as i32;
        for ty in t..=b {
            for tx in l..=r {
                if tx < 0 || ty < 0 {
                    return true;
                }
                let v = level.tile_at(tx as usize, ty as usize);
                // 0x27 / 0x45 are activated by the down-key, not by collision.
                if v == TILE_DROP_THROUGH || v == TILE_TELEPORT {
                    continue;
                }
                if v != 0 && v <= TILE_SOLID_MAX {
                    return true;
                }
                // (0x4D..0x52) ceiling band only blocks the upper half of the
                // player. Standing on top of one is allowed but heads bonk.
                if v > TILE_SOLID_MAX && v <= TILE_CEIL_MAX && ty == t {
                    return true;
                }
            }
        }
        false
    }

    pub fn take_damage(&mut self, amount: f32) -> DamageOutcome {
        if self.invincible_frames > 0 || amount <= 0.0 {
            return DamageOutcome::None;
        }
        self.energy -= amount;
        if self.energy <= 0.0 {
            self.energy = 0.0;
            self.die();
            DamageOutcome::Die
        } else {
            DamageOutcome::Hurt
        }
    }
    pub fn die(&mut self) {
        self.alive = false;
        self.lives -= 1;
        self.respawn_timer = DEATH_ANIMATION_TIME;
    }
    pub fn respawn(&mut self, x: f32, y: f32) {
        self.alive = true;
        self.energy = self.max_energy;
        self.x = x;
        self.y = y;
        self.vx = 0.0;
        self.vy = 0.0;
        self.invincible_frames = RESPAWN_INVINCIBILITY_FRAMES;
        self.crouching = false;
    }
    pub fn apply_continue_bomb_minimums(&mut self) {
        self.bombs[0] = self.bombs[0].max(100);
        self.bombs[1] = self.bombs[1].max(10);
        self.bombs[2] = self.bombs[2].max(2);
    }
    pub fn apply_yellow_bomb_box(&mut self, medium: i32, large: i32) {
        self.bombs[0] = 200;
        self.bombs[1] = i32::from(original_capped_byte_add(
            self.bombs[1] as u8,
            medium.clamp(1, 10) as i8,
            99,
        ));
        self.bombs[2] = i32::from(original_capped_byte_add(
            self.bombs[2] as u8,
            large.clamp(1, 4) as i8,
            99,
        ));
    }
    pub fn apply_green_bomb_box(&mut self, medium: i32, large: i32, super_bombs: i32) {
        self.has_super_bombs = true;
        self.bombs[0] = 200;
        self.bombs[1] = i32::from(original_capped_byte_add(
            self.bombs[1] as u8,
            medium.clamp(1, 13) as i8,
            99,
        ));
        self.bombs[2] = i32::from(original_capped_byte_add(
            self.bombs[2] as u8,
            large.clamp(2, 6) as i8,
            99,
        ));
        self.bombs[3] = i32::from(original_capped_byte_add(
            self.bombs[3] as u8,
            super_bombs.clamp(1, 2) as i8,
            99,
        ));
    }
    pub fn apply_hot_dog(&mut self) {
        self.energy = (self.energy + 33.0).min(self.max_energy);
    }
    pub fn apply_invincibility_bonus(&mut self) {
        self.invincible_frames = INVINCIBILITY_BONUS_FRAMES;
    }
    pub fn sprite_index(&self) -> usize {
        if !self.on_ground {
            return 36;
        }
        if self.crouching {
            return 37;
        }
        match (self.player_idx, self.facing_right) {
            (0, true) => self.anim_frame % 8,
            (0, false) => 8 + self.anim_frame % 8,
            (_, true) => 19 + self.anim_frame % 5,
            (_, false) => 24 + self.anim_frame % 4,
        }
    }
}

/// Resolve a 0x45 teleport tile to its destination. `FUN_1000_5999` matches
/// the tile attribute against the seven-byte entity table's `map_cell_ref`.
fn find_teleport_target_px(level: &Level, sx: usize, sy: usize) -> Option<(u16, u16)> {
    let attr_ref = level.attr_at(sx, sy) & 0x7fff;
    if attr_ref != 0 {
        for spawn in &level.bonuses {
            if spawn.map_cell_ref() == attr_ref {
                let tx = spawn.x_px();
                let ty = spawn.y_px();
                let target_tile = (tx as usize / TILE_PIXELS, ty as usize / TILE_PIXELS);
                if target_tile.0 < level.width && target_tile.1 < level.height {
                    return Some((tx, ty));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bomb_sprite_indices_match_original_object_sprites() {
        assert_eq!(BombType::Small.sprite_index(), 58);
        assert_eq!(BombType::Medium.sprite_index(), 59);
        assert_eq!(BombType::Large.sprite_index(), 60);
        assert_eq!(BombType::Super.sprite_index(), 61);
    }

    #[test]
    fn player_starts_with_original_basic_bomb_inventory() {
        let p = Player::new(0.0, 0.0, 0);
        assert_eq!(p.bombs, [99, 0, 0, 0]);
        assert_eq!(p.current_bomb, BombType::Small);
        assert!(!p.has_super_bombs);
    }

    #[test]
    fn original_capped_byte_add_matches_fun_1000_5715() {
        assert_eq!(original_capped_byte_add(10, 5, 99), 15);
        assert_eq!(original_capped_byte_add(98, 5, 99), 99);
        assert_eq!(original_capped_byte_add(250, 10, 99), 4);
    }

    #[test]
    fn continue_restores_original_minimum_bomb_counts() {
        let mut p = Player::new(0.0, 0.0, 0);
        p.bombs = [3, 1, 0, 0];

        p.apply_continue_bomb_minimums();

        assert_eq!(p.bombs, [100, 10, 2, 0]);

        p.bombs = [120, 11, 3, 4];
        p.apply_continue_bomb_minimums();
        assert_eq!(p.bombs, [120, 11, 3, 4]);
    }

    #[test]
    fn bomb_box_awards_match_original_ranges() {
        let mut yellow = Player::new(0.0, 0.0, 0);
        yellow.apply_yellow_bomb_box(20, 0);
        assert_eq!(yellow.bombs, [200, 10, 1, 0]);
        assert!(!yellow.has_super_bombs);

        let mut green = Player::new(0.0, 0.0, 0);
        green.apply_green_bomb_box(20, 0, 20);
        assert_eq!(green.bombs, [200, 13, 2, 2]);
        assert!(green.has_super_bombs);
    }

    #[test]
    fn original_pickup_effect_matches_fun_1000_6053_effect_ids() {
        assert_eq!(
            original_pickup_effect(2),
            Some(OriginalPickupEffect {
                vitality: Some(OriginalVitalityEffect::Set(100)),
                invincibility_frames: None,
                bomb_box: None,
                sound_offset: 8,
                sound_priority: 5,
            })
        );
        assert_eq!(
            original_pickup_effect(3).unwrap().vitality,
            Some(OriginalVitalityEffect::AddCapped {
                amount: 0x21,
                cap: 100,
            })
        );
        assert_eq!(
            original_pickup_effect(4).unwrap().invincibility_frames,
            Some(0x2e)
        );
        assert_eq!(
            original_pickup_effect(5).unwrap().bomb_box,
            Some(OriginalBombBoxEffect {
                basic_bomb_timer: 200,
                medium_roll_mod: 10,
                medium_add_base: 1,
                large_roll_mod: 4,
                large_add_base: 1,
                super_roll_mod: None,
                super_add_base: 0,
                marks_inventory_dirty: true,
            })
        );
        assert_eq!(
            original_pickup_effect(6).unwrap().bomb_box,
            Some(OriginalBombBoxEffect {
                basic_bomb_timer: 200,
                medium_roll_mod: 0x0d,
                medium_add_base: 1,
                large_roll_mod: 5,
                large_add_base: 2,
                super_roll_mod: Some(2),
                super_add_base: 1,
                marks_inventory_dirty: true,
            })
        );
        assert_eq!(original_pickup_effect(1), None);
    }

    #[test]
    fn hot_dog_restores_original_33_energy_points() {
        let mut p = Player::new(0.0, 0.0, 0);
        p.energy = 40.0;

        p.apply_hot_dog();
        assert_eq!(p.energy, 73.0);

        p.apply_hot_dog();
        assert_eq!(p.energy, 100.0);
    }

    #[test]
    fn invincibility_bonus_counts_exact_original_frames() {
        let level = empty_level();
        let mut p = Player::new(0.0, 0.0, 0);

        p.apply_invincibility_bonus();
        assert_eq!(p.invincible_frames, 0x2e);

        for _ in 0..0x2e {
            p.update(&PlayerInput::default(), &level, 1.0 / 70.0);
        }
        assert_eq!(p.invincible_frames, 0);

        p.invincible_frames = 140;
        p.apply_invincibility_bonus();
        assert_eq!(p.invincible_frames, 0x2e);
    }

    #[test]
    fn respawn_invincibility_is_two_seconds_at_70hz() {
        let mut p = Player::new(0.0, 0.0, 0);

        p.respawn(8.0, 16.0);

        assert_eq!(p.invincible_frames, 140);
    }

    #[test]
    fn take_damage_reports_hurt_die_or_invincible() {
        let mut p = Player::new(0.0, 0.0, 0);
        p.energy = 10.0;

        assert_eq!(p.take_damage(3.0), DamageOutcome::Hurt);
        assert!(p.alive);
        assert_eq!(p.energy, 7.0);

        assert_eq!(p.take_damage(7.0), DamageOutcome::Die);
        assert!(!p.alive);

        let mut invincible = Player::new(0.0, 0.0, 0);
        invincible.apply_invincibility_bonus();
        assert_eq!(invincible.take_damage(10.0), DamageOutcome::None);
        assert_eq!(invincible.energy, 100.0);
    }

    #[test]
    fn death_animation_timer_matches_original_18_frames() {
        let mut p = Player::new(0.0, 0.0, 0);

        p.die();

        assert_eq!(p.respawn_timer, 18.0 / 70.0);
    }

    #[test]
    fn player_sprite_indices_use_original_directional_ranges() {
        let mut p1 = Player::new(0.0, 0.0, 0);
        p1.on_ground = true;
        p1.anim_frame = 7;
        p1.facing_right = true;
        assert_eq!(p1.sprite_index(), 7);
        p1.facing_right = false;
        assert_eq!(p1.sprite_index(), 15);

        let mut p2 = Player::new(0.0, 0.0, 1);
        p2.on_ground = true;
        p2.anim_frame = 4;
        p2.facing_right = true;
        assert_eq!(p2.sprite_index(), 23);
        p2.facing_right = false;
        assert_eq!(p2.sprite_index(), 24);

        p1.on_ground = false;
        assert_eq!(p1.sprite_index(), 36);
    }

    #[test]
    fn down_key_uses_crouch_frame_when_not_activating_special_tile() {
        let mut level = empty_level();
        level.tiles[8] = 1;
        level.tiles[16] = 1;
        let mut p = Player::new(0.0, 0.0, 0);
        p.on_ground = true;

        p.update(
            &PlayerInput {
                down: true,
                ..PlayerInput::default()
            },
            &level,
            1.0 / 70.0,
        );

        assert!(p.crouching);
        assert_eq!(p.sprite_index(), 37);
    }

    #[test]
    fn down_key_special_platform_does_not_leave_player_crouching() {
        let mut level = empty_level();
        level.tiles[8] = 1;
        level.tiles[16] = TILE_DROP_THROUGH;
        level.attrs[16] = 0x4088;
        let mut p = Player::new(0.0, 0.0, 0);
        p.on_ground = true;

        let result = p.update(
            &PlayerInput {
                down: true,
                ..PlayerInput::default()
            },
            &level,
            1.0 / 70.0,
        );

        assert!(!p.crouching);
        assert!(p.drop_through_timer > 0);
        assert_eq!(result.platform_action_ref, Some(0x4088));
    }

    #[test]
    fn teleport_target_uses_seven_byte_map_cell_reference() {
        let mut level = empty_level();
        level.tiles[9] = TILE_TELEPORT;
        level.attrs[9] = 0x801f;
        level.bonuses.push(crate::level::BonusSpawn {
            raw: [0x1f, 0x00, 0x28, 0x00, 0x18, 0x00, 0],
        });

        assert_eq!(find_teleport_target_px(&level, 1, 1), Some((0x28, 0x18)));
    }

    #[test]
    fn teleport_target_ignores_seven_byte_start_flag() {
        let mut level = empty_level();
        level.tiles[9] = TILE_TELEPORT;
        level.attrs[9] = 0x801f;
        level.bonuses.push(crate::level::BonusSpawn {
            raw: [0x1f, 0x00, 0x28, 0x00, 0x18, 0x00, 2],
        });

        assert_eq!(find_teleport_target_px(&level, 1, 1), Some((0x28, 0x18)));
    }

    #[test]
    fn teleport_target_requires_original_map_cell_reference() {
        let mut level = empty_level();
        level.tiles[9] = TILE_TELEPORT;
        level.tiles[31] = TILE_TELEPORT;
        level.attrs[9] = 0;
        level.platforms.push(crate::level::PlatformSpawn {
            raw: [8, 0, 8, 0, 0x20, 0, 3, 0, 0, 0, 0, 0, 0, 0],
        });

        assert_eq!(find_teleport_target_px(&level, 1, 1), None);

        level.attrs[9] = 0x802a;
        assert_eq!(find_teleport_target_px(&level, 1, 1), None);
    }

    #[test]
    fn down_key_teleport_reports_update_event() {
        let mut level = empty_level();
        level.height = 8;
        level.tiles.resize(level.width * level.height, 0);
        level.attrs.resize(level.width * level.height, 0);
        level.orig_tiles.resize(level.width * level.height, 0);
        level.tiles[17] = TILE_TELEPORT;
        level.attrs[17] = 0x801f;
        level.bonuses.push(crate::level::BonusSpawn {
            raw: [0x1f, 0x00, 0x28, 0x00, 0x18, 0x00, 0],
        });
        let mut p = Player::new(8.0, 0.0, 0);
        p.on_ground = true;

        let result = p.update(
            &PlayerInput {
                down: true,
                ..PlayerInput::default()
            },
            &level,
            1.0 / 70.0,
        );

        assert!(result.teleported);
        assert_eq!((p.x, p.y), (40.0, 24.25));
        assert!(!result.placed_bomb);
    }

    fn empty_level() -> Level {
        Level {
            width: 8,
            height: 4,
            destruction_tile: 1,
            bonus_target: 0,
            destruction_pct: 0,
            tiles: vec![0; 32],
            attrs: vec![0; 32],
            orig_tiles: vec![0; 32],
            initial_destruction_tile_count: 0,
            scroll_x: 0,
            scroll_y: 0,
            monsters: Vec::new(),
            bonuses: Vec::new(),
            platforms: Vec::new(),
        }
    }

    fn ground_level() -> Level {
        let mut level = empty_level();
        for tx in 0..level.width {
            level.tiles[tx + level.width * 2] = 1;
        }
        level
    }

    #[test]
    fn landing_velocity_bounce_uses_original_0x641_cutoff() {
        let level = ground_level();
        let mut soft = Player::new(0.0, 0.0, 0);
        soft.vy = 0x600 as f32 / 256.0;

        soft.update(&PlayerInput::default(), &level, 1.0 / 70.0);

        assert!(soft.on_ground);
        assert_eq!(soft.y, 0.0);
        assert_eq!(soft.vy, 0.0);
        assert_eq!(soft.energy, 100.0);

        let mut hard = Player::new(0.0, 0.0, 0);
        hard.vy = 0x601 as f32 / 256.0;

        hard.update(&PlayerInput::default(), &level, 1.0 / 70.0);

        assert!(hard.on_ground);
        assert_eq!(hard.y, 0.0);
        assert_eq!(hard.vy, -(0x641 as f32 / 256.0) * 0.25);
        assert_eq!(hard.energy, 100.0);
    }

    #[test]
    fn bomb_arming_motion_inherits_initial_velocity_for_four_frames() {
        let level = empty_level();
        let mut bomb = Bomb::new_with_velocity(8.0, 8.0, BombType::Small, 0, 2.0, -1.0);

        for _ in 0..4 {
            bomb.update_arming_motion(&level, 1.0 / 70.0);
        }
        assert_eq!((bomb.x, bomb.y), (16.0, 4.0));
        assert!(bomb.arm_timer <= f32::EPSILON);

        bomb.update_arming_motion(&level, 1.0 / 70.0);
        assert_eq!((bomb.x, bomb.y), (16.0, 4.0));
    }
}
