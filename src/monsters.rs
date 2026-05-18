use crate::level::{
    Level, MonsterAnimationSeed, MonsterMotionAccumulator, MonsterMotionRuntimeFields,
    MonsterMovementSeed, MonsterTemplate, TILE_CEIL_MAX, TILE_SOLID_MAX,
};
use crate::original_rng::OriginalRng;
use crate::player::Player;

const TILE_SIZE: f32 = 8.0;
/// FUN_1000_5102 applies monster vertical velocity as a signed byte accumulator:
/// `local_e += 4` until it reaches 0x7B. Converted through the original
/// tile/subpixel movement, that is 0.25 px/frame with ~7.7 px/frame terminal speed.
const MONSTER_GRAVITY: f32 = 0x04 as f32 / 16.0;
const MONSTER_MAX_FALL: f32 = 0x7b as f32 / 16.0;
/// Falling bonus entities use the normal 8.8 fixed-point gravity branch in
/// FUN_1000_6053 (`local_10 += 0x40`, capped at `0x7FF`).
const POWERUP_GRAVITY_WORD: i16 = 0x40;
const POWERUP_MAX_FALL_WORD: i16 = 0x7ff;
const POWERUP_GRAVITY: f32 = POWERUP_GRAVITY_WORD as f32 / 256.0;
const POWERUP_MAX_FALL: f32 = POWERUP_MAX_FALL_WORD as f32 / 256.0;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct State6CollisionFlags {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct State6RandomImpulse {
    play_sound: bool,
    x_velocity_word: i16,
    y_velocity_word: Option<i16>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OriginalDeathCountdownResult {
    Inactive,
    Active,
    Cleaned { effect_count: u8 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalDropBranchRebirth {
    pub bonus_id: u8,
    pub object_id: u8,
    pub selector_id: u8,
    pub countdown: u8,
    pub clears_animation_mode: bool,
    pub y_velocity_word_delta: i16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OriginalFourTileDamageScan {
    last_u_tile_index: Option<usize>,
    damage_delta: i8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OriginalLowObjectDamageOutcome {
    VitalityUpdated {
        vitality_byte_0x24: u8,
    },
    DeathTransition {
        object_id: u8,
        state: u8,
        countdown: u8,
        clears_animation_mode: bool,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OriginalLowObjectDamageResponse {
    selector_id: u8,
    animation_subcounter_0x19: u8,
    outcome: OriginalLowObjectDamageOutcome,
}

fn state6_scan_collision_flags(
    level: &Level,
    x_px: i16,
    y_px: i16,
    size_tiles: (u8, u8),
) -> State6CollisionFlags {
    let (width, height) = size_tiles;
    let x_tile = (x_px >> 3).max(0) as usize;
    let y_tile = (y_px >> 3).max(0) as usize;
    let width = width as usize;
    let height = height as usize;
    let mut flags = State6CollisionFlags::default();

    for dx in 0..width {
        let top = if y_tile == 0 {
            1
        } else {
            level.tile_at(x_tile + dx, y_tile - 1)
        };
        if top != 0 && top <= TILE_SOLID_MAX {
            flags.top = true;
        }

        let bottom = level.tile_at(x_tile + dx, y_tile + height);
        if bottom != 0 && bottom <= TILE_CEIL_MAX {
            flags.bottom = true;
        }
    }

    for dy in 0..height {
        let left = if x_tile == 0 {
            1
        } else {
            level.tile_at(x_tile - 1, y_tile + dy)
        };
        if left != 0 && left <= TILE_SOLID_MAX {
            flags.left = true;
        }

        let right = level.tile_at(x_tile + width, y_tile + dy);
        if right != 0 && right <= TILE_SOLID_MAX {
            flags.right = true;
        }
    }

    flags
}

fn state6_apply_collision_velocity(
    flags: State6CollisionFlags,
    x_velocity_word: i16,
    y_velocity_word: i16,
) -> (i16, i16) {
    let mut x_velocity_word = x_velocity_word;
    let mut y_velocity_word = y_velocity_word;

    if !flags.bottom {
        y_velocity_word = y_velocity_word.saturating_add(POWERUP_GRAVITY_WORD);
    }
    if (flags.top && y_velocity_word < 0) || (flags.bottom && y_velocity_word > 0) {
        y_velocity_word = -(y_velocity_word / 2);
    }
    if (flags.left && x_velocity_word < 0) || (flags.right && x_velocity_word > 0) {
        x_velocity_word = -(x_velocity_word / 2);
    }

    (x_velocity_word, y_velocity_word)
}

fn original_ground_friction_velocity_word(x_velocity_word: i16) -> i16 {
    if x_velocity_word.abs() < 0x2b {
        0
    } else if x_velocity_word < 0 {
        x_velocity_word + 0x2a
    } else {
        x_velocity_word - 0x2a
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OriginalState2MotionResponse {
    x_velocity_word: i16,
    y_velocity_word: i16,
    y_position_word: i16,
}

fn original_state2_motion_response(
    grounded: bool,
    x_velocity_word: i16,
    y_velocity_word: i16,
    y_position_word: i16,
) -> OriginalState2MotionResponse {
    let mut x_velocity_word = x_velocity_word;
    let mut y_velocity_word = y_velocity_word;
    let mut y_position_word = y_position_word;

    if grounded && -1 < y_velocity_word {
        if 0 < y_velocity_word {
            y_velocity_word = 0;
            y_position_word &= !0x0007;
        }
    } else {
        y_velocity_word = (y_velocity_word + POWERUP_GRAVITY_WORD).min(POWERUP_MAX_FALL_WORD);
    }

    if grounded {
        x_velocity_word = original_ground_friction_velocity_word(x_velocity_word);
    }

    OriginalState2MotionResponse {
        x_velocity_word,
        y_velocity_word,
        y_position_word,
    }
}

fn original_four_tile_damage_scan(
    tiles_in_original_order: [u8; 4],
    solid_threshold_gate: i16,
    solid_damage_unit: i8,
) -> OriginalFourTileDamageScan {
    let mut scan = OriginalFourTileDamageScan {
        last_u_tile_index: None,
        damage_delta: 0,
    };

    for (idx, tile) in tiles_in_original_order.into_iter().enumerate() {
        let original_loop_value = 4 - idx as i16;
        if tile == b'u' {
            scan.last_u_tile_index = Some(idx);
            scan.damage_delta = scan.damage_delta.wrapping_sub(2);
        }
        if solid_threshold_gate < original_loop_value && tile != 0 && tile < 0x4d {
            scan.damage_delta = scan.damage_delta.wrapping_sub(solid_damage_unit);
        }
    }

    scan
}

fn original_low_object_damage_response(
    object_id: u8,
    y_velocity_word: i16,
    vitality_byte_0x24: u8,
    animation_source_byte_0x1a: u8,
    damage_delta_0x661e: i8,
    selector_bytes_0x77: &[u8],
) -> Option<OriginalLowObjectDamageResponse> {
    if object_id == 0 || object_id >= 9 || damage_delta_0x661e == 0 {
        return None;
    }

    let vertical_selector_slot = if y_velocity_word < 1 { 1 } else { 2 };
    let selector_index = object_id as usize * 2 + vertical_selector_slot;
    let selector_id = *selector_bytes_0x77.get(selector_index)?;
    let animation_subcounter_0x19 = animation_source_byte_0x1a.wrapping_sub(4);
    let signed_vitality = vitality_byte_0x24 as i16 + damage_delta_0x661e as i16;
    let outcome = if signed_vitality < 0 {
        OriginalLowObjectDamageOutcome::DeathTransition {
            object_id: 0x0c,
            state: 2,
            countdown: 0x19,
            clears_animation_mode: true,
        }
    } else {
        OriginalLowObjectDamageOutcome::VitalityUpdated {
            vitality_byte_0x24: signed_vitality as u8,
        }
    };

    Some(OriginalLowObjectDamageResponse {
        selector_id,
        animation_subcounter_0x19,
        outcome,
    })
}

fn state6_scan_damage_tile_count(level: &Level, x_px: i16, y_px: i16, size_tiles: (u8, u8)) -> u8 {
    let (width, height) = size_tiles;
    let x_tile = (x_px >> 3).max(0) as usize;
    let y_tile = (y_px >> 3).max(0) as usize;
    let mut count = 0u8;

    for dy in 0..height as usize {
        for dx in (0..width as usize).step_by(2) {
            if level.tile_at(x_tile + dx, y_tile + dy) == b'u' {
                count = count.saturating_add(1);
            }
        }
    }

    count
}

fn state6_apply_damage_count(
    removal_counter: u8,
    damage_budget: u8,
    damage_count: u8,
) -> (u8, u8, bool) {
    let removal_counter = if damage_budget < damage_count {
        removal_counter.wrapping_sub(1)
    } else {
        removal_counter
    };
    let damage_budget = damage_budget.wrapping_sub(damage_count);
    let removed = removal_counter == 0xff;

    (removal_counter, damage_budget, removed)
}

fn state6_random_impulse(
    frame_counter: u32,
    negative_x_direction: bool,
    bottom_blocked: bool,
    original_rng: &mut OriginalRng,
) -> Option<State6RandomImpulse> {
    if !frame_counter.is_multiple_of(0x1d) {
        return None;
    }

    let play_sound = original_rng.gen_mod(100) > 0x46 && frame_counter.is_multiple_of(2);
    let x_roll = original_rng.gen_mod(800) as i16;
    let x_velocity_word = if negative_x_direction {
        -150 - x_roll
    } else {
        150 + x_roll
    };
    let y_velocity_word = if bottom_blocked {
        Some(-300 - original_rng.gen_mod(0x05dc) as i16)
    } else {
        None
    };

    Some(State6RandomImpulse {
        play_sound,
        x_velocity_word,
        y_velocity_word,
    })
}

fn state6_nearest_player_x_delta(players: &[Player], object_x_px: i16, object_y_px: i16) -> i16 {
    players
        .iter()
        .filter(|player| player.alive)
        .map(|player| {
            let dx = player.x as i16 - object_x_px;
            let dy = player.y as i16 - object_y_px;
            (dx, i32::from(dx).abs() + i32::from(dy).abs())
        })
        .min_by_key(|(_, distance)| *distance)
        .map_or(6000, |(dx, _)| dx)
}

fn original_axis_step(position_px: i16, fraction: u8, velocity_word: i16) -> (i16, u8) {
    let low = velocity_word as u8;
    let high = velocity_word >> 8;
    let (fraction, carry) = fraction.overflowing_add(low);
    (position_px + high + i16::from(carry), fraction)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalMotionState {
    pub x_px: i16,
    pub y_px: i16,
    pub x_fraction: u8,
    pub y_fraction: u8,
    pub x_velocity_word: i16,
    pub y_velocity_word: i16,
}

impl OriginalMotionState {
    fn new(x: f32, y: f32, seed: MonsterMovementSeed) -> Self {
        Self {
            x_px: x as i16,
            y_px: y as i16,
            x_fraction: seed.x_fraction_word as u8,
            y_fraction: seed.y_fraction_word as u8,
            x_velocity_word: seed.x_velocity_word,
            y_velocity_word: seed.y_velocity_word,
        }
    }

    fn advance(&mut self) {
        (self.x_px, self.x_fraction) =
            original_axis_step(self.x_px, self.x_fraction, self.x_velocity_word);
        (self.y_px, self.y_fraction) =
            original_axis_step(self.y_px, self.y_fraction, self.y_velocity_word);
    }

    fn apply_falling_object_gravity(&mut self) {
        self.y_velocity_word =
            (self.y_velocity_word + POWERUP_GRAVITY_WORD).min(POWERUP_MAX_FALL_WORD);
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MonsterType {
    Walker,
    Chaser,
    Floater,
    Jumper,
}
impl MonsterType {
    pub fn from_id(id: u8) -> Self {
        match id % 4 {
            0 => Self::Walker,
            1 => Self::Chaser,
            2 => Self::Floater,
            _ => Self::Jumper,
        }
    }
    pub fn speed(&self) -> f32 {
        match self {
            Self::Walker => 0.5,
            Self::Chaser => 0.7,
            Self::Floater => 0.4,
            Self::Jumper => 0.6,
        }
    }
    pub fn damage(&self) -> f32 {
        match self {
            Self::Walker => 0.5,
            Self::Chaser => 0.8,
            Self::Floater => 0.3,
            Self::Jumper => 0.6,
        }
    }
    pub fn health(&self) -> f32 {
        match self {
            Self::Walker => 20.0,
            Self::Chaser => 30.0,
            Self::Floater => 15.0,
            Self::Jumper => 25.0,
        }
    }
    pub fn sprite_base(&self) -> usize {
        match self {
            Self::Walker => 10,
            Self::Chaser => 20,
            Self::Floater => 30,
            Self::Jumper => 40,
        }
    }
}

#[derive(Clone)]
pub struct Monster {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub monster_type: MonsterType,
    pub health: f32,
    pub alive: bool,
    pub facing_right: bool,
    pub anim_frame: usize,
    pub anim_frame_count: usize,
    pub anim_timer: f32,
    pub patrol_timer: f32,
    pub start_x: f32,
    pub start_y: f32,
    /// Original GRAN.MST object/entity id for this spawned monster template.
    pub object_id: u8,
    /// Original GRAN.MST byte 0x15 state selector used by `FUN_1000_6053`.
    pub original_state: u8,
    /// Signed fixed-record byte `0x14`, subtracted before original state
    /// dispatch and added back before runtime position writeback.
    pub original_position_origin_offset: i8,
    /// State-6 (`FUN_1000_5cb0`) tile rectangle size, decoded from GRAN.MST
    /// bytes `0x0e..0x0f` when applicable.
    pub state6_collision_size_tiles: Option<(u8, u8)>,
    /// State-6 damage budget byte at GRAN.MST offset `0x24`.
    pub state6_damage_budget: Option<u8>,
    /// Original fixed-record byte `0x24`, a mutable vitality/damage-budget byte.
    pub original_vitality_byte: u8,
    /// Original fixed-record byte `0x02`, a mutable countdown/removal counter.
    pub original_countdown_byte: u8,
    /// State-6 removal/death counter byte at GRAN.MST offset `0x02`.
    pub state6_removal_counter: Option<u8>,
    /// Original countdown byte set by `FUN_1000_5bcc` after state-6 removal.
    pub original_death_timer: u8,
    /// Original animation/state byte at offset `0x1b`; `FUN_1000_5bcc` clears it.
    pub original_animation_mode: u8,
    /// Original GRAN.MST animation bytes at fixed-record offsets `0x16..0x1c`.
    pub original_animation_seed: MonsterAnimationSeed,
    /// Backup animation bytes at fixed-record offsets `0x1d..0x23`.
    pub original_animation_backup_block: [u8; 7],
    /// Word at fixed-record offset `0x12`, used to wake dependent object-id
    /// `0x1f` entities when this state-6 object enters its death transition.
    pub state6_death_wakeup_key: u16,
    /// Byte at fixed-record offset `0x25`, compared with a state-6 death wakeup key.
    pub dependent_death_key: u8,
    /// Fixed-record byte 1, used by the original as an X/Y offset-table index.
    pub anchor_table_index: u8,
    /// Original X/Y offset pair from the GRAN.MST trailing table.
    pub anchor_offset: Option<(i16, i16)>,
    /// Full original X/Y offset table used by `FUN_1000_432a` motion records.
    pub motion_anchor_offsets: Vec<(i16, i16)>,
    /// Fixed-template motion ids passed to `FUN_1000_5872` by the original.
    pub motion_sequence_ids: [u8; 3],
    /// Decoded motion records resolved from `motion_sequence_ids`.
    pub motion_sequence_fields: [Option<MonsterMotionRuntimeFields>; 3],
    /// Per-sequence reverse flags from high-bit encoded original motion ids.
    pub motion_sequence_reverse: [bool; 3],
    /// Fixed-record movement seed words loaded by the original update path.
    pub movement_seed: MonsterMovementSeed,
    /// Runtime signed 8.8 movement state mirroring the original object fields.
    pub original_motion: OriginalMotionState,
    /// Accumulator used by the original `FUN_1000_5872` motion sequence helper.
    pub motion_accumulator: MonsterMotionAccumulator,
    /// Sprite-sheet base index pulled from GRAN.MST (or fallback per type).
    pub sprite_base: usize,
    /// Per-frame placeholder movement speed until original motion is wired.
    pub speed: f32,
    /// Per-frame placeholder contact damage until original impact logic is wired.
    pub damage: f32,
    /// Runtime behavior flags used by the Rust bridge while the original
    /// template state fields are still being mapped. Loaded GRAN.MST templates
    /// currently leave this at zero; byte 0 is the object id.
    pub flags: u8,
}

impl Monster {
    pub fn new(
        x: f32,
        y: f32,
        mt: MonsterType,
        sprite_base: usize,
        speed: f32,
        damage: f32,
        flags: u8,
    ) -> Self {
        Monster {
            x,
            y,
            vx: speed,
            vy: 0.0,
            monster_type: mt,
            health: mt.health(),
            alive: true,
            facing_right: true,
            anim_frame: 0,
            anim_frame_count: monster_sprite_frame_count(sprite_base),
            anim_timer: 0.0,
            patrol_timer: 0.0,
            start_x: x,
            start_y: y,
            object_id: 0,
            original_state: 0,
            original_position_origin_offset: 0,
            state6_collision_size_tiles: None,
            state6_damage_budget: None,
            original_vitality_byte: 0,
            original_countdown_byte: 0,
            state6_removal_counter: None,
            original_death_timer: 0,
            original_animation_mode: 0,
            original_animation_seed: MonsterAnimationSeed::default(),
            original_animation_backup_block: [0; 7],
            state6_death_wakeup_key: 0,
            dependent_death_key: 0,
            anchor_table_index: 0,
            anchor_offset: None,
            motion_anchor_offsets: Vec::new(),
            motion_sequence_ids: [0; 3],
            motion_sequence_fields: [None; 3],
            motion_sequence_reverse: [false; 3],
            movement_seed: MonsterMovementSeed::default(),
            original_motion: OriginalMotionState::new(x, y, MonsterMovementSeed::default()),
            motion_accumulator: MonsterMotionAccumulator::default(),
            sprite_base,
            speed,
            damage,
            flags,
        }
    }
    /// True when the current bridge treats this monster as floating.
    pub fn floats(&self) -> bool {
        self.monster_type == MonsterType::Floater || self.flags & 0x01 != 0
    }

    fn apply_gravity(&mut self) {
        if !self.floats() {
            self.vy = (self.vy + MONSTER_GRAVITY).min(MONSTER_MAX_FALL);
        }
    }

    pub fn update(&mut self, level: &Level, players: &[Player], dt: f32) -> bool {
        self.update_with_original_motion_inputs(level, players, dt, &[], &[0; 128])
    }

    pub fn update_with_original_motion_inputs(
        &mut self,
        level: &Level,
        players: &[Player],
        dt: f32,
        original_motion_random_words: &[(i16, i16)],
        original_motion_trig_words: &[i16],
    ) -> bool {
        if !self.alive {
            return false;
        }
        self.advance_live_animation(dt);

        if self.uses_original_motion_update() {
            self.advance_original_motion_with_preprocess(
                original_motion_random_words,
                original_motion_trig_words,
            );
            self.facing_right = self.original_motion.x_velocity_word >= 0;
            self.clamp_to_level(level);
            return false;
        }
        if self.uses_state6_update() {
            let removed = self.advance_state6_motion_once(level);
            self.clamp_to_level(level);
            return removed;
        }

        self.update_placeholder_motion(level, players, dt);
        false
    }

    pub fn update_with_preprocessed_original_motion_fields(
        &mut self,
        level: &Level,
        players: &[Player],
        dt: f32,
        preprocessed_motion_sequence_fields: [Option<MonsterMotionRuntimeFields>; 3],
    ) -> bool {
        if !self.alive {
            return false;
        }
        self.advance_live_animation(dt);

        if self.uses_original_motion_update() {
            self.motion_sequence_fields = preprocessed_motion_sequence_fields;
            self.advance_original_motion_once();
            self.facing_right = self.original_motion.x_velocity_word >= 0;
            self.clamp_to_level(level);
            return false;
        }
        if self.uses_state6_update() {
            let removed = self.advance_state6_motion_once(level);
            self.clamp_to_level(level);
            return removed;
        }

        self.update_placeholder_motion(level, players, dt);
        false
    }

    fn advance_live_animation(&mut self, dt: f32) {
        if self.object_id != 0 {
            self.original_animation_seed
                .advance_original_tick_with_mode3_backup(Some(
                    self.original_animation_backup_block,
                ));
            self.original_animation_mode = self.original_animation_seed.mode;
            return;
        }

        self.anim_timer += dt;
        if self.anim_timer > 0.15 {
            self.anim_timer = 0.0;
            self.anim_frame = (self.anim_frame + 1) % self.anim_frame_count.max(1);
        }
    }

    fn update_placeholder_motion(&mut self, level: &Level, players: &[Player], dt: f32) {
        match self.monster_type {
            MonsterType::Walker => {
                self.apply_gravity();
                let nx = self.x + self.vx;
                let tx = if self.vx > 0.0 {
                    ((nx + 15.0) / TILE_SIZE) as usize
                } else {
                    (nx / TILE_SIZE) as usize
                };
                let ty = (self.y / TILE_SIZE) as usize;
                if level.is_solid(tx, ty) || level.is_solid(tx, ty + 1) {
                    self.vx = -self.vx;
                    self.facing_right = self.vx > 0.0;
                }
                let gx = if self.vx > 0.0 {
                    ((self.x + 15.0) / TILE_SIZE) as usize
                } else {
                    (self.x / TILE_SIZE) as usize
                };
                let gy = ((self.y + 16.0) / TILE_SIZE) as usize;
                if !level.is_solid(gx, gy) && self.vy <= 0.1 {
                    self.vx = -self.vx;
                    self.facing_right = self.vx > 0.0;
                }
                self.x += self.vx;
                let ny = self.y + self.vy;
                let bty = ((ny + 15.0) / TILE_SIZE) as usize;
                let ctx = ((self.x + 8.0) / TILE_SIZE) as usize;
                if level.is_solid(ctx, bty) {
                    self.y = (bty as f32 * TILE_SIZE) - 16.0;
                    self.vy = 0.0;
                } else {
                    self.y = ny;
                }
            }
            MonsterType::Chaser => {
                let mut nd = f32::MAX;
                let mut tx = self.x;
                for p in players {
                    if p.alive {
                        let d = (p.x - self.x) * (p.x - self.x) + (p.y - self.y) * (p.y - self.y);
                        if d < nd {
                            nd = d;
                            tx = p.x;
                        }
                    }
                }
                let s = self.speed;
                if tx > self.x + 2.0 {
                    self.vx = s;
                    self.facing_right = true;
                } else if tx < self.x - 2.0 {
                    self.vx = -s;
                    self.facing_right = false;
                } else {
                    self.vx = 0.0;
                }
                self.apply_gravity();
                self.x += self.vx;
                let ny = self.y + self.vy;
                let bty = ((ny + 15.0) / TILE_SIZE) as usize;
                let ctx = ((self.x + 8.0) / TILE_SIZE) as usize;
                if level.is_solid(ctx, bty) {
                    self.y = (bty as f32 * TILE_SIZE) - 16.0;
                    self.vy = 0.0;
                } else {
                    self.y = ny;
                }
            }
            MonsterType::Floater => {
                self.patrol_timer += dt;
                self.x = self.start_x + (self.patrol_timer * 1.5).sin() * 40.0;
                self.y = self.start_y + (self.patrol_timer * 2.0).cos() * 20.0;
                self.facing_right = (self.patrol_timer * 1.5).cos() > 0.0;
            }
            MonsterType::Jumper => {
                self.patrol_timer += dt;
                self.apply_gravity();
                if self.patrol_timer > 1.5 {
                    self.patrol_timer = 0.0;
                    self.vy = -3.5;
                }
                self.x += self.vx;
                let tx = if self.vx > 0.0 {
                    ((self.x + 15.0) / TILE_SIZE) as usize
                } else {
                    (self.x / TILE_SIZE) as usize
                };
                let ty = ((self.y + 8.0) / TILE_SIZE) as usize;
                if level.is_solid(tx, ty) {
                    self.vx = -self.vx;
                    self.facing_right = self.vx > 0.0;
                }
                let ny = self.y + self.vy;
                let bty = ((ny + 15.0) / TILE_SIZE) as usize;
                let ctx = ((self.x + 8.0) / TILE_SIZE) as usize;
                if level.is_solid(ctx, bty) {
                    self.y = (bty as f32 * TILE_SIZE) - 16.0;
                    self.vy = 0.0;
                } else {
                    self.y = ny;
                }
            }
        }
        self.clamp_to_level(level);
    }

    pub fn uses_original_motion_update(&self) -> bool {
        self.object_id == 0x1f
            && self.motion_sequence_ids[0] != 0
            && self.motion_sequence_fields[0].is_some()
    }

    pub fn uses_state6_update(&self) -> bool {
        self.object_id == 0x1e
            && self.original_state == 6
            && self.state6_collision_size_tiles.is_some()
    }

    fn clamp_to_level(&mut self, level: &Level) {
        let mx = (level.width as f32 * TILE_SIZE) - 16.0;
        let my = (level.height as f32 * TILE_SIZE) - 16.0;
        self.x = self.x.clamp(0.0, mx);
        self.y = self.y.clamp(0.0, my);
        self.original_motion.x_px = self.x as i16;
        self.original_motion.y_px = self.y as i16;
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health -= amount;
        if self.health <= 0.0 {
            self.alive = false;
        }
    }

    pub fn apply_original_death_transition(&mut self, countdown: u8) {
        self.object_id = 0x0e;
        self.original_state = 2;
        self.original_death_timer = countdown;
        self.original_countdown_byte = countdown;
        self.original_animation_mode = 0;
        self.alive = false;
    }

    pub fn depends_on_state6_death_key(&self, wakeup_key: u16) -> bool {
        self.object_id == 0x1f && u16::from(self.dependent_death_key) == wakeup_key
    }

    pub fn advance_original_death_countdown(
        &mut self,
        frame_counter: u32,
    ) -> OriginalDeathCountdownResult {
        if self.object_id != 0x0e || self.original_state != 2 || self.original_death_timer == 0 {
            return OriginalDeathCountdownResult::Inactive;
        }

        let death_object_id = self.object_id;
        self.original_death_timer = self
            .original_death_timer
            .wrapping_sub((frame_counter as u8) & 1);
        self.original_countdown_byte = self.original_death_timer;
        if self.original_death_timer != 0 && self.original_death_timer != 0xff {
            return OriginalDeathCountdownResult::Active;
        }

        let effect_count = if (0x0c..0x13).contains(&death_object_id) {
            let local = death_object_id.wrapping_sub(0x0c).max(1);
            local.wrapping_add(1)
        } else {
            0
        };

        self.original_state = 5;
        self.original_motion.x_velocity_word = 0;
        self.original_motion.y_velocity_word = 0;
        self.vx = 0.0;
        self.vy = 0.0;
        self.object_id = 0;
        self.original_death_timer = 0x12;
        self.original_countdown_byte = 0x12;
        OriginalDeathCountdownResult::Cleaned { effect_count }
    }

    pub fn sprite_index(&self) -> usize {
        if self.object_id != 0 && self.original_animation_mode != 0 {
            return self.original_animation_seed.frame as usize;
        }
        self.sprite_base + self.anim_frame
    }
    pub fn collides_with_player(&self, p: &Player) -> bool {
        if !self.alive || !p.alive {
            return false;
        }
        (p.x - self.x).abs() < 10.0 && (p.y - self.y).abs() < 10.0
    }

    pub fn apply_original_motion_sequence_fields(&mut self) {
        for (field, reverse) in self
            .motion_sequence_fields
            .into_iter()
            .zip(self.motion_sequence_reverse)
        {
            let Some(field) = field else {
                continue;
            };
            if let Some((x_word, y_word)) =
                field.apply_to_accumulator(&mut self.motion_accumulator, reverse)
            {
                self.original_motion.x_velocity_word = x_word;
                self.original_motion.y_velocity_word = y_word;
            }
        }
    }

    pub fn preprocess_original_motion_sequence_fields(
        &mut self,
        anchor_offsets: &[(i16, i16)],
        random_words: &[(i16, i16)],
        trig_words: &[i16],
    ) {
        for (idx, field) in self.motion_sequence_fields.iter_mut().enumerate() {
            let Some(current) = *field else {
                continue;
            };
            *field = Some(
                if let Some((x_phase, y_phase)) =
                    current.absolute_trig_phase_indices_after_advance()
                {
                    let anchor = anchor_offsets
                        .get(current.anchor_index as usize)
                        .copied()
                        .unwrap_or_default();
                    let trig_x = trig_words
                        .get(x_phase as usize)
                        .copied()
                        .unwrap_or_default();
                    let trig_y = trig_words
                        .get(y_phase as usize)
                        .copied()
                        .unwrap_or_default();
                    current.with_absolute_preprocess(anchor, trig_x, trig_y)
                } else {
                    let (random_x, random_y) = random_words.get(idx).copied().unwrap_or_default();
                    current.with_random_preprocess(random_x, random_y)
                },
            );
        }
    }

    pub fn preprocess_original_motion_sequence_fields_with_stored_anchors(
        &mut self,
        random_words: &[(i16, i16)],
        trig_words: &[i16],
    ) {
        let anchor_offsets = self.motion_anchor_offsets.clone();
        self.preprocess_original_motion_sequence_fields(&anchor_offsets, random_words, trig_words);
    }

    pub fn advance_original_motion_once(&mut self) {
        if self.object_id == 0x1f && self.motion_sequence_ids[0] != 0 {
            self.original_countdown_byte = 0xfa;
        }
        self.apply_original_motion_sequence_fields();
        self.original_motion.advance();
        self.x = self.original_motion.x_px as f32;
        self.y = self.original_motion.y_px as f32;
    }

    pub fn advance_original_motion_with_preprocess(
        &mut self,
        random_words: &[(i16, i16)],
        trig_words: &[i16],
    ) {
        self.preprocess_original_motion_sequence_fields_with_stored_anchors(
            random_words,
            trig_words,
        );
        self.advance_original_motion_once();
    }

    pub fn advance_state6_motion_once(&mut self, level: &Level) -> bool {
        let Some(size_tiles) = self.state6_collision_size_tiles else {
            return false;
        };
        let flags = state6_scan_collision_flags(
            level,
            self.original_motion.x_px,
            self.original_motion.y_px,
            size_tiles,
        );
        let damage_count = state6_scan_damage_tile_count(
            level,
            self.original_motion.x_px,
            self.original_motion.y_px,
            size_tiles,
        );
        if damage_count != 0 {
            if let (Some(removal_counter), Some(damage_budget)) =
                (self.state6_removal_counter, self.state6_damage_budget)
            {
                let (removal_counter, damage_budget, removed) =
                    state6_apply_damage_count(removal_counter, damage_budget, damage_count);
                self.state6_removal_counter = Some(removal_counter);
                self.state6_damage_budget = Some(damage_budget);
                if removed {
                    self.apply_original_death_transition(0x3c);
                    return true;
                }
            }
        }
        let (x_velocity_word, y_velocity_word) = state6_apply_collision_velocity(
            flags,
            self.original_motion.x_velocity_word,
            self.original_motion.y_velocity_word,
        );
        self.original_motion.x_velocity_word = x_velocity_word;
        self.original_motion.y_velocity_word = y_velocity_word;
        self.original_motion.advance();
        self.x = self.original_motion.x_px as f32;
        self.y = self.original_motion.y_px as f32;
        self.facing_right = self.original_motion.x_velocity_word >= 0;
        false
    }

    pub fn apply_state6_random_impulse(
        &mut self,
        level: &Level,
        players: &[Player],
        frame_counter: u32,
        original_rng: &mut OriginalRng,
    ) -> bool {
        let Some(size_tiles) = self.state6_collision_size_tiles else {
            return false;
        };
        if !self.uses_state6_update() {
            return false;
        }
        let flags = state6_scan_collision_flags(
            level,
            self.original_motion.x_px,
            self.original_motion.y_px,
            size_tiles,
        );
        let nearest_player_x_delta = state6_nearest_player_x_delta(
            players,
            self.original_motion.x_px,
            self.original_motion.y_px,
        );
        let negative_x_direction = nearest_player_x_delta < 1;
        let Some(impulse) = state6_random_impulse(
            frame_counter,
            negative_x_direction,
            flags.bottom,
            original_rng,
        ) else {
            return false;
        };
        self.original_motion.x_velocity_word = impulse.x_velocity_word;
        if let Some(y_velocity_word) = impulse.y_velocity_word {
            self.original_motion.y_velocity_word = y_velocity_word;
        }
        impulse.play_sound
    }
}

fn monster_sprite_frame_count(sprite_base: usize) -> usize {
    match sprite_base {
        // Shipped GRAN.MST bases and consecutive same-size runs in BOMOMIMK.SPR.
        0x28 => 3, // 0x28..0x2a are 16x16; 0x2b starts a 17x10 group.
        0x2a => 1, // 0x2a is the last 16x16 frame before the 17x10 group.
        0x2b => 6, // 0x2b..0x30 are 17x10.
        0x2d => 4, // 0x2d..0x30 are 17x10.
        0x2e => 3, // 0x2e..0x30 are 17x10; 0x31 starts 16x16.
        _ => 4,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PowerupType {
    Present,
    BonusToken,
    HotDog,
    FirstAid,
    Invincibility,
    YellowBombBox,
    GreenBombBox,
    JollyCloud,
    BigDiamond,
}
impl PowerupType {
    pub fn from_drop_roll(roll: u16) -> Self {
        match roll % 8 {
            0 => Self::Present,
            1 => Self::HotDog,
            2 => Self::FirstAid,
            3 => Self::Invincibility,
            4 => Self::YellowBombBox,
            5 => Self::GreenBombBox,
            6 => Self::JollyCloud,
            _ => Self::BigDiamond,
        }
    }

    pub fn random_from_original_rng(original_rng: &mut OriginalRng) -> Self {
        Self::from_drop_roll(original_rng.gen_mod(8))
    }

    pub fn original_drop_bonus_id(roll: u8, thresholds: &[u8]) -> Option<u8> {
        if thresholds.is_empty() || roll < thresholds[0] {
            return None;
        }
        let mut idx = 1usize;
        while idx < thresholds.len() && thresholds[idx] < roll {
            idx += 1;
        }
        Some(idx.saturating_sub(1) as u8)
    }

    pub fn original_drop_branch_rebirth(
        roll: u8,
        thresholds: &[u8],
    ) -> Option<OriginalDropBranchRebirth> {
        let bonus_id = Self::original_drop_bonus_id(roll, thresholds)?;
        Some(OriginalDropBranchRebirth {
            bonus_id,
            object_id: bonus_id.wrapping_add(0x13),
            selector_id: bonus_id.wrapping_add(0x3e),
            countdown: 100,
            clears_animation_mode: true,
            y_velocity_word_delta: -200,
        })
    }

    pub fn from_original_drop_roll(roll: u8, thresholds: &[u8]) -> Option<Self> {
        Self::original_drop_bonus_id(roll, thresholds).map(Self::from_bonus_id)
    }

    /// Current bridge from tile-map bonus ids and seven-byte level entity
    /// flags. Shipped non-start seven-byte records carry flag 0, so they spawn
    /// as the baseline present/token collectible.
    pub fn from_bonus_id(b: u8) -> Self {
        match b {
            1 => Self::BonusToken,
            2 => Self::FirstAid,      // full energy
            3 => Self::HotDog,        // +33 energy
            4 => Self::Invincibility, // 46 frames invuln
            5 => Self::YellowBombBox, // random bombs
            6 => Self::GreenBombBox,  // larger bomb supply
            _ => Self::Present,
        }
    }

    pub fn original_effect_id(self) -> Option<u8> {
        match self {
            Self::FirstAid => Some(2),
            Self::HotDog => Some(3),
            Self::Invincibility => Some(4),
            Self::YellowBombBox => Some(5),
            Self::GreenBombBox => Some(6),
            _ => None,
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            Self::Present => 2000,
            Self::BonusToken => 2000,
            Self::HotDog => 1500,
            Self::FirstAid => 1000,
            Self::Invincibility => 2500,
            Self::YellowBombBox => 3000,
            Self::GreenBombBox => 1000,
            Self::JollyCloud => 2000,
            Self::BigDiamond => 5000,
        }
    }
    pub fn sprite_index(&self) -> usize {
        match self {
            Self::Present => 0x13,
            Self::BonusToken => 0x14,
            Self::FirstAid => 0x15,
            Self::HotDog => 0x16,
            Self::Invincibility => 0x17,
            Self::YellowBombBox => 0x18,
            Self::GreenBombBox => 0x19,
            Self::JollyCloud => 0x1a,
            Self::BigDiamond => 0x1b,
        }
    }
}

#[derive(Clone)]
pub struct Powerup {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub powerup_type: PowerupType,
    pub counts_for_level_goal: bool,
    pub alive: bool,
    pub timer: f32,
}
impl Powerup {
    pub fn new(x: f32, y: f32, pt: PowerupType) -> Self {
        Powerup {
            x,
            y,
            vy: -1.0,
            powerup_type: pt,
            counts_for_level_goal: false,
            alive: true,
            timer: 8.0,
        }
    }
    pub fn level_goal(mut self) -> Self {
        self.counts_for_level_goal = true;
        self
    }
    pub fn update(&mut self, level: &Level, dt: f32) {
        if !self.alive {
            return;
        }
        if !self.counts_for_level_goal {
            self.timer -= dt;
            if self.timer <= 0.0 {
                self.alive = false;
                return;
            }
        }
        self.vy = (self.vy + POWERUP_GRAVITY).min(POWERUP_MAX_FALL);
        let ny = self.y + self.vy;
        let bty = ((ny + 15.0) / TILE_SIZE) as usize;
        let ctx = ((self.x + 8.0) / TILE_SIZE) as usize;
        if level.is_solid(ctx, bty) {
            self.y = (bty as f32 * TILE_SIZE) - 16.0;
            self.vy = 0.0;
        } else {
            self.y = ny;
        }
    }
    pub fn collides_with_player(&self, p: &Player) -> bool {
        if !self.alive || !p.alive {
            return false;
        }
        self.x < p.x + 12.0 && self.x + 14.0 > p.x && self.y < p.y + 16.0 && self.y + 14.0 > p.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_level() -> Level {
        level_with_size(4, 4)
    }

    fn level_with_size(width: usize, height: usize) -> Level {
        Level {
            width,
            height,
            destruction_tile: 1,
            bonus_target: 0,
            destruction_pct: 0,
            tiles: vec![0; width * height],
            attrs: vec![0; width * height],
            orig_tiles: vec![0; width * height],
            initial_destruction_tile_count: 0,
            scroll_x: 0,
            scroll_y: 0,
            monsters: Vec::new(),
            bonuses: Vec::new(),
            platforms: Vec::new(),
        }
    }

    #[test]
    fn level_goal_powerups_do_not_expire_like_timed_drops() {
        let level = empty_level();
        let mut drop = Powerup::new(0.0, 0.0, PowerupType::Present);
        let mut goal = Powerup::new(0.0, 0.0, PowerupType::Present).level_goal();

        drop.update(&level, 9.0);
        goal.update(&level, 9.0);

        assert!(!drop.alive);
        assert!(goal.alive);
        assert_eq!(goal.timer, 8.0);
    }

    #[test]
    fn monster_gravity_matches_original_signed_byte_accumulator() {
        let mut falling = Monster::new(0.0, 0.0, MonsterType::Walker, 0, 0.0, 0.0, 0);

        falling.apply_gravity();
        assert_eq!(falling.vy, 0x04 as f32 / 16.0);

        falling.vy = MONSTER_MAX_FALL - 0.01;
        falling.apply_gravity();
        assert_eq!(falling.vy, MONSTER_MAX_FALL);

        let mut floating = Monster::new(0.0, 0.0, MonsterType::Floater, 0, 0.0, 0.0, 1);
        floating.apply_gravity();
        assert_eq!(floating.vy, 0.0);
    }

    #[test]
    fn original_axis_step_matches_signed_fraction_accumulator() {
        assert_eq!(original_axis_step(10, 0, 0x0100), (11, 0));
        assert_eq!(original_axis_step(10, 0, -0x0100), (9, 0));

        let mut pos = 10;
        let mut frac = 0;
        for _ in 0..4 {
            (pos, frac) = original_axis_step(pos, frac, 0x0040);
        }
        assert_eq!((pos, frac), (11, 0));

        assert_eq!(original_axis_step(10, 0, -1), (9, 0xff));
        assert_eq!(original_axis_step(9, 0xff, -1), (9, 0xfe));
    }

    #[test]
    fn state6_collision_scan_matches_original_rectangle_thresholds() {
        let mut level = level_with_size(12, 12);
        let x_tile = 4;
        let y_tile = 5;
        let size = (5, 4);

        level.tiles[(y_tile - 1) * level.width + x_tile + 1] = TILE_SOLID_MAX;
        level.tiles[(y_tile + size.1 as usize) * level.width + x_tile + 2] = TILE_CEIL_MAX;
        level.tiles[(y_tile + 1) * level.width + x_tile - 1] = TILE_SOLID_MAX;
        level.tiles[(y_tile + 2) * level.width + x_tile + size.0 as usize] = TILE_SOLID_MAX;

        assert_eq!(
            state6_scan_collision_flags(&level, (x_tile * 8) as i16, (y_tile * 8) as i16, size),
            State6CollisionFlags {
                top: true,
                bottom: true,
                left: true,
                right: true,
            }
        );

        level.tiles[(y_tile - 1) * level.width + x_tile + 1] = TILE_SOLID_MAX + 1;
        level.tiles[(y_tile + size.1 as usize) * level.width + x_tile + 2] = TILE_CEIL_MAX + 1;
        level.tiles[(y_tile + 1) * level.width + x_tile - 1] = TILE_SOLID_MAX + 1;
        level.tiles[(y_tile + 2) * level.width + x_tile + size.0 as usize] = TILE_SOLID_MAX + 1;

        assert_eq!(
            state6_scan_collision_flags(&level, (x_tile * 8) as i16, (y_tile * 8) as i16, size),
            State6CollisionFlags::default()
        );
    }

    #[test]
    fn state6_collision_velocity_matches_original_bounce_rules() {
        assert_eq!(
            state6_apply_collision_velocity(State6CollisionFlags::default(), 0x0100, 0),
            (0x0100, POWERUP_GRAVITY_WORD)
        );

        assert_eq!(
            state6_apply_collision_velocity(
                State6CollisionFlags {
                    top: true,
                    ..State6CollisionFlags::default()
                },
                0,
                -0x0200,
            ),
            (0, 0x00e0)
        );

        assert_eq!(
            state6_apply_collision_velocity(
                State6CollisionFlags {
                    bottom: true,
                    ..State6CollisionFlags::default()
                },
                0,
                0x0200,
            ),
            (0, -0x0100)
        );

        assert_eq!(
            state6_apply_collision_velocity(
                State6CollisionFlags {
                    left: true,
                    ..State6CollisionFlags::default()
                },
                -0x0200,
                0,
            ),
            (0x0100, POWERUP_GRAVITY_WORD)
        );
        assert_eq!(
            state6_apply_collision_velocity(
                State6CollisionFlags {
                    right: true,
                    ..State6CollisionFlags::default()
                },
                0x0200,
                0,
            ),
            (-0x0100, POWERUP_GRAVITY_WORD)
        );
    }

    #[test]
    fn original_ground_friction_matches_fun_1000_5b86_threshold_and_step() {
        assert_eq!(original_ground_friction_velocity_word(0), 0);
        assert_eq!(original_ground_friction_velocity_word(0x2a), 0);
        assert_eq!(original_ground_friction_velocity_word(-0x2a), 0);
        assert_eq!(original_ground_friction_velocity_word(0x2b), 1);
        assert_eq!(original_ground_friction_velocity_word(-0x2b), -1);
        assert_eq!(original_ground_friction_velocity_word(0x0100), 0x00d6);
        assert_eq!(original_ground_friction_velocity_word(-0x0100), -0x00d6);
    }

    #[test]
    fn original_state2_motion_response_matches_fun_1000_6053_ground_and_gravity_rules() {
        assert_eq!(
            original_state2_motion_response(true, 0x0100, 0x0040, 0x0125),
            OriginalState2MotionResponse {
                x_velocity_word: 0x00d6,
                y_velocity_word: 0,
                y_position_word: 0x0120,
            }
        );
        assert_eq!(
            original_state2_motion_response(true, 0x002a, 0, 0x0127),
            OriginalState2MotionResponse {
                x_velocity_word: 0,
                y_velocity_word: 0,
                y_position_word: 0x0127,
            }
        );
        assert_eq!(
            original_state2_motion_response(true, 0x0100, -1, 0x0127),
            OriginalState2MotionResponse {
                x_velocity_word: 0x00d6,
                y_velocity_word: 0x003f,
                y_position_word: 0x0127,
            }
        );
        assert_eq!(
            original_state2_motion_response(false, 0x0100, 0x07f0, 0x0127),
            OriginalState2MotionResponse {
                x_velocity_word: 0x0100,
                y_velocity_word: 0x07ff,
                y_position_word: 0x0127,
            }
        );
    }

    #[test]
    fn original_four_tile_damage_scan_matches_fun_1000_56b6_loop_shape() {
        assert_eq!(
            original_four_tile_damage_scan([1, 0x4c, b'u', 0], 2, 3),
            OriginalFourTileDamageScan {
                last_u_tile_index: Some(2),
                damage_delta: -8,
            }
        );
        assert_eq!(
            original_four_tile_damage_scan([1, 2, 3, 4], 4, 5),
            OriginalFourTileDamageScan {
                last_u_tile_index: None,
                damage_delta: 0,
            }
        );
        assert_eq!(
            original_four_tile_damage_scan([b'u', b'u', 0x4d, 1], 0, 1),
            OriginalFourTileDamageScan {
                last_u_tile_index: Some(1),
                damage_delta: -5,
            }
        );
    }

    #[test]
    fn original_low_object_damage_response_matches_fun_1000_6053_shape() {
        let mut selectors = [0u8; 0x90];
        selectors[5 * 2 + 1] = 0x44;
        selectors[5 * 2 + 2] = 0x45;

        assert_eq!(
            original_low_object_damage_response(5, 0, 10, 9, -3, &selectors),
            Some(OriginalLowObjectDamageResponse {
                selector_id: 0x44,
                animation_subcounter_0x19: 5,
                outcome: OriginalLowObjectDamageOutcome::VitalityUpdated {
                    vitality_byte_0x24: 7,
                },
            })
        );
        assert_eq!(
            original_low_object_damage_response(5, 1, 2, 3, -3, &selectors),
            Some(OriginalLowObjectDamageResponse {
                selector_id: 0x45,
                animation_subcounter_0x19: 0xff,
                outcome: OriginalLowObjectDamageOutcome::DeathTransition {
                    object_id: 0x0c,
                    state: 2,
                    countdown: 0x19,
                    clears_animation_mode: true,
                },
            })
        );
        assert_eq!(
            original_low_object_damage_response(9, 0, 10, 9, -3, &selectors),
            None
        );
        assert_eq!(
            original_low_object_damage_response(5, 0, 10, 9, 0, &selectors),
            None
        );
        assert_eq!(
            original_low_object_damage_response(5, 0, 10, 9, -3, &selectors[..4]),
            None
        );
    }

    #[test]
    fn state6_damage_scan_counts_u_tiles_every_other_column() {
        let mut level = level_with_size(12, 12);
        let x_tile = 4;
        let y_tile = 5;
        let size = (5, 4);

        level.tiles[y_tile * level.width + x_tile] = b'u';
        level.tiles[y_tile * level.width + x_tile + 1] = b'u';
        level.tiles[y_tile * level.width + x_tile + 2] = b'u';
        level.tiles[(y_tile + 3) * level.width + x_tile + 4] = b'u';

        assert_eq!(
            state6_scan_damage_tile_count(&level, (x_tile * 8) as i16, (y_tile * 8) as i16, size),
            3
        );
    }

    #[test]
    fn state6_damage_count_updates_budget_and_removal_counter() {
        assert_eq!(state6_apply_damage_count(3, 10, 4), (3, 6, false));
        assert_eq!(state6_apply_damage_count(3, 2, 4), (2, 0xfe, false));
        assert_eq!(state6_apply_damage_count(0, 2, 4), (0xff, 0xfe, true));
    }

    #[test]
    fn state6_random_impulse_matches_original_29_frame_rng_shape() {
        let seed = 0x1234_5678;
        let mut rng = OriginalRng::new(seed);
        assert_eq!(state6_random_impulse(28, false, true, &mut rng), None);
        assert_eq!(rng.seed(), seed);

        let mut expected = OriginalRng::new(seed);
        let sound_roll = expected.gen_mod(100);
        let x_roll = expected.gen_mod(800) as i16;
        let y_roll = expected.gen_mod(0x05dc) as i16;

        let impulse = state6_random_impulse(58, false, true, &mut rng).unwrap();
        assert_eq!(
            impulse,
            State6RandomImpulse {
                play_sound: sound_roll > 0x46,
                x_velocity_word: 150 + x_roll,
                y_velocity_word: Some(-300 - y_roll),
            }
        );
        assert_eq!(rng.seed(), expected.seed());

        let mut expected = OriginalRng::new(seed);
        let _sound_roll = expected.gen_mod(100);
        let x_roll = expected.gen_mod(800) as i16;
        let mut rng = OriginalRng::new(seed);
        let impulse = state6_random_impulse(29, true, false, &mut rng).unwrap();
        assert_eq!(
            impulse,
            State6RandomImpulse {
                play_sound: false,
                x_velocity_word: -150 - x_roll,
                y_velocity_word: None,
            }
        );
        assert_eq!(rng.seed(), expected.seed());
    }

    #[test]
    fn state6_live_random_impulse_uses_frame_counter_scan_and_rng() {
        let seed = 0x1234_5678;
        let mut level = level_with_size(12, 12);
        let x_tile = 4;
        let y_tile = 5;
        for x in x_tile..x_tile + 5 {
            level.tiles[(y_tile + 4) * level.width + x] = 1;
        }

        let mut monster = Monster::new(
            (x_tile * 8) as f32,
            (y_tile * 8) as f32,
            MonsterType::Walker,
            0x2e,
            0.0,
            0.0,
            0,
        );
        monster.object_id = 0x1e;
        monster.original_state = 6;
        monster.state6_collision_size_tiles = Some((5, 4));
        monster.original_motion = OriginalMotionState::new(
            monster.x,
            monster.y,
            MonsterMovementSeed {
                x_velocity_word: -1,
                y_velocity_word: 0,
                x_fraction_word: 0,
                y_fraction_word: 0,
            },
        );

        let mut rng = OriginalRng::new(seed);
        assert!(!monster.apply_state6_random_impulse(&level, &[], 28, &mut rng));
        assert_eq!(rng.seed(), seed);

        let mut expected = OriginalRng::new(seed);
        let sound_roll = expected.gen_mod(100);
        let x_roll = expected.gen_mod(800) as i16;
        let y_roll = expected.gen_mod(0x05dc) as i16;

        assert_eq!(
            monster.apply_state6_random_impulse(&level, &[], 58, &mut rng),
            sound_roll > 0x46
        );
        assert_eq!(monster.original_motion.x_velocity_word, 150 + x_roll);
        assert_eq!(monster.original_motion.y_velocity_word, -300 - y_roll);
        assert_eq!(rng.seed(), expected.seed());
    }

    #[test]
    fn state6_live_random_impulse_uses_nearest_player_x_delta_for_direction() {
        let seed = 0x1234_5678;
        let level = level_with_size(12, 12);
        let mut monster = Monster::new(40.0, 40.0, MonsterType::Walker, 0x2e, 0.0, 0.0, 0);
        monster.object_id = 0x1e;
        monster.original_state = 6;
        monster.state6_collision_size_tiles = Some((5, 4));
        monster.original_motion = OriginalMotionState::new(
            monster.x,
            monster.y,
            MonsterMovementSeed {
                x_velocity_word: 0x0200,
                y_velocity_word: 0,
                x_fraction_word: 0,
                y_fraction_word: 0,
            },
        );

        let mut far_left = Player::new(20.0, 40.0, 0);
        let near_right = Player::new(48.0, 40.0, 1);
        let mut rng = OriginalRng::new(seed);
        let mut expected = OriginalRng::new(seed);
        let _sound_roll = expected.gen_mod(100);
        let x_roll = expected.gen_mod(800) as i16;

        let _play_sound = monster.apply_state6_random_impulse(
            &level,
            &[far_left.clone(), near_right],
            29,
            &mut rng,
        );
        assert_eq!(monster.original_motion.x_velocity_word, 150 + x_roll);

        far_left.alive = true;
        let mut near_left_monster =
            Monster::new(40.0, 40.0, MonsterType::Walker, 0x2e, 0.0, 0.0, 0);
        near_left_monster.object_id = 0x1e;
        near_left_monster.original_state = 6;
        near_left_monster.state6_collision_size_tiles = Some((5, 4));
        near_left_monster.original_motion = OriginalMotionState::new(
            near_left_monster.x,
            near_left_monster.y,
            MonsterMovementSeed {
                x_velocity_word: 0x0200,
                y_velocity_word: 0,
                x_fraction_word: 0,
                y_fraction_word: 0,
            },
        );
        let mut rng = OriginalRng::new(seed);
        let mut expected = OriginalRng::new(seed);
        let _sound_roll = expected.gen_mod(100);
        let x_roll = expected.gen_mod(800) as i16;

        let _play_sound =
            near_left_monster.apply_state6_random_impulse(&level, &[far_left], 29, &mut rng);
        assert_eq!(
            near_left_monster.original_motion.x_velocity_word,
            -150 - x_roll
        );
    }

    #[test]
    fn state6_live_update_uses_original_scan_bounce_and_fixed_point_advance() {
        let mut level = level_with_size(12, 12);
        let mut monster = Monster::new(32.0, 40.0, MonsterType::Walker, 0x2e, 0.0, 0.0, 0);
        monster.object_id = 0x1e;
        monster.original_state = 6;
        monster.state6_collision_size_tiles = Some((5, 4));
        monster.original_motion = OriginalMotionState::new(
            32.0,
            40.0,
            MonsterMovementSeed {
                x_velocity_word: 0,
                y_velocity_word: 0,
                x_fraction_word: 0,
                y_fraction_word: 0,
            },
        );

        assert!(monster.uses_state6_update());
        for _ in 0..4 {
            monster.update(&level, &[], 1.0 / 70.0);
        }
        assert_eq!(monster.original_motion.y_px, 42);
        assert_eq!(monster.y, 42.0);
        assert_eq!(monster.original_motion.y_velocity_word, 0x0100);

        level.tiles[(9 * level.width) + 5] = TILE_CEIL_MAX;
        monster.original_motion.y_velocity_word = 0x0200;
        monster.update(&level, &[], 1.0 / 70.0);
        assert_eq!(monster.original_motion.y_velocity_word, -0x0100);
    }

    #[test]
    fn state6_live_update_applies_scanned_damage_bytes() {
        let mut level = level_with_size(12, 12);
        let x_tile = 4;
        let y_tile = 5;
        level.tiles[y_tile * level.width + x_tile] = b'u';
        level.tiles[y_tile * level.width + x_tile + 2] = b'u';
        level.tiles[y_tile * level.width + x_tile + 4] = b'u';

        let mut monster = Monster::new(32.0, 40.0, MonsterType::Walker, 0x2e, 0.0, 0.0, 0);
        monster.object_id = 0x1e;
        monster.original_state = 6;
        monster.state6_collision_size_tiles = Some((5, 4));
        monster.state6_removal_counter = Some(1);
        monster.state6_damage_budget = Some(2);
        monster.original_motion = OriginalMotionState::new(
            32.0,
            40.0,
            MonsterMovementSeed {
                x_velocity_word: 0,
                y_velocity_word: 0,
                x_fraction_word: 0,
                y_fraction_word: 0,
            },
        );

        assert!(!monster.update(&level, &[], 1.0 / 70.0));

        assert!(monster.alive);
        assert_eq!(monster.state6_removal_counter, Some(0));
        assert_eq!(monster.state6_damage_budget, Some(0xff));

        monster.state6_damage_budget = Some(2);
        assert!(monster.update(&level, &[], 1.0 / 70.0));
        assert!(!monster.alive);
        assert_eq!(monster.object_id, 0x0e);
        assert_eq!(monster.original_state, 2);
        assert_eq!(monster.original_death_timer, 0x3c);
        assert_eq!(monster.original_countdown_byte, 0x3c);
        assert_eq!(monster.original_animation_mode, 0);
    }

    #[test]
    fn original_death_transition_countdown_matches_odd_frame_decrement_and_cleanup() {
        let mut monster = Monster::new(32.0, 40.0, MonsterType::Walker, 0x2e, 0.0, 0.0, 0);
        monster.apply_original_death_transition(2);
        monster.original_motion.x_velocity_word = 0x0100;
        monster.original_motion.y_velocity_word = -0x0200;
        monster.vx = 1.0;
        monster.vy = -2.0;

        assert_eq!(
            monster.advance_original_death_countdown(2),
            OriginalDeathCountdownResult::Active
        );
        assert_eq!(monster.object_id, 0x0e);
        assert_eq!(monster.original_state, 2);
        assert_eq!(monster.original_death_timer, 2);
        assert_eq!(monster.original_countdown_byte, 2);

        assert_eq!(
            monster.advance_original_death_countdown(3),
            OriginalDeathCountdownResult::Active
        );
        assert_eq!(monster.object_id, 0x0e);
        assert_eq!(monster.original_death_timer, 1);
        assert_eq!(monster.original_countdown_byte, 1);

        assert_eq!(
            monster.advance_original_death_countdown(5),
            OriginalDeathCountdownResult::Cleaned { effect_count: 3 }
        );
        assert_eq!(monster.object_id, 0);
        assert_eq!(monster.original_state, 5);
        assert_eq!(monster.original_death_timer, 0x12);
        assert_eq!(monster.original_countdown_byte, 0x12);
        assert_eq!(monster.original_motion.x_velocity_word, 0);
        assert_eq!(monster.original_motion.y_velocity_word, 0);
        assert_eq!(monster.vx, 0.0);
        assert_eq!(monster.vy, 0.0);
        assert_eq!(
            monster.advance_original_death_countdown(7),
            OriginalDeathCountdownResult::Inactive
        );
    }

    #[test]
    fn original_motion_state_advances_both_axes() {
        let seed = MonsterMovementSeed {
            x_velocity_word: 0x0040,
            y_velocity_word: -0x0100,
            x_fraction_word: 0,
            y_fraction_word: 0,
        };
        let mut state = OriginalMotionState::new(20.0, 30.0, seed);

        for _ in 0..4 {
            state.advance();
        }

        assert_eq!(
            state,
            OriginalMotionState {
                x_px: 21,
                y_px: 26,
                x_fraction: 0,
                y_fraction: 0,
                x_velocity_word: 0x0040,
                y_velocity_word: -0x0100,
            }
        );
    }

    #[test]
    fn original_motion_state_applies_falling_object_gravity_word() {
        let mut state = OriginalMotionState::new(0.0, 0.0, MonsterMovementSeed::default());

        state.apply_falling_object_gravity();
        assert_eq!(state.y_velocity_word, 0x40);

        state.y_velocity_word = POWERUP_MAX_FALL_WORD - 1;
        state.apply_falling_object_gravity();
        assert_eq!(state.y_velocity_word, POWERUP_MAX_FALL_WORD);
    }

    #[test]
    fn monster_applies_resolved_original_motion_sequence_fields() {
        let bounded = MonsterMotionRuntimeFields {
            anchor_index: 0,
            secondary_anchor_index: 0,
            phase_step: 0,
            limit_or_sentinel: 10,
            angle_phase: 0,
            base_x_word: 0,
            base_y_word: 0,
            x_word: 12,
            y_word: -12,
            random_y_base: 0,
        };
        let absolute = MonsterMotionRuntimeFields {
            limit_or_sentinel: -1,
            x_word: -3,
            y_word: 4,
            ..bounded
        };
        let mut monster = Monster::new(0.0, 0.0, MonsterType::Walker, 0, 0.0, 0.0, 0);
        monster.motion_sequence_fields = [Some(bounded), None, None];

        monster.apply_original_motion_sequence_fields();
        assert_eq!(
            monster.motion_accumulator,
            MonsterMotionAccumulator { x: 2, y: -2 }
        );
        assert_eq!(monster.original_motion.x_velocity_word, 0);
        assert_eq!(monster.original_motion.y_velocity_word, 0);

        monster.motion_sequence_fields = [Some(absolute), None, None];
        monster.apply_original_motion_sequence_fields();
        assert_eq!(
            monster.motion_accumulator,
            MonsterMotionAccumulator::default()
        );
        assert_eq!(monster.original_motion.x_velocity_word, -3);
        assert_eq!(monster.original_motion.y_velocity_word, 4);

        monster.motion_sequence_fields = [Some(bounded), None, None];
        monster.motion_sequence_reverse = [true, false, false];
        monster.apply_original_motion_sequence_fields();
        assert_eq!(
            monster.motion_accumulator,
            MonsterMotionAccumulator { x: -2, y: 2 }
        );
    }

    #[test]
    fn monster_preprocesses_original_motion_sequences_in_place() {
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
            random_y_base: 3,
        };
        let absolute = MonsterMotionRuntimeFields {
            anchor_index: 1,
            secondary_anchor_index: 0,
            phase_step: 3,
            limit_or_sentinel: -1,
            angle_phase: 0x7e,
            base_x_word: 4,
            base_y_word: -5,
            x_word: 0,
            y_word: 0,
            random_y_base: 0,
        };
        let mut monster = Monster::new(0.0, 0.0, MonsterType::Walker, 0, 0.0, 0.0, 0);
        monster.motion_sequence_fields = [Some(bounded), Some(absolute), None];
        monster.motion_anchor_offsets = vec![(0, 0), (100, 200)];
        let mut trig = vec![0; 128];
        trig[0x21] = 7;
        trig[0x01] = -9;

        monster.preprocess_original_motion_sequence_fields_with_stored_anchors(
            &[(11, 13), (0, 0), (0, 0)],
            &trig,
        );

        assert_eq!(monster.motion_sequence_fields[0].unwrap().x_word, 11);
        assert_eq!(monster.motion_sequence_fields[0].unwrap().y_word, 16);
        assert_eq!(monster.motion_sequence_fields[1].unwrap().angle_phase, 0x01);
        assert_eq!(monster.motion_sequence_fields[1].unwrap().x_word, 111);
        assert_eq!(monster.motion_sequence_fields[1].unwrap().y_word, 186);
    }

    #[test]
    fn monster_can_advance_original_motion_once_and_sync_position() {
        let absolute = MonsterMotionRuntimeFields {
            anchor_index: 0,
            secondary_anchor_index: 0,
            phase_step: 0,
            limit_or_sentinel: -1,
            angle_phase: 0,
            base_x_word: 0,
            base_y_word: 0,
            x_word: 0x0100,
            y_word: -0x0100,
            random_y_base: 0,
        };
        let mut monster = Monster::new(10.0, 20.0, MonsterType::Walker, 0, 0.0, 0.0, 0);
        monster.motion_sequence_fields = [Some(absolute), None, None];

        monster.advance_original_motion_once();

        assert_eq!(monster.original_motion.x_px, 11);
        assert_eq!(monster.original_motion.y_px, 19);
        assert_eq!(monster.x, 11.0);
        assert_eq!(monster.y, 19.0);
    }

    #[test]
    fn monster_original_motion_tick_preprocesses_before_advancing() {
        let absolute = MonsterMotionRuntimeFields {
            anchor_index: 0,
            secondary_anchor_index: 0,
            phase_step: 1,
            limit_or_sentinel: -1,
            angle_phase: 0,
            base_x_word: 0,
            base_y_word: 0,
            x_word: 0,
            y_word: 0,
            random_y_base: 0,
        };
        let mut monster = Monster::new(0.0, 0.0, MonsterType::Walker, 0, 0.0, 0.0, 0);
        monster.motion_anchor_offsets = vec![(0x0100, -0x0100)];
        monster.motion_sequence_fields = [Some(absolute), None, None];

        monster.advance_original_motion_with_preprocess(&[], &[0; 128]);

        assert_eq!(monster.motion_sequence_fields[0].unwrap().angle_phase, 1);
        assert_eq!(monster.original_motion.x_px, 1);
        assert_eq!(monster.original_motion.y_px, -1);
        assert_eq!(monster.x, 1.0);
        assert_eq!(monster.y, -1.0);

        monster.advance_original_motion_with_preprocess(&[], &[0; 128]);

        assert_eq!(monster.motion_sequence_fields[0].unwrap().angle_phase, 2);
        assert_eq!(monster.original_motion.x_px, 2);
        assert_eq!(monster.original_motion.y_px, -2);
    }

    #[test]
    fn object_0x1f_live_update_uses_original_motion_records() {
        let absolute = MonsterMotionRuntimeFields {
            anchor_index: 0,
            secondary_anchor_index: 0,
            phase_step: 0,
            limit_or_sentinel: -1,
            angle_phase: 0,
            base_x_word: 0,
            base_y_word: 0,
            x_word: 0x0100,
            y_word: -0x0100,
            random_y_base: 0,
        };
        let mut monster = Monster::new(10.0, 20.0, MonsterType::Floater, 0, 0.0, 0.0, 0);
        monster.object_id = 0x1f;
        monster.motion_sequence_ids = [1, 0, 0];
        monster.motion_sequence_fields = [Some(absolute), None, None];
        monster.motion_anchor_offsets = vec![(0x0100, -0x0100)];
        monster.original_countdown_byte = 0x04;
        monster.original_motion =
            OriginalMotionState::new(10.0, 20.0, MonsterMovementSeed::default());

        monster.update_with_original_motion_inputs(
            &level_with_size(16, 16),
            &[],
            1.0,
            &[],
            &[0; 128],
        );

        assert_eq!(monster.x, 11.0);
        assert_eq!(monster.y, 19.0);
        assert_eq!(monster.original_motion.x_px, 11);
        assert_eq!(monster.original_motion.y_px, 19);
        assert_eq!(monster.original_countdown_byte, 0xfa);
        assert!(monster.facing_right);
    }

    #[test]
    fn object_0x1f_original_motion_requires_first_sequence_id() {
        let mut monster = Monster::new(10.0, 20.0, MonsterType::Floater, 0, 0.0, 0.0, 0);
        monster.object_id = 0x1f;
        monster.motion_sequence_ids = [0, 1, 0];
        monster.motion_sequence_fields = [
            None,
            Some(MonsterMotionRuntimeFields {
                anchor_index: 0,
                secondary_anchor_index: 0,
                phase_step: 0,
                limit_or_sentinel: 0,
                angle_phase: 0,
                base_x_word: 0,
                base_y_word: 0,
                x_word: 0,
                y_word: 0,
                random_y_base: 0,
            }),
            None,
        ];

        assert!(!monster.uses_original_motion_update());
    }

    #[test]
    fn shared_motion_runtime_table_advances_phase_once_per_frame() {
        let absolute = MonsterMotionRuntimeFields {
            anchor_index: 0,
            secondary_anchor_index: 0,
            phase_step: 1,
            limit_or_sentinel: -1,
            angle_phase: 0,
            base_x_word: 0,
            base_y_word: 0,
            x_word: 0,
            y_word: 0,
            random_y_base: 0,
        };
        let mut runtime_records = vec![absolute];
        let anchors = [(0x0100, -0x0100)];

        preprocess_monster_motion_runtime_fields(&mut runtime_records, &anchors, &[], &[0; 128]);
        assert_eq!(runtime_records[0].angle_phase, 1);

        let (fields, reverse) =
            resolve_motion_sequence_fields_from_runtime_records(&runtime_records, [1, 0, 0]);
        assert_eq!(reverse, [false, false, false]);

        let mut first = Monster::new(0.0, 0.0, MonsterType::Walker, 0, 0.0, 0.0, 0);
        first.object_id = 0x1f;
        first.motion_sequence_ids = [1, 0, 0];
        first.motion_sequence_fields = [Some(absolute), None, None];
        let mut second = first.clone();

        first.update_with_preprocessed_original_motion_fields(
            &level_with_size(16, 16),
            &[],
            1.0,
            fields,
        );
        second.update_with_preprocessed_original_motion_fields(
            &level_with_size(16, 16),
            &[],
            1.0,
            fields,
        );

        assert_eq!(first.original_motion.x_px, 1);
        assert_eq!(second.original_motion.x_px, 1);
        assert_eq!(runtime_records[0].angle_phase, 1);

        preprocess_monster_motion_runtime_fields(&mut runtime_records, &anchors, &[], &[0; 128]);
        assert_eq!(runtime_records[0].angle_phase, 2);
    }

    #[test]
    fn monster_player_contact_uses_original_ten_pixel_delta() {
        let monster = Monster::new(20.0, 30.0, MonsterType::Walker, 0, 0.0, 0.0, 0);

        assert!(monster.collides_with_player(&Player::new(29.9, 39.9, 0)));
        assert!(!monster.collides_with_player(&Player::new(30.0, 39.9, 0)));
        assert!(!monster.collides_with_player(&Player::new(29.9, 40.0, 0)));
    }

    #[test]
    fn original_backed_monster_does_not_use_placeholder_animation_when_mode_is_zero() {
        let mut monster = Monster::new(0.0, 0.0, MonsterType::Walker, 0x2e, 0.0, 0.0, 0);
        monster.object_id = 0x1f;
        monster.original_animation_seed = MonsterAnimationSeed {
            frame: 0x31,
            frame_min: 0x0a,
            frame_max: 0x75,
            counter: 0x9d,
            delay: 0x02,
            mode: 0,
            step: 0x1b,
        };
        monster.original_animation_mode = 0;
        monster.anim_timer = 0.16;

        monster.update(&empty_level(), &[], 0.0);

        assert_eq!(monster.anim_frame, 0);
        assert_eq!(monster.sprite_index(), 0x2e);
        assert_eq!(monster.original_animation_seed.frame, 0x31);
    }

    #[test]
    fn original_animation_tick_drives_absolute_sprite_selector_when_mode_is_active() {
        let mut monster = Monster::new(0.0, 0.0, MonsterType::Walker, 0x28, 0.0, 0.0, 0);
        monster.object_id = 0x1f;
        monster.original_animation_seed = MonsterAnimationSeed {
            frame: 0x2d,
            frame_min: 0x2d,
            frame_max: 0x2f,
            counter: 0,
            delay: 0,
            mode: 1,
            step: 1,
        };
        monster.original_animation_mode = 1;

        assert_eq!(monster.sprite_index(), 0x2d);
        monster.update(&empty_level(), &[], 0.0);

        assert_eq!(monster.anim_frame, 0);
        assert_eq!(monster.original_animation_seed.frame, 0x2e);
        assert_eq!(monster.sprite_index(), 0x2e);
    }

    #[test]
    fn shipped_monster_sprite_bases_wrap_within_same_dimension_runs() {
        assert_eq!(monster_sprite_frame_count(0x28), 3);
        assert_eq!(monster_sprite_frame_count(0x2a), 1);
        assert_eq!(monster_sprite_frame_count(0x2b), 6);
        assert_eq!(monster_sprite_frame_count(0x2d), 4);
        assert_eq!(monster_sprite_frame_count(0x2e), 3);

        let mut monster = Monster::new(0.0, 0.0, MonsterType::Walker, 0x2e, 0.0, 0.0, 0);
        monster.anim_frame = 2;
        monster.anim_timer = 0.16;
        monster.update(&empty_level(), &[], 0.0);
        assert_eq!(monster.sprite_index(), 0x2e);
    }

    #[test]
    fn dropped_powerup_gravity_uses_original_falling_entity_scale() {
        let level = empty_level();
        let mut powerup = Powerup::new(0.0, 0.0, PowerupType::Present);
        powerup.vy = 0.0;

        powerup.update(&level, 1.0 / 70.0);
        assert_eq!(powerup.vy, POWERUP_GRAVITY);

        powerup.vy = POWERUP_MAX_FALL - 0.01;
        powerup.update(&level, 1.0 / 70.0);
        assert_eq!(powerup.vy, POWERUP_MAX_FALL);
    }

    #[test]
    fn powerup_sprites_use_original_bonus_tile_range() {
        assert_eq!(PowerupType::from_bonus_id(0), PowerupType::Present);
        assert_eq!(PowerupType::from_bonus_id(1), PowerupType::BonusToken);
        assert_eq!(PowerupType::from_bonus_id(2), PowerupType::FirstAid);
        assert_eq!(PowerupType::from_bonus_id(3), PowerupType::HotDog);

        assert_eq!(PowerupType::Present.sprite_index(), 0x13);
        assert_eq!(PowerupType::BonusToken.sprite_index(), 0x14);
        assert_eq!(PowerupType::FirstAid.sprite_index(), 0x15);
        assert_eq!(PowerupType::HotDog.sprite_index(), 0x16);
        assert_eq!(PowerupType::Invincibility.sprite_index(), 0x17);
        assert_eq!(PowerupType::YellowBombBox.sprite_index(), 0x18);
        assert_eq!(PowerupType::GreenBombBox.sprite_index(), 0x19);
        assert_eq!(PowerupType::JollyCloud.sprite_index(), 0x1a);
        assert_eq!(PowerupType::BigDiamond.sprite_index(), 0x1b);

        assert_eq!(PowerupType::FirstAid.original_effect_id(), Some(2));
        assert_eq!(PowerupType::HotDog.original_effect_id(), Some(3));
        assert_eq!(PowerupType::Invincibility.original_effect_id(), Some(4));
        assert_eq!(PowerupType::YellowBombBox.original_effect_id(), Some(5));
        assert_eq!(PowerupType::GreenBombBox.original_effect_id(), Some(6));
        assert_eq!(PowerupType::Present.original_effect_id(), None);
    }

    #[test]
    fn incidental_drop_selection_consumes_original_rng() {
        let seed = 0x1357_2468;
        let mut expected = OriginalRng::new(seed);
        let expected_powerup = PowerupType::from_drop_roll(expected.gen_mod(8));

        let mut rng = OriginalRng::new(seed);
        let powerup = PowerupType::random_from_original_rng(&mut rng);

        assert_eq!(powerup, expected_powerup);
        assert_eq!(rng.seed(), expected.seed());
        assert_eq!(PowerupType::from_drop_roll(0), PowerupType::Present);
        assert_eq!(PowerupType::from_drop_roll(7), PowerupType::BigDiamond);
        assert_eq!(PowerupType::from_drop_roll(8), PowerupType::Present);
    }

    #[test]
    fn original_drop_threshold_selection_matches_fun_1000_6053_shape() {
        let thresholds = [20, 40, 70, 100];

        assert_eq!(PowerupType::original_drop_bonus_id(19, &thresholds), None);
        assert_eq!(
            PowerupType::original_drop_bonus_id(20, &thresholds),
            Some(0)
        );
        assert_eq!(
            PowerupType::original_drop_bonus_id(40, &thresholds),
            Some(0)
        );
        assert_eq!(
            PowerupType::original_drop_bonus_id(41, &thresholds),
            Some(1)
        );
        assert_eq!(
            PowerupType::original_drop_bonus_id(71, &thresholds),
            Some(2)
        );

        assert_eq!(
            PowerupType::from_original_drop_roll(20, &thresholds),
            Some(PowerupType::Present)
        );
        assert_eq!(
            PowerupType::from_original_drop_roll(41, &thresholds),
            Some(PowerupType::BonusToken)
        );

        assert_eq!(
            PowerupType::original_drop_branch_rebirth(41, &thresholds),
            Some(OriginalDropBranchRebirth {
                bonus_id: 1,
                object_id: 0x14,
                selector_id: 0x3f,
                countdown: 100,
                clears_animation_mode: true,
                y_velocity_word_delta: -200,
            })
        );
        assert_eq!(
            PowerupType::original_drop_branch_rebirth(19, &thresholds),
            None
        );
    }

    #[test]
    fn monster_spawns_use_original_pixel_coords() {
        let mut level = empty_level();
        level.monsters.push(crate::level::MonsterSpawn {
            raw: [
                0x50, 0x01, 0xa8, 0x00, 0x16, 0x05, 0, 0, 1, 3, 2, 1, 200, 0, 101, 0, 0, 0, 1, 0,
                0, 0, 1, 0, 1, 1, 3, 0, 90, 3,
            ],
        });
        let templates = (0..7)
            .map(|i| {
                let mut raw = vec![0; 38];
                raw[0x0e] = if i == 1 { 0x81 } else { i };
                raw[0x0f] = i + 1;
                raw[0x10] = i + 2;
                raw[0x06] = i;
                raw[0x08] = i + 10;
                raw[0x0a] = i + 20;
                raw[0x0c] = i + 30;
                let mut motion_record = [0; 16];
                motion_record[3] = i;
                MonsterTemplate {
                    object_id: 0x1e + i as u8,
                    anchor_table_index: i,
                    sprite_base: 0x20 + i,
                    speed: 8,
                    record_byte_05: 16,
                    initial_state: i + 20,
                    damage: 16,
                    flags: 0,
                    raw,
                    anchor_offset: Some((i as i16, -(i as i16))),
                    motion_record: Some(motion_record),
                }
            })
            .collect::<Vec<_>>();

        let monsters = spawn_monsters(&level, &templates, 0);

        assert_eq!(monsters.len(), 1);
        assert_eq!(monsters[0].x, 0x0150 as f32);
        assert_eq!(monsters[0].y, 0x00a8 as f32 - 16.0);
        assert_eq!(monsters[0].object_id, 0x1f);
        assert_eq!(monsters[0].original_state, 21);
        assert_eq!(monsters[0].state6_collision_size_tiles, None);
        assert_eq!(monsters[0].state6_damage_budget, None);
        assert_eq!(monsters[0].state6_removal_counter, None);
        assert_eq!(monsters[0].anchor_table_index, 1);
        assert_eq!(monsters[0].anchor_offset, Some((1, -1)));
        assert_eq!(
            monsters[0].motion_anchor_offsets,
            vec![(0, 0), (1, -1), (2, -2), (3, -3), (4, -4), (5, -5), (6, -6)]
        );
        assert_eq!(monsters[0].motion_sequence_ids, [0x81, 2, 3]);
        assert_eq!(monsters[0].motion_sequence_reverse, [true, false, false]);
        assert_eq!(
            monsters[0]
                .motion_sequence_fields
                .map(|field| field.map(|field| field.limit_or_sentinel)),
            [Some(0), Some(1), Some(2)]
        );
        assert_eq!(
            monsters[0].movement_seed,
            MonsterMovementSeed {
                x_velocity_word: 1,
                y_velocity_word: 11,
                x_fraction_word: 21,
                y_fraction_word: 31,
            }
        );
        assert_eq!(
            monsters[0].original_motion,
            OriginalMotionState {
                x_px: 0x0150,
                y_px: 0x00a8 - 16,
                x_fraction: 21,
                y_fraction: 31,
                x_velocity_word: 1,
                y_velocity_word: 11,
            }
        );
        assert_eq!(monsters[0].sprite_base, 0x21);
        assert_eq!(monsters[0].flags, 0);
    }

    #[test]
    fn shipped_monster_spawns_carry_original_motion_metadata() {
        let levels = crate::level::load_levels("assets/LIVELS.SCH");
        let templates = crate::level::load_monster_defs("assets/GRAN.MST");
        let expected_anchor_offsets = vec![
            (38, 10),
            (42, 4),
            (-7, 10),
            (-12, 16),
            (0, 0),
            (-8, 18),
            (39, 17),
        ];
        assert_eq!(
            resolve_monster_anchor_offsets(&templates),
            expected_anchor_offsets
        );
        let expected_animation_seeds = templates
            .iter()
            .map(MonsterTemplate::animation_seed)
            .collect::<Vec<_>>();
        let expected_animation_backups = templates
            .iter()
            .map(MonsterTemplate::animation_backup_block)
            .collect::<Vec<_>>();
        let expected_vitality_bytes = templates
            .iter()
            .map(MonsterTemplate::original_vitality_byte)
            .collect::<Vec<_>>();
        let expected_countdown_bytes = templates
            .iter()
            .map(MonsterTemplate::initial_countdown_byte)
            .collect::<Vec<_>>();
        let mut checked = 0usize;

        for (level_idx, level) in levels.iter().enumerate() {
            let monsters = spawn_monsters(level, &templates, level_idx);
            for monster in monsters {
                checked += 1;
                assert!(monster.object_id != 0);
                assert!(monster.anchor_offset.is_some());
                assert_eq!(monster.original_position_origin_offset, 0);
                assert_eq!(monster.motion_anchor_offsets, expected_anchor_offsets);
                assert!(expected_animation_seeds.contains(&monster.original_animation_seed));
                assert!(
                    expected_animation_backups.contains(&monster.original_animation_backup_block)
                );
                assert!(expected_vitality_bytes.contains(&monster.original_vitality_byte));
                assert!(expected_countdown_bytes.contains(&monster.original_countdown_byte));
                assert_eq!(
                    monster.original_animation_mode,
                    monster.original_animation_seed.mode
                );
                if monster.motion_sequence_ids.iter().any(|&id| id != 0) {
                    assert!(
                        monster.motion_sequence_fields.iter().any(Option::is_some),
                        "level {level_idx} monster with ids {:?} did not resolve a motion record",
                        monster.motion_sequence_ids
                    );
                }
            }
        }

        assert_eq!(
            checked,
            levels.iter().map(|level| level.monsters.len()).sum()
        );
    }

    #[test]
    fn shipped_monster_spawns_object_id_distribution_is_known() {
        let levels = crate::level::load_levels("assets/LIVELS.SCH");
        let templates = crate::level::load_monster_defs("assets/GRAN.MST");
        let mut distribution = std::collections::BTreeMap::new();
        let mut non_original_motion_spawns = Vec::new();
        let mut state6_sizes = Vec::new();
        let mut state6_budgets = Vec::new();
        let mut state6_counters = Vec::new();

        for (level_idx, level) in levels.iter().enumerate() {
            for (spawn_idx, monster) in spawn_monsters(level, &templates, level_idx)
                .into_iter()
                .enumerate()
            {
                *distribution.entry(monster.object_id).or_insert(0usize) += 1;
                if monster.object_id != 0x1f {
                    non_original_motion_spawns.push((level_idx, spawn_idx, monster.x, monster.y));
                    state6_sizes.push(monster.state6_collision_size_tiles);
                    state6_budgets.push(monster.state6_damage_budget);
                    state6_counters.push(monster.state6_removal_counter);
                }
                assert!(
                    monster.uses_original_motion_update() || monster.uses_state6_update(),
                    "level {level_idx} monster {spawn_idx} object {:#04x} would use fallback motion",
                    monster.object_id
                );
            }
        }

        assert_eq!(distribution.get(&0x1e), Some(&1));
        assert_eq!(distribution.get(&0x1f), Some(&14));
        assert_eq!(distribution.len(), 2);
        assert_eq!(non_original_motion_spawns, vec![(2, 2, 880.0, 248.0)]);
        assert_eq!(state6_sizes, vec![Some((5, 4))]);
        assert_eq!(state6_budgets, vec![Some(0x0a)]);
        assert_eq!(state6_counters, vec![Some(0x01)]);
    }
}

/// Spawn monsters from the level's monster table (per LIVELS.SCH spawn records).
/// Each spawn is a 30-byte original spawn-controller record. The current live
/// bridge still maps raw byte `0x04` to a GRAN.MST template directly until the
/// full original timer/count/template-selector lifecycle is wired. We blend the
/// template's sprite_base/speed with the per-type defaults so unknown templates
/// still look reasonable.
pub fn spawn_monsters(
    level: &Level,
    templates: &[MonsterTemplate],
    _level_idx: usize,
) -> Vec<Monster> {
    let mut monsters = Vec::new();
    let motion_anchor_offsets = resolve_monster_anchor_offsets(templates);
    for s in &level.monsters {
        let x = s.x_px() as f32;
        let y = s.y_px() as f32;
        let tidx = s.template_seed() as usize;
        let mt = MonsterType::from_id(tidx as u8);
        let (
            object_id,
            anchor_table_index,
            anchor_offset,
            motion_sequence_ids,
            motion_sequence_fields,
            motion_sequence_reverse,
            movement_seed,
            animation_seed,
            animation_backup_block,
            original_state,
            original_position_origin_offset,
            state6_collision_size_tiles,
            state6_damage_budget,
            original_vitality_byte,
            original_countdown_byte,
            state6_removal_counter,
            state6_death_wakeup_key,
            dependent_death_key,
            sprite_base,
            speed,
            damage,
            flags,
        ) = if !templates.is_empty() {
            let t = &templates[tidx % templates.len()];
            // GRAN.MST byte 1 is an anchor-table index in the original, so
            // keep placeholder movement on the local type default until the
            // original state machine is wired.
            let sp = mt.speed();
            // GRAN.MST byte 5 is not yet mapped to contact damage in the
            // decompiled path, so keep placeholder damage on the local type.
            let dmg = mt.damage();
            let (motion_sequence_fields, motion_sequence_reverse) =
                resolve_motion_sequence_fields(templates, t.motion_sequence_ids());
            (
                t.object_id,
                t.anchor_table_index,
                t.anchor_offset,
                t.motion_sequence_ids(),
                motion_sequence_fields,
                motion_sequence_reverse,
                t.movement_seed(),
                t.animation_seed(),
                t.animation_backup_block(),
                t.initial_state,
                t.position_origin_offset(),
                t.state6_collision_size_tiles(),
                t.state6_damage_budget(),
                t.original_vitality_byte(),
                t.initial_countdown_byte(),
                t.state6_removal_counter(),
                t.state6_death_wakeup_key(),
                t.dependent_death_key(),
                t.sprite_base as usize,
                sp,
                dmg,
                t.flags,
            )
        } else {
            (
                0,
                0,
                None,
                [0; 3],
                [None; 3],
                [false; 3],
                MonsterMovementSeed::default(),
                MonsterAnimationSeed::default(),
                [0; 7],
                0,
                0,
                None,
                None,
                0,
                0,
                None,
                0,
                0,
                mt.sprite_base(),
                mt.speed(),
                mt.damage(),
                0,
            )
        };
        let mut monster = Monster::new(x, y - 16.0, mt, sprite_base, speed, damage, flags);
        monster.object_id = object_id;
        monster.original_state = original_state;
        monster.original_position_origin_offset = original_position_origin_offset;
        monster.state6_collision_size_tiles = state6_collision_size_tiles;
        monster.state6_damage_budget = state6_damage_budget;
        monster.original_vitality_byte = original_vitality_byte;
        monster.original_countdown_byte = original_countdown_byte;
        monster.state6_removal_counter = state6_removal_counter;
        monster.state6_death_wakeup_key = state6_death_wakeup_key;
        monster.dependent_death_key = dependent_death_key;
        monster.anchor_table_index = anchor_table_index;
        monster.anchor_offset = anchor_offset;
        monster.motion_anchor_offsets = motion_anchor_offsets.clone();
        monster.motion_sequence_ids = motion_sequence_ids;
        monster.motion_sequence_fields = motion_sequence_fields;
        monster.motion_sequence_reverse = motion_sequence_reverse;
        monster.movement_seed = movement_seed;
        monster.original_animation_seed = animation_seed;
        monster.original_animation_backup_block = animation_backup_block;
        monster.original_animation_mode = animation_seed.mode;
        monster.original_motion = OriginalMotionState::new(x, y - 16.0, movement_seed);
        monsters.push(monster);
    }
    monsters
}

fn resolve_monster_anchor_offsets(templates: &[MonsterTemplate]) -> Vec<(i16, i16)> {
    let Some(max_index) = templates
        .iter()
        .map(|template| template.anchor_table_index as usize)
        .max()
    else {
        return Vec::new();
    };
    let mut offsets = vec![(0, 0); max_index + 1];
    for template in templates {
        if let Some(anchor_offset) = template.anchor_offset {
            offsets[template.anchor_table_index as usize] = anchor_offset;
        }
    }
    offsets
}

pub fn monster_motion_runtime_fields_from_templates(
    templates: &[MonsterTemplate],
) -> Vec<MonsterMotionRuntimeFields> {
    templates
        .iter()
        .filter_map(MonsterTemplate::runtime_motion_fields)
        .collect()
}

pub fn preprocess_monster_motion_runtime_fields(
    fields: &mut [MonsterMotionRuntimeFields],
    anchor_offsets: &[(i16, i16)],
    random_words: &[(i16, i16)],
    trig_words: &[i16],
) {
    for (idx, field) in fields.iter_mut().enumerate() {
        *field = if let Some((x_phase, y_phase)) = field.absolute_trig_phase_indices_after_advance()
        {
            let anchor = anchor_offsets
                .get(field.anchor_index as usize)
                .copied()
                .unwrap_or_default();
            let trig_x = trig_words
                .get(x_phase as usize)
                .copied()
                .unwrap_or_default();
            let trig_y = trig_words
                .get(y_phase as usize)
                .copied()
                .unwrap_or_default();
            field.with_absolute_preprocess(anchor, trig_x, trig_y)
        } else {
            let (random_x, random_y) = random_words.get(idx).copied().unwrap_or_default();
            field.with_random_preprocess(random_x, random_y)
        };
    }
}

pub fn resolve_motion_sequence_fields_from_runtime_records(
    records: &[MonsterMotionRuntimeFields],
    ids: [u8; 3],
) -> ([Option<MonsterMotionRuntimeFields>; 3], [bool; 3]) {
    let fields = ids.map(|id| {
        if id == 0 {
            None
        } else {
            let index = if id > 0x80 { id.wrapping_add(0x80) } else { id };
            records.get(index as usize - 1).copied()
        }
    });
    let reverse = ids.map(|id| id > 0x80);
    (fields, reverse)
}

fn resolve_motion_sequence_fields(
    templates: &[MonsterTemplate],
    ids: [u8; 3],
) -> ([Option<MonsterMotionRuntimeFields>; 3], [bool; 3]) {
    let fields = ids.map(|id| {
        if id == 0 {
            None
        } else {
            let index = if id > 0x80 { id.wrapping_add(0x80) } else { id };
            templates
                .get(index as usize - 1)
                .and_then(MonsterTemplate::runtime_motion_fields)
        }
    });
    let reverse = ids.map(|id| id > 0x80);
    (fields, reverse)
}
