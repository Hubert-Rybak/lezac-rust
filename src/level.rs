use crate::assets::rle_decompress;
use crate::original_rng::OriginalRng;

/// Solid-for-walking tile threshold from FUN_1000_6053.
pub const TILE_SOLID_MAX: u8 = 0x4C;
/// Solid-for-ceiling threshold (slightly higher).
pub const TILE_CEIL_MAX: u8 = 0x52;
pub const TILE_PIXELS: usize = 8;

/// Low-memory threshold table at DS:0x52 in the original executable.
pub const ORIGINAL_DROP_THRESHOLDS_0X52: [u8; 8] = [0x28, 0x41, 0x47, 0x4e, 0x53, 0x59, 0x5d, 0x64];
/// Low-memory contact selector bytes at DS:0x42, indexed as `(object_id - 0x12) * 2`.
pub const ORIGINAL_PLAYER_CONTACT_SELECTORS_0X42: [u8; 24] = [
    0x88, 0x13, 0x58, 0x00, 0x56, 0x00, 0x57, 0x00, 0x58, 0x00, 0x59, 0x00, 0x56, 0x00, 0x5a, 0x00,
    0x28, 0x41, 0x47, 0x4e, 0x53, 0x59, 0x5d, 0x64,
];
/// Low-memory animation min/max pairs at DS:0x58.
pub const ORIGINAL_ANIMATION_RANGES_0X58: [(u8, u8); 25] = [
    (0x5d, 0x64),
    (0x2c, 0x2d),
    (0x2e, 0x2f),
    (0x01, 0x02),
    (0x22, 0x27),
    (0x02, 0x09),
    (0x0a, 0x11),
    (0x15, 0x1c),
    (0x1d, 0x23),
    (0x45, 0x49),
    (0x4a, 0x4f),
    (0x28, 0x2a),
    (0x32, 0x34),
    (0x36, 0x38),
    (0x29, 0x2a),
    (0x2b, 0x2c),
    (0x28, 0x28),
    (0x30, 0x31),
    (0x2b, 0x2b),
    (0x35, 0x35),
    (0x39, 0x39),
    (0x01, 0x02),
    (0x0b, 0x0b),
    (0x0c, 0x0c),
    (0x0d, 0x0d),
];
/// Low-memory selector bytes starting at DS:0x77 for low-object damage response.
pub const ORIGINAL_LOW_OBJECT_DAMAGE_SELECTORS_0X77: [u8; 19] = [
    0x2c, 0x28, 0x28, 0x30, 0x31, 0x2b, 0x2b, 0x35, 0x35, 0x39, 0x39, 0x01, 0x02, 0x0b, 0x0b, 0x0c,
    0x0c, 0x0d, 0x0d,
];
/// Low-memory spawn selector pairs at DS:0x80/0x81.
pub const ORIGINAL_SPAWN_SELECTOR_PAIRS_0X80: [(u8, u8); 5] = [
    (0x39, 0x39),
    (0x01, 0x02),
    (0x0b, 0x0b),
    (0x0c, 0x0c),
    (0x0d, 0x0d),
];
/// Low-memory cleanup/death animation selectors at DS:0x6a/0x6c/0x6d.
pub const ORIGINAL_CLEANUP_ANIMATION_SELECTORS: OriginalAnimationSelectors =
    OriginalAnimationSelectors {
        low_object_frame: 0x45,
        high_object_frame: 0x4a,
        max_frame: 0x4f,
    };

#[derive(Clone, Copy, Debug)]
pub struct MonsterSpawn {
    pub raw: [u8; 30],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalSpawnRuntimeFields {
    pub word_0x0e: u16,
    pub word_0x10: u16,
    pub word_0x12: u16,
    pub vitality: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalSpawnAllocationRequest {
    pub x_px: u16,
    pub y_px: u16,
    pub template_selector: u8,
    pub allocation_param: u8,
    pub selector_table_byte_0x80: u8,
    pub selector_table_byte_0x81: u8,
    pub animation_seed: MonsterAnimationSeed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalSpawnControllerEvent {
    pub runtime_fields: OriginalSpawnRuntimeFields,
    pub allocation_request: OriginalSpawnAllocationRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalTeleportEffectRequest {
    pub target_x_px: u16,
    pub target_y_px: u16,
    pub allocation_call: OriginalObjectAllocationCall,
    pub animation_seed: MonsterAnimationSeed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalDeathDebrisEffectRequest {
    pub allocation_call: OriginalObjectAllocationCall,
    pub allocated_animation_seed: MonsterAnimationSeed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalDirectionalTileHitEffectRequest {
    pub allocation_call: OriginalObjectAllocationCall,
    pub cleared_animation_seed: MonsterAnimationSeed,
    pub next_effect_count_0x208e: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalObjectAllocationCall {
    pub param_1: u8,
    pub param_2: u8,
    pub param_3: u8,
    pub param_4: u16,
    pub param_5_x_velocity: i16,
    pub param_6_y_velocity: i16,
    pub param_7_x_px: u16,
    pub param_8_y_px: u16,
}

impl OriginalSpawnAllocationRequest {
    pub fn original_object_allocation_call(self) -> OriginalObjectAllocationCall {
        OriginalObjectAllocationCall {
            param_1: self.allocation_param,
            param_2: 0,
            param_3: self.template_selector,
            param_4: u16::from(self.animation_seed.frame_min),
            param_5_x_velocity: 0,
            param_6_y_velocity: 0,
            param_7_x_px: self.x_px,
            param_8_y_px: self.y_px,
        }
    }
}

impl OriginalObjectAllocationCall {
    pub fn attempt(
        self,
        active_count: u8,
        selector_entry: Option<OriginalSpriteSelectorEntry>,
    ) -> Option<OriginalObjectAllocationFields> {
        original_object_allocation_attempt(
            active_count,
            self.param_5_x_velocity,
            self.param_6_y_velocity,
            self.param_4,
            selector_entry,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalObjectAllocationFields {
    pub next_active_count: u8,
    pub x_velocity_word: i16,
    pub y_velocity_word: i16,
    pub position_origin_offset: i8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalTileHitResult {
    /// Tile byte copied to low-memory `0x79a3`.
    pub tile_id: u8,
    /// Word copied to `0x2072` from the injected `0x1a` table.
    pub spawn_word_0x2072: u16,
    /// Signed word added to `0x2074` from the injected table at offset `0x02`.
    pub score_delta_0x2074: i16,
    /// Whether `FUN_1000_5afd` sets `0x661e = 1`, causing the caller to run
    /// `FUN_1000_370e` for the tile above the sampled cell.
    pub triggers_above_tile_damage_scan: bool,
    /// Replacement byte written to the sampled tile cell.
    pub replacement_tile: u8,
    /// Whether the matching attribute word is also cleared.
    pub clears_tile_attribute: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalAttributeEffectRecord {
    pub attr_ref: u16,
    pub effect_a: u8,
    pub effect_b: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalAttributeEffectLookup {
    pub one_based_index: u16,
    pub effect: u8,
}

pub fn original_destructible_tile_hit(
    tile: u8,
    tile_attr: u16,
    paired_attr: u16,
    spawn_words_0x1a: &[u16],
    score_deltas_0x02: &[i16],
) -> Option<OriginalTileHitResult> {
    if !(b'g'..=b'r').contains(&tile) {
        return None;
    }

    let index = (tile - b'g') as usize;
    let clears_tile_attribute = tile_attr < 0x8000;
    Some(OriginalTileHitResult {
        tile_id: tile,
        spawn_word_0x2072: spawn_words_0x1a.get(index).copied().unwrap_or_default(),
        score_delta_0x2074: score_deltas_0x02.get(index).copied().unwrap_or_default(),
        triggers_above_tile_damage_scan: paired_attr != 0 && paired_attr < 0x8000,
        replacement_tile: if clears_tile_attribute { 0 } else { 0xff },
        clears_tile_attribute,
    })
}

pub fn original_reverse_lookup_11_byte_record_index(
    record_refs: &[u16],
    target_ref: u16,
) -> Option<u16> {
    record_refs
        .iter()
        .enumerate()
        .rev()
        .find(|(_, record_ref)| **record_ref == target_ref)
        .map(|(idx, _)| idx as u16 + 1)
}

pub fn original_attribute_effect_lookup(
    attr_ref: u16,
    low_records: &[OriginalAttributeEffectRecord],
    high_records: &[OriginalAttributeEffectRecord],
    use_second_effect_byte: bool,
) -> Option<OriginalAttributeEffectLookup> {
    if attr_ref & 0x8000 == 0 {
        return None;
    }

    let records = if attr_ref & 0x7fff < 0x4000 {
        low_records
    } else {
        high_records
    };

    records
        .iter()
        .enumerate()
        .rev()
        .find(|(_, record)| record.attr_ref == attr_ref)
        .map(|(idx, record)| OriginalAttributeEffectLookup {
            one_based_index: idx as u16 + 1,
            effect: if use_second_effect_byte {
                record.effect_b
            } else {
                record.effect_a
            },
        })
}

pub fn original_teleport_effect_request(
    bonus_records: &[BonusSpawn],
    tile_attr_ref: u16,
    selector_frame_min_0x6c: u8,
    selector_frame_max_0x6d: u8,
) -> Option<OriginalTeleportEffectRequest> {
    let attr_ref = tile_attr_ref & 0x7fff;
    if attr_ref == 0 {
        return None;
    }

    let target = bonus_records
        .iter()
        .find(|record| record.map_cell_ref() == attr_ref)?;
    let target_x_px = target.x_px();
    let target_y_px = target.y_px();
    Some(OriginalTeleportEffectRequest {
        target_x_px,
        target_y_px,
        allocation_call: OriginalObjectAllocationCall {
            param_1: 5,
            param_2: 8,
            param_3: 0x0b,
            param_4: u16::from(selector_frame_min_0x6c),
            param_5_x_velocity: 0,
            param_6_y_velocity: 0,
            param_7_x_px: target_x_px,
            param_8_y_px: target_y_px,
        },
        animation_seed: MonsterAnimationSeed::from_original_setup(
            1,
            2,
            selector_frame_max_0x6d,
            selector_frame_min_0x6c,
        ),
    })
}

pub fn original_death_debris_effect_request(
    x_px: u16,
    y_px: u16,
    x_velocity_word: i16,
    y_velocity_word: i16,
    selectors: OriginalAnimationSelectors,
) -> OriginalDeathDebrisEffectRequest {
    OriginalDeathDebrisEffectRequest {
        allocation_call: OriginalObjectAllocationCall {
            param_1: 5,
            param_2: 0x0f,
            param_3: 0x0b,
            param_4: 0x0d,
            param_5_x_velocity: x_velocity_word,
            param_6_y_velocity: y_velocity_word,
            param_7_x_px: x_px,
            param_8_y_px: y_px,
        },
        allocated_animation_seed: MonsterAnimationSeed::from_original_setup(
            2,
            2,
            selectors.max_frame,
            selectors.low_object_frame,
        ),
    }
}

pub fn original_directional_tile_hit_effect_request(
    origin_x_px: i16,
    origin_y_px: i16,
    direction_index_1_to_4: u8,
    selector_word_0x2072: u16,
    active_effect_count_0x208e: u8,
    x_velocity_roll_0_to_199: u16,
) -> Option<OriginalDirectionalTileHitEffectRequest> {
    if selector_word_0x2072 == 0 || active_effect_count_0x208e >= 0x0e {
        return None;
    }

    let (x_offset, y_offset) = match direction_index_1_to_4 {
        1 => (-2, -2),
        2 => (-2, 10),
        3 => (10, 10),
        4 => (10, -2),
        _ => return None,
    };

    Some(OriginalDirectionalTileHitEffectRequest {
        allocation_call: OriginalObjectAllocationCall {
            param_1: 5,
            param_2: 0x0c,
            param_3: 10,
            param_4: selector_word_0x2072,
            param_5_x_velocity: -0x28 - x_velocity_roll_0_to_199 as i16,
            param_6_y_velocity: 0,
            param_7_x_px: origin_x_px.wrapping_add(x_offset) as u16,
            param_8_y_px: origin_y_px.wrapping_add(y_offset) as u16,
        },
        cleared_animation_seed: MonsterAnimationSeed::from_original_setup(0, 0, 0, 0),
        next_effect_count_0x208e: active_effect_count_0x208e.wrapping_add(1),
    })
}

pub fn original_object_allocation_velocity_word(value: i16) -> i16 {
    value.clamp(-0x07ff, 0x07ff)
}

pub fn original_object_allocation_next_count(active_count: u8) -> Option<u8> {
    if active_count < 0x1e {
        Some(active_count.wrapping_add(1))
    } else {
        None
    }
}

pub fn original_object_allocation_position_origin_offset(
    sprite_selector_id: u16,
    selector_entry: Option<OriginalSpriteSelectorEntry>,
) -> Option<i8> {
    if sprite_selector_id == 0x1f {
        Some(0)
    } else {
        selector_entry.map(OriginalSpriteSelectorEntry::position_origin_offset)
    }
}

pub fn original_object_allocation_attempt(
    active_count: u8,
    x_velocity_word: i16,
    y_velocity_word: i16,
    sprite_selector_id: u16,
    selector_entry: Option<OriginalSpriteSelectorEntry>,
) -> Option<OriginalObjectAllocationFields> {
    Some(OriginalObjectAllocationFields {
        next_active_count: original_object_allocation_next_count(active_count)?,
        x_velocity_word: original_object_allocation_velocity_word(x_velocity_word),
        y_velocity_word: original_object_allocation_velocity_word(y_velocity_word),
        position_origin_offset: original_object_allocation_position_origin_offset(
            sprite_selector_id,
            selector_entry,
        )?,
    })
}

impl MonsterSpawn {
    fn raw_u16_at(self, offset: usize) -> u16 {
        u16::from_le_bytes([self.raw[offset], self.raw[offset + 1]])
    }

    pub fn x_px(self) -> u16 {
        self.raw_u16_at(0)
    }
    pub fn y_px(self) -> u16 {
        self.raw_u16_at(2)
    }
    pub fn template_seed(self) -> u8 {
        self.raw[4]
    }
    pub fn original_active_flag(self) -> u8 {
        self.raw[0x08]
    }
    pub fn original_spawn_count(self) -> u8 {
        self.raw[0x09]
    }
    pub fn original_spawn_budget(self) -> u8 {
        self.raw[0x0a]
    }
    pub fn original_template_selector(self) -> u8 {
        self.raw[0x0b]
    }
    pub fn original_runtime_word_0x0e_base(self) -> u16 {
        self.raw_u16_at(0x0c)
    }
    pub fn original_runtime_word_0x0e_random_modulus(self) -> u16 {
        self.raw_u16_at(0x0e)
    }
    pub fn original_runtime_word_0x10_base(self) -> u16 {
        self.raw_u16_at(0x10)
    }
    pub fn original_runtime_word_0x10_random_modulus(self) -> u16 {
        self.raw_u16_at(0x12)
    }
    pub fn original_runtime_word_0x12_base(self) -> u16 {
        self.raw_u16_at(0x14)
    }
    pub fn original_runtime_word_0x12_random_modulus(self) -> u16 {
        self.raw_u16_at(0x16)
    }
    pub fn original_health_base(self) -> u8 {
        self.raw[0x18]
    }
    pub fn original_health_random_modulus(self) -> u8 {
        self.raw[0x19]
    }
    pub fn original_spawn_allocation_param(self) -> u8 {
        self.raw[0x1a]
    }
    pub fn original_spawn_timer(self) -> u8 {
        self.raw[0x1b]
    }
    pub fn original_spawn_timer_reset(self) -> u8 {
        self.raw[0x1c]
    }
    pub fn original_spawn_animation_delay(self) -> u8 {
        self.raw[0x1d]
    }
    pub fn original_tick_spawn_controller(&mut self) -> bool {
        self.raw[0x1b] = self.raw[0x1b].wrapping_sub(1);
        let should_attempt = self.original_spawn_timer() == 0
            && self.original_spawn_budget() != 0
            && self.original_spawn_count() != 0
            && self.original_active_flag() == 1;
        if should_attempt {
            self.raw[0x1b] = self.raw[0x1c];
        }
        should_attempt
    }
    pub fn original_commit_spawn_success(&mut self) {
        self.raw[0x09] = self.raw[0x09].wrapping_sub(1);
        self.raw[0x0a] = self.raw[0x0a].wrapping_sub(1);
    }
    pub fn original_spawn_runtime_fields(
        self,
        original_rng: &mut OriginalRng,
    ) -> OriginalSpawnRuntimeFields {
        OriginalSpawnRuntimeFields {
            word_0x0e: self.original_runtime_word_0x0e_base().wrapping_add(
                original_rng.gen_mod(self.original_runtime_word_0x0e_random_modulus()),
            ),
            word_0x10: self.original_runtime_word_0x10_base().wrapping_add(
                original_rng.gen_mod(self.original_runtime_word_0x10_random_modulus()),
            ),
            word_0x12: self.original_runtime_word_0x12_base().wrapping_add(
                original_rng.gen_mod(self.original_runtime_word_0x12_random_modulus()),
            ),
            vitality: self.original_health_base().wrapping_add(
                original_rng.gen_mod(self.original_health_random_modulus() as u16) as u8,
            ),
        }
    }
    pub fn original_spawn_allocation_request(
        self,
        selector_table_pair: (u8, u8),
        animation_range: (u8, u8),
    ) -> OriginalSpawnAllocationRequest {
        OriginalSpawnAllocationRequest {
            x_px: self.x_px(),
            y_px: self.y_px(),
            template_selector: self.original_template_selector(),
            allocation_param: self.original_spawn_allocation_param(),
            selector_table_byte_0x80: selector_table_pair.0,
            selector_table_byte_0x81: selector_table_pair.1,
            animation_seed: MonsterAnimationSeed::from_original_setup(
                1,
                self.original_spawn_animation_delay(),
                animation_range.1,
                animation_range.0,
            ),
        }
    }
    pub fn original_spawn_allocation_request_from_tables(
        self,
        selector_table_pairs: &[(u8, u8)],
        animation_ranges: &[(u8, u8)],
    ) -> Option<OriginalSpawnAllocationRequest> {
        let selector_pair = *selector_table_pairs.get(self.original_template_selector() as usize)?;
        let animation_range = *animation_ranges.get(selector_pair.0 as usize)?;
        Some(self.original_spawn_allocation_request(selector_pair, animation_range))
    }
    pub fn original_spawn_allocation_request_from_original_tables(
        self,
    ) -> Option<OriginalSpawnAllocationRequest> {
        self.original_spawn_allocation_request_from_tables(
            &ORIGINAL_SPAWN_SELECTOR_PAIRS_0X80,
            &ORIGINAL_ANIMATION_RANGES_0X58,
        )
    }
    pub fn advance_original_spawn_controller(
        &mut self,
        original_rng: &mut OriginalRng,
    ) -> Option<OriginalSpawnControllerEvent> {
        if !self.original_tick_spawn_controller() {
            return None;
        }

        let runtime_fields = self.original_spawn_runtime_fields(original_rng);
        let allocation_request = self.original_spawn_allocation_request_from_original_tables()?;
        self.original_commit_spawn_success();
        Some(OriginalSpawnControllerEvent {
            runtime_fields,
            allocation_request,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BonusSpawn {
    pub raw: [u8; 7],
}
impl BonusSpawn {
    pub fn map_cell_ref(self) -> u16 {
        u16::from_le_bytes([self.raw[0], self.raw[1]])
    }
    pub fn x_px(self) -> u16 {
        u16::from_le_bytes([self.raw[2], self.raw[3]])
    }
    pub fn y_px(self) -> u16 {
        u16::from_le_bytes([self.raw[4], self.raw[5]])
    }
    pub fn entity_flags(self) -> u8 {
        self.raw[6]
    }
    pub fn player_start_mask(self) -> u8 {
        self.entity_flags() & 0x03
    }
    pub fn starts_player(self, player_index: usize) -> bool {
        let mask = 1_u8 << player_index.min(7);
        self.player_start_mask() & mask != 0
    }
    /// Shipped non-start seven-byte records carry flag 0, which maps to the
    /// baseline level-goal pickup used by the current collectible bridge.
    pub fn spawned_bonus_id(self) -> u8 {
        self.entity_flags()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PlatformSpawn {
    pub raw: [u8; 14],
}
impl PlatformSpawn {
    pub fn source_x_px(self) -> u16 {
        u16::from_le_bytes([self.raw[0], self.raw[1]])
    }
    pub fn source_y_px(self) -> u16 {
        u16::from_le_bytes([self.raw[2], self.raw[3]])
    }
    pub fn source_tile(self) -> (usize, usize) {
        let x = self.source_x_px() as usize / TILE_PIXELS;
        let y = self.source_y_px() as usize / TILE_PIXELS;
        (x, y)
    }
    pub fn destination_tile(self) -> (usize, usize) {
        let x = if self.raw[5] == 0x40 {
            self.raw[4] as usize
        } else {
            u16::from_le_bytes([self.raw[4], self.raw[5]]) as usize / TILE_PIXELS
        };
        let y = self.raw[6] as usize;
        (x, y)
    }
    pub fn affected_map_cell_ref_range(self) -> (u16, u16) {
        (
            u16::from_le_bytes([self.raw[0], self.raw[1]]),
            u16::from_le_bytes([self.raw[2], self.raw[3]]),
        )
    }
    pub fn trigger_map_cell_ref(self) -> u16 {
        u16::from_le_bytes([self.raw[4], self.raw[5]])
    }
    pub fn tile_substitutions(self) -> [(u8, u8); 4] {
        [
            (self.raw[6], self.raw[10]),
            (self.raw[7], self.raw[11]),
            (self.raw[8], self.raw[12]),
            (self.raw[9], self.raw[13]),
        ]
    }
}

#[derive(Clone)]
pub struct Level {
    pub width: usize,
    pub height: usize,
    pub destruction_tile: u8,
    pub bonus_target: u16,
    pub destruction_pct: u8,
    pub tiles: Vec<u8>,      // w*h bytes
    pub attrs: Vec<u16>,     // w*h u16s
    pub orig_tiles: Vec<u8>, // pristine copy for restart
    pub initial_destruction_tile_count: u32,
    pub scroll_x: u16,
    pub scroll_y: u16,
    pub monsters: Vec<MonsterSpawn>,
    pub bonuses: Vec<BonusSpawn>,
    pub platforms: Vec<PlatformSpawn>,
}

impl Level {
    pub fn tile_at(&self, x: usize, y: usize) -> u8 {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x]
        } else {
            1
        }
    }
    pub fn set_tile(&mut self, x: usize, y: usize, v: u8) {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x] = v;
        }
    }
    pub fn attr_at(&self, x: usize, y: usize) -> u16 {
        if x < self.width && y < self.height {
            self.attrs[y * self.width + x]
        } else {
            0
        }
    }

    /// Solid for walking/collision: non-zero and ≤ 0x4C.
    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        let t = self.tile_at(x, y);
        t != 0 && t <= TILE_SOLID_MAX
    }

    pub fn count_destruction_tiles(&self) -> u32 {
        self.tiles
            .iter()
            .filter(|&&t| t == self.destruction_tile)
            .count() as u32
    }

    pub fn reset(&mut self) {
        self.tiles.clone_from(&self.orig_tiles);
    }

    pub fn apply_platform_action(&mut self, trigger_ref: u16) -> bool {
        let trigger_ref = trigger_ref & 0x7fff;
        let Some(platform) = original_platform_action_record(&self.platforms, trigger_ref).copied()
        else {
            return false;
        };
        let (range_start, range_end) = platform.affected_map_cell_ref_range();
        let substitutions = platform.tile_substitutions();
        let mut changed = false;

        for (tile, attr) in self.tiles.iter_mut().zip(&self.attrs) {
            let attr_ref = attr & 0x7fff;
            if attr_ref < range_start || attr_ref > range_end {
                continue;
            }
            for (from, to) in substitutions {
                if from != 0 && *tile == from {
                    *tile = to;
                    changed = true;
                    break;
                }
            }
        }

        changed
    }
}

fn original_platform_action_record(
    platforms: &[PlatformSpawn],
    trigger_ref: u16,
) -> Option<&PlatformSpawn> {
    let trigger_ref = trigger_ref & 0x7fff;
    let mut selected = None;
    for platform in platforms {
        if platform.trigger_map_cell_ref() == trigger_ref {
            selected = Some(platform);
        }
    }
    selected
}

fn read_u16(d: &[u8], p: &mut usize) -> u16 {
    let v = u16::from_le_bytes([d[*p], d[*p + 1]]);
    *p += 2;
    v
}
fn read_u8(d: &[u8], p: &mut usize) -> u8 {
    let v = d[*p];
    *p += 1;
    v
}

fn read_rle_block(d: &[u8], p: &mut usize, output_size: usize) -> Vec<u8> {
    let comp_size = read_u16(d, p) as usize;
    let end = (*p + comp_size).min(d.len());
    let out = rle_decompress(&d[*p..end], output_size);
    *p = end;
    out
}

pub fn load_levels(path: &str) -> Vec<Level> {
    let d = crate::assets::load_file(path);
    let mut out = Vec::new();
    let mut p = 0;
    while p + 8 <= d.len() {
        let w = read_u16(&d, &mut p) as usize;
        let h = read_u16(&d, &mut p) as usize;
        if w == 0 || h == 0 || w > 500 || h > 500 {
            break;
        }
        let destruction_tile = read_u8(&d, &mut p);
        let bonus_target = read_u16(&d, &mut p);
        let destruction_pct = read_u8(&d, &mut p);
        let n = w * h;
        let tiles = read_rle_block(&d, &mut p, n);
        let attr_bytes = read_rle_block(&d, &mut p, n * 2);
        let mut attrs = Vec::with_capacity(n);
        for i in 0..n {
            attrs.push(u16::from_le_bytes([
                attr_bytes[i * 2],
                attr_bytes[i * 2 + 1],
            ]));
        }
        if p + 4 > d.len() {
            break;
        }
        let scroll_x = read_u16(&d, &mut p);
        let scroll_y = read_u16(&d, &mut p);
        if p >= d.len() {
            break;
        }

        let mcount = read_u8(&d, &mut p) as usize;
        let mut monsters = Vec::with_capacity(mcount);
        for _ in 0..mcount {
            if p + 30 > d.len() {
                break;
            }
            let mut raw = [0u8; 30];
            raw.copy_from_slice(&d[p..p + 30]);
            monsters.push(MonsterSpawn { raw });
            p += 30;
        }

        if p >= d.len() {
            out.push(finalize(
                w,
                h,
                destruction_tile,
                bonus_target,
                destruction_pct,
                tiles,
                attrs,
                scroll_x,
                scroll_y,
                monsters,
                vec![],
                vec![],
            ));
            break;
        }
        let bcount = read_u8(&d, &mut p) as usize;
        let mut bonuses = Vec::with_capacity(bcount);
        for _ in 0..bcount {
            if p + 7 > d.len() {
                break;
            }
            let mut raw = [0u8; 7];
            raw.copy_from_slice(&d[p..p + 7]);
            bonuses.push(BonusSpawn { raw });
            p += 7;
        }

        let platforms = if p < d.len() {
            let pcount = read_u8(&d, &mut p) as usize;
            let mut v = Vec::with_capacity(pcount);
            for _ in 0..pcount {
                if p + 14 > d.len() {
                    break;
                }
                let mut raw = [0u8; 14];
                raw.copy_from_slice(&d[p..p + 14]);
                v.push(PlatformSpawn { raw });
                p += 14;
            }
            v
        } else {
            vec![]
        };

        out.push(finalize(
            w,
            h,
            destruction_tile,
            bonus_target,
            destruction_pct,
            tiles,
            attrs,
            scroll_x,
            scroll_y,
            monsters,
            bonuses,
            platforms,
        ));
        if out.len() >= 7 {
            break;
        }
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn finalize(
    w: usize,
    h: usize,
    destruction_tile: u8,
    bonus_target: u16,
    destruction_pct: u8,
    tiles: Vec<u8>,
    attrs: Vec<u16>,
    scroll_x: u16,
    scroll_y: u16,
    monsters: Vec<MonsterSpawn>,
    bonuses: Vec<BonusSpawn>,
    platforms: Vec<PlatformSpawn>,
) -> Level {
    let orig_tiles = tiles.clone();
    let initial_destruction_tile_count =
        tiles.iter().filter(|&&t| t == destruction_tile).count() as u32;
    Level {
        width: w,
        height: h,
        destruction_tile,
        bonus_target,
        destruction_pct,
        tiles,
        attrs,
        orig_tiles,
        initial_destruction_tile_count,
        scroll_x,
        scroll_y,
        monsters,
        bonuses,
        platforms,
    }
}

/// GRAN.MST begins with a count byte followed by 38-byte behavior templates.
/// The trailing block starts with one sprite byte per template, then one X/Y
/// word pair per template, then a count byte followed by that many 16-byte
/// motion/animation records. Most fields are still unnamed, so we expose the
/// partitioned bytes without over-interpreting them.
#[derive(Clone, Debug)]
pub struct MonsterTemplate {
    /// Original object/entity id from GRAN.MST byte 0. The decompiled update
    /// code reads this as `local_11` and branches on values such as `0x1f`.
    pub object_id: u8,
    /// Fixed-record byte 1. `FUN_1000_6053` reads this as `bVar2` and uses it
    /// to look up the current X/Y anchor in the offset table.
    pub anchor_table_index: u8,
    pub sprite_base: u8,
    /// Legacy bridge field retained for compatibility with older tests; byte 1
    /// is now decoded separately as `anchor_table_index` and is not used as
    /// placeholder movement speed.
    pub speed: u8,
    /// Fixed-record byte 5, retained as an unnamed source byte while its
    /// original role is still being mapped.
    pub record_byte_05: u8,
    /// Fixed-record byte 0x15. `FUN_1000_6053` reads this as `local_33` and
    /// dispatches major per-object state behavior from it.
    pub initial_state: u8,
    /// Legacy bridge field retained for compatibility with older tests; byte 5
    /// is now decoded separately as `record_byte_05` and is not used as
    /// placeholder contact damage.
    pub damage: u8,
    /// Runtime behavior flags used by the Rust bridge. GRAN.MST byte 0 is an
    /// object id, not these flags, so loaded templates currently leave this at
    /// zero until the original flag/state fields are fully mapped.
    pub flags: u8,
    pub raw: Vec<u8>,
    pub anchor_offset: Option<(i16, i16)>,
    pub motion_record: Option<[u8; MONSTER_MOTION_RECORD_LEN]>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MonsterAnimationSeed {
    pub frame: u8,
    pub frame_min: u8,
    pub frame_max: u8,
    pub counter: u8,
    pub delay: u8,
    pub mode: u8,
    pub step: i8,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OriginalAnimationSelectors {
    pub low_object_frame: u8,
    pub high_object_frame: u8,
    pub max_frame: u8,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OriginalSpriteSelectorEntry {
    pub x_offset: i8,
    pub origin_height: i8,
    pub y_word: i16,
}

impl OriginalSpriteSelectorEntry {
    /// `FUN_1000_5a75` copies the selector entry's second byte into the active
    /// offset table, then stores `0x10 - copied_byte` in object byte `0x14`.
    pub fn position_origin_offset(self) -> i8 {
        0x10i8.wrapping_sub(self.origin_height)
    }
}

impl MonsterAnimationSeed {
    /// Advance the animation seed the way the top of `FUN_1000_6053` advances
    /// bytes `0x16..0x1c`: increment the counter, and when it exceeds the
    /// delay, add the signed step to the frame and apply the original mode
    /// bounds. Returns whether the active frame byte changed.
    pub fn advance_original_tick(&mut self) -> bool {
        self.advance_original_tick_with_mode3_backup(None)
    }

    pub fn advance_original_tick_with_mode3_backup(
        &mut self,
        mode3_backup: Option<[u8; 7]>,
    ) -> bool {
        if self.mode == 0 {
            return false;
        }
        self.counter = self.counter.wrapping_add(1);
        if self.counter <= self.delay {
            return false;
        }

        self.counter = 0;
        self.frame = self.frame.wrapping_add(self.step as u8);
        if self.mode == 2 {
            if self.frame_max <= self.frame || self.frame <= self.frame_min {
                self.step = self.step.wrapping_neg();
            }
        } else if self.frame_max < self.frame {
            self.frame = self.frame_min;
            if self.mode == 3 {
                if let Some(block) = mode3_backup {
                    *self = Self::from_original_block(block);
                }
            }
        }
        true
    }

    pub fn from_original_block(block: [u8; 7]) -> Self {
        Self {
            frame: block[0],
            frame_min: block[1],
            frame_max: block[2],
            counter: block[3],
            delay: block[4],
            mode: block[5],
            step: block[6] as i8,
        }
    }

    pub fn original_block(self) -> [u8; 7] {
        [
            self.frame,
            self.frame_min,
            self.frame_max,
            self.counter,
            self.delay,
            self.mode,
            self.step as u8,
        ]
    }

    /// Build the seven-byte block written by `FUN_1000_06ab`.
    ///
    /// The original helper stores `[min, min, max, delay, delay, mode, 1]`
    /// into the destination block, with the first byte serving as the current
    /// frame.
    pub fn from_original_setup(mode: u8, delay: u8, frame_max: u8, frame_min: u8) -> Self {
        Self {
            frame: frame_min,
            frame_min,
            frame_max,
            counter: delay,
            delay,
            mode,
            step: 1,
        }
    }

    /// Backup block built by `FUN_1000_6053` before switching to a landing
    /// animation, when no previous animation block is active.
    pub fn from_original_landing_backup(idle_frame: u8) -> Self {
        Self::from_original_setup(3, 2, idle_frame, idle_frame)
    }

    /// Active landing block built by `FUN_1000_6053`; the original passes
    /// `first_frame - 1` as the current/min frame and the supplied max frame as
    /// the upper bound.
    pub fn from_original_landing_active(first_frame: u8, frame_max: u8) -> Self {
        Self::from_original_setup(3, 3, frame_max, first_frame.wrapping_sub(1))
    }

    /// Cleanup/death animation block from the bottom of `FUN_1000_6053`.
    /// Object ids below `0x13` use the low-memory byte at `0x6a`; the rest use
    /// `0x6c`; both use `0x6d` as the max frame.
    pub fn from_original_cleanup_selector(
        object_id: u8,
        selectors: OriginalAnimationSelectors,
    ) -> Self {
        let frame_min = if object_id < 0x13 {
            selectors.low_object_frame
        } else {
            selectors.high_object_frame
        };
        Self::from_original_setup(1, 2, selectors.max_frame, frame_min)
    }
}

impl MonsterTemplate {
    pub fn runtime_motion_fields(&self) -> Option<MonsterMotionRuntimeFields> {
        self.motion_record
            .map(MonsterMotionRuntimeFields::from_copied_record)
    }

    /// Fixed-record bytes passed to `FUN_1000_5872` by the original object
    /// update path for object id `0x1f`.
    pub fn motion_sequence_ids(&self) -> [u8; 3] {
        [
            self.raw
                .get(MONSTER_TEMPLATE_MOTION_ID0_OFFSET)
                .copied()
                .unwrap_or(0),
            self.raw
                .get(MONSTER_TEMPLATE_MOTION_ID1_OFFSET)
                .copied()
                .unwrap_or(0),
            self.raw
                .get(MONSTER_TEMPLATE_MOTION_ID2_OFFSET)
                .copied()
                .unwrap_or(0),
        ]
    }

    /// State-6 objects dispatch to `FUN_1000_5cb0`, which reads the word at
    /// fixed-record offset `0x0e` as low-byte width and high-byte height in
    /// tiles while scanning the surrounding map.
    pub fn state6_collision_size_tiles(&self) -> Option<(u8, u8)> {
        if self.initial_state == 6 {
            Some((
                self.raw
                    .get(MONSTER_TEMPLATE_MOTION_ID0_OFFSET)
                    .copied()
                    .unwrap_or(0),
                self.raw
                    .get(MONSTER_TEMPLATE_MOTION_ID1_OFFSET)
                    .copied()
                    .unwrap_or(0),
            ))
        } else {
            None
        }
    }

    /// State-6 `FUN_1000_5cb0` subtracts scanned damage from fixed-record byte
    /// `0x24`, and uses fixed-record byte `0x02` as the adjacent removal/death
    /// counter when damage exceeds that budget.
    pub fn state6_damage_budget(&self) -> Option<u8> {
        if self.initial_state == 6 {
            self.raw
                .get(MONSTER_TEMPLATE_STATE6_DAMAGE_BUDGET_OFFSET)
                .copied()
        } else {
            None
        }
    }

    /// Fixed-record byte `0x24`, used as a mutable vitality/damage-budget byte
    /// by the original runtime. State-6 objects consume the same byte through
    /// `state6_damage_budget`.
    pub fn original_vitality_byte(&self) -> u8 {
        self.raw
            .get(MONSTER_TEMPLATE_ORIGINAL_VITALITY_OFFSET)
            .copied()
            .unwrap_or_default()
    }

    pub fn state6_removal_counter(&self) -> Option<u8> {
        if self.initial_state == 6 {
            self.raw
                .get(MONSTER_TEMPLATE_STATE6_REMOVAL_COUNTER_OFFSET)
                .copied()
        } else {
            None
        }
    }

    /// Fixed-record byte `0x02`, used by multiple `FUN_1000_6053` branches as
    /// a mutable countdown/removal counter.
    pub fn initial_countdown_byte(&self) -> u8 {
        self.raw
            .get(MONSTER_TEMPLATE_COUNTDOWN_OFFSET)
            .copied()
            .unwrap_or_default()
    }

    /// `FUN_1000_5bcc` reads the dying object's word at fixed-record offset
    /// `0x12` and compares it with other object-id `0x1f` records' byte `0x25`.
    pub fn state6_death_wakeup_key(&self) -> u16 {
        read_u16_field(&self.raw, MONSTER_TEMPLATE_STATE6_WAKEUP_KEY_OFFSET)
    }

    pub fn dependent_death_key(&self) -> u8 {
        self.raw
            .get(MONSTER_TEMPLATE_DEPENDENT_DEATH_KEY_OFFSET)
            .copied()
            .unwrap_or(0)
    }

    /// Signed fixed-record byte `0x14`. `FUN_1000_6053` subtracts it from the
    /// anchor-adjusted object position before dispatching behavior, then adds
    /// it back before writing the runtime position.
    pub fn position_origin_offset(&self) -> i8 {
        self.raw
            .get(MONSTER_TEMPLATE_POSITION_ORIGIN_OFFSET)
            .copied()
            .unwrap_or_default() as i8
    }

    /// In the `local_33 == 3` branch, `FUN_1000_6053` reads fixed-record byte
    /// `0x03` when X velocity is negative and byte `0x04` when it is positive,
    /// then uses that byte as an index into the animation range table at
    /// low-memory bytes `0x58/0x59`.
    pub fn horizontal_animation_selectors(&self) -> (u8, u8) {
        (
            self.raw.get(0x03).copied().unwrap_or_default(),
            self.raw.get(0x04).copied().unwrap_or_default(),
        )
    }

    /// Fixed-record movement seed words loaded by `FUN_1000_6053` before it
    /// advances the object's signed velocity/fractional accumulators.
    pub fn movement_seed(&self) -> MonsterMovementSeed {
        MonsterMovementSeed {
            x_velocity_word: read_i16_field(&self.raw, MONSTER_TEMPLATE_X_VELOCITY_OFFSET),
            y_velocity_word: read_i16_field(&self.raw, MONSTER_TEMPLATE_Y_VELOCITY_OFFSET),
            x_fraction_word: read_u16_field(&self.raw, MONSTER_TEMPLATE_X_FRACTION_OFFSET),
            y_fraction_word: read_u16_field(&self.raw, MONSTER_TEMPLATE_Y_FRACTION_OFFSET),
        }
    }

    /// Original fixed-record bytes read at the top of `FUN_1000_6053` before
    /// movement dispatch updates the active sprite pointer.
    pub fn animation_seed(&self) -> MonsterAnimationSeed {
        let mut block = [0; 7];
        for (idx, byte) in block.iter_mut().enumerate() {
            *byte = self.raw.get(0x16 + idx).copied().unwrap_or_default();
        }
        MonsterAnimationSeed::from_original_block(block)
    }

    /// Backup animation block at fixed-record bytes `0x1d..0x23`. The
    /// original mode-3 path restores these seven bytes over `0x16..0x1c`.
    pub fn animation_backup_block(&self) -> [u8; 7] {
        let mut block = [0; 7];
        for (idx, byte) in block.iter_mut().enumerate() {
            *byte = self.raw.get(0x1d + idx).copied().unwrap_or_default();
        }
        block
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MonsterMovementSeed {
    pub x_velocity_word: i16,
    pub y_velocity_word: i16,
    pub x_fraction_word: u16,
    pub y_fraction_word: u16,
}

/// Copied record fields at the addresses read by the original runtime motion
/// helper (`FUN_1000_5872`).
///
/// The loader copies each 16-byte record to `0x79ea + n * 0x10`, then adjusts
/// copied offsets 0 and 1 by the active entity-table base. `FUN_1000_432a`
/// reads offset 0 as an index into the X/Y offset table, and
/// `FUN_1000_5872` reads from `0x79ed`, `0x79f5`, and `0x79f7`, so those are
/// offsets 3, 11, and 13 within the copied record. `FUN_1000_432a` can
/// overwrite the two word slots before movement uses them, so these are the
/// initial copied values rather than always the final per-frame deltas.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MonsterMotionRuntimeFields {
    pub anchor_index: u8,
    pub secondary_anchor_index: u8,
    pub phase_step: u8,
    pub limit_or_sentinel: i8,
    pub angle_phase: u8,
    pub base_x_word: i16,
    pub base_y_word: i16,
    pub x_word: i16,
    pub y_word: i16,
    pub random_y_base: i8,
}

impl MonsterMotionRuntimeFields {
    fn from_copied_record(record: [u8; MONSTER_MOTION_RECORD_LEN]) -> Self {
        Self {
            anchor_index: record[MONSTER_MOTION_ANCHOR_INDEX_OFFSET],
            secondary_anchor_index: record[MONSTER_MOTION_SECONDARY_ANCHOR_INDEX_OFFSET],
            phase_step: record[MONSTER_MOTION_PHASE_STEP_OFFSET],
            limit_or_sentinel: record[MONSTER_MOTION_LIMIT_OFFSET] as i8,
            angle_phase: record[MONSTER_MOTION_ANGLE_PHASE_OFFSET],
            base_x_word: i16::from_le_bytes([
                record[MONSTER_MOTION_BASE_X_OFFSET],
                record[MONSTER_MOTION_BASE_X_OFFSET + 1],
            ]),
            base_y_word: i16::from_le_bytes([
                record[MONSTER_MOTION_BASE_Y_OFFSET],
                record[MONSTER_MOTION_BASE_Y_OFFSET + 1],
            ]),
            x_word: i16::from_le_bytes([
                record[MONSTER_MOTION_X_DELTA_OFFSET],
                record[MONSTER_MOTION_X_DELTA_OFFSET + 1],
            ]),
            y_word: i16::from_le_bytes([
                record[MONSTER_MOTION_Y_DELTA_OFFSET],
                record[MONSTER_MOTION_Y_DELTA_OFFSET + 1],
            ]),
            random_y_base: record[MONSTER_MOTION_RANDOM_Y_BASE_OFFSET] as i8,
        }
    }

    pub fn uses_absolute_velocity(self) -> bool {
        self.limit_or_sentinel == -1
    }

    pub fn apply_to_accumulator(
        self,
        accumulator: &mut MonsterMotionAccumulator,
        reverse: bool,
    ) -> Option<(i16, i16)> {
        if self.uses_absolute_velocity() {
            accumulator.x = 0;
            accumulator.y = 0;
            Some((self.x_word, self.y_word))
        } else {
            let direction = if reverse { -1 } else { 1 };
            let limit = self.limit_or_sentinel as i16;
            accumulator.x = wrap_motion_accumulator(accumulator.x + self.x_word * direction, limit);
            accumulator.y = wrap_motion_accumulator(accumulator.y + self.y_word * direction, limit);
            None
        }
    }

    pub fn with_random_preprocess(self, random_x: i16, random_y: i16) -> Self {
        if self.uses_absolute_velocity() {
            self
        } else {
            Self {
                x_word: random_x,
                y_word: self.random_y_base as i16 + random_y,
                ..self
            }
        }
    }

    pub fn with_advanced_phase(self) -> Self {
        if self.uses_absolute_velocity() {
            Self {
                angle_phase: self.angle_phase.wrapping_add(self.phase_step) & 0x7f,
                ..self
            }
        } else {
            self
        }
    }

    pub fn absolute_trig_phase_indices_after_advance(self) -> Option<(u8, u8)> {
        if self.uses_absolute_velocity() {
            let phase = self.with_advanced_phase().angle_phase;
            Some((phase.wrapping_add(0x20) & 0x7f, phase))
        } else {
            None
        }
    }

    pub fn with_absolute_preprocess(self, anchor: (i16, i16), trig_x: i16, trig_y: i16) -> Self {
        if self.uses_absolute_velocity() {
            let advanced = self.with_advanced_phase();
            Self {
                x_word: anchor.0 + advanced.base_x_word + trig_x,
                y_word: anchor.1 + advanced.base_y_word + trig_y,
                ..advanced
            }
        } else {
            self
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MonsterMotionAccumulator {
    pub x: i16,
    pub y: i16,
}

fn wrap_motion_accumulator(value: i16, limit: i16) -> i16 {
    if limit < value.abs() {
        if value < 1 {
            value + limit
        } else {
            value - limit
        }
    } else {
        value
    }
}

const MONSTER_TEMPLATE_RECORD_LEN: usize = 38;
const MONSTER_TEMPLATE_MOTION_ID0_OFFSET: usize = 0x0e;
const MONSTER_TEMPLATE_MOTION_ID1_OFFSET: usize = 0x0f;
const MONSTER_TEMPLATE_MOTION_ID2_OFFSET: usize = 0x10;
const MONSTER_TEMPLATE_X_VELOCITY_OFFSET: usize = 0x06;
const MONSTER_TEMPLATE_Y_VELOCITY_OFFSET: usize = 0x08;
const MONSTER_TEMPLATE_X_FRACTION_OFFSET: usize = 0x0a;
const MONSTER_TEMPLATE_Y_FRACTION_OFFSET: usize = 0x0c;
const MONSTER_TEMPLATE_INITIAL_STATE_OFFSET: usize = 0x15;
const MONSTER_TEMPLATE_POSITION_ORIGIN_OFFSET: usize = 0x14;
const MONSTER_TEMPLATE_COUNTDOWN_OFFSET: usize = 0x02;
const MONSTER_TEMPLATE_STATE6_REMOVAL_COUNTER_OFFSET: usize = MONSTER_TEMPLATE_COUNTDOWN_OFFSET;
const MONSTER_TEMPLATE_ORIGINAL_VITALITY_OFFSET: usize = 0x24;
const MONSTER_TEMPLATE_STATE6_DAMAGE_BUDGET_OFFSET: usize =
    MONSTER_TEMPLATE_ORIGINAL_VITALITY_OFFSET;
const MONSTER_TEMPLATE_STATE6_WAKEUP_KEY_OFFSET: usize = 0x12;
const MONSTER_TEMPLATE_DEPENDENT_DEATH_KEY_OFFSET: usize = 0x25;
const MONSTER_ANCHOR_RECORD_LEN: usize = 4;
const MONSTER_MOTION_RECORD_LEN: usize = 16;
const MONSTER_MOTION_ANCHOR_INDEX_OFFSET: usize = 0;
const MONSTER_MOTION_SECONDARY_ANCHOR_INDEX_OFFSET: usize = 1;
const MONSTER_MOTION_PHASE_STEP_OFFSET: usize = 2;
const MONSTER_MOTION_LIMIT_OFFSET: usize = 3;
const MONSTER_MOTION_ANGLE_PHASE_OFFSET: usize = 6;
const MONSTER_MOTION_BASE_X_OFFSET: usize = 7;
const MONSTER_MOTION_BASE_Y_OFFSET: usize = 9;
const MONSTER_MOTION_X_DELTA_OFFSET: usize = 11;
const MONSTER_MOTION_Y_DELTA_OFFSET: usize = 13;
const MONSTER_MOTION_RANDOM_Y_BASE_OFFSET: usize = 15;

fn read_u16_field(bytes: &[u8], offset: usize) -> u16 {
    bytes
        .get(offset..offset + 2)
        .map(|field| u16::from_le_bytes([field[0], field[1]]))
        .unwrap_or(0)
}

fn read_i16_field(bytes: &[u8], offset: usize) -> i16 {
    read_u16_field(bytes, offset) as i16
}

pub fn load_monster_defs(path: &str) -> Vec<MonsterTemplate> {
    let d = match crate::assets::try_load_file(path) {
        Some(d) => d,
        None => return default_templates(),
    };
    if d.is_empty() {
        return default_templates();
    }
    let count = d[0] as usize;
    if count == 0 {
        return default_templates();
    }
    let records_len = match count.checked_mul(MONSTER_TEMPLATE_RECORD_LEN) {
        Some(len) => len,
        None => return default_templates(),
    };
    if d.len() < 1 + records_len {
        return default_templates();
    }
    let trailing = &d[1 + records_len..];
    let anchor_start = count;
    let motion_count_offset = anchor_start + count * MONSTER_ANCHOR_RECORD_LEN;
    let motion_count = trailing.get(motion_count_offset).copied().unwrap_or(0) as usize;
    let motion_start = motion_count_offset + 1;
    let anchor_offsets = (0..count)
        .map(|i| {
            trailing
                .get(
                    anchor_start + i * MONSTER_ANCHOR_RECORD_LEN
                        ..anchor_start + (i + 1) * MONSTER_ANCHOR_RECORD_LEN,
                )
                .map(|bytes| {
                    (
                        i16::from_le_bytes([bytes[0], bytes[1]]),
                        i16::from_le_bytes([bytes[2], bytes[3]]),
                    )
                })
        })
        .collect::<Vec<_>>();
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let s = 1 + i * MONSTER_TEMPLATE_RECORD_LEN;
        let e = s + MONSTER_TEMPLATE_RECORD_LEN;
        let raw = d[s..e].to_vec();
        let sprite_base = trailing
            .get(i)
            .copied()
            .or_else(|| raw.get(3).copied())
            .unwrap_or(0x10 + i as u8 * 3);
        let anchor_table_index = raw.get(1).copied().unwrap_or(0);
        let anchor_offset = anchor_offsets
            .get(anchor_table_index as usize)
            .copied()
            .flatten();
        let motion_record = trailing
            .get(
                motion_start + i * MONSTER_MOTION_RECORD_LEN
                    ..motion_start + (i + 1) * MONSTER_MOTION_RECORD_LEN,
            )
            .filter(|_| i < motion_count)
            .map(|bytes| {
                let mut record = [0u8; MONSTER_MOTION_RECORD_LEN];
                record.copy_from_slice(bytes);
                record
            });
        let object_id = raw.first().copied().unwrap_or(0);
        let speed = raw.get(1).copied().unwrap_or(2);
        let record_byte_05 = raw.get(5).copied().unwrap_or(0);
        let initial_state = raw
            .get(MONSTER_TEMPLATE_INITIAL_STATE_OFFSET)
            .copied()
            .unwrap_or(0);
        let damage = record_byte_05;
        let flags = 0;
        out.push(MonsterTemplate {
            object_id,
            anchor_table_index,
            sprite_base,
            speed,
            record_byte_05,
            initial_state,
            damage,
            flags,
            raw,
            anchor_offset,
            motion_record,
        });
    }
    if out.is_empty() {
        default_templates()
    } else {
        out
    }
}

fn default_templates() -> Vec<MonsterTemplate> {
    (0..7)
        .map(|i| MonsterTemplate {
            object_id: 0,
            anchor_table_index: 0,
            sprite_base: 0x10 + i * 3,
            speed: 2,
            record_byte_05: 0,
            initial_state: 0,
            damage: 8,
            flags: 0,
            raw: vec![],
            anchor_offset: None,
            motion_record: None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn livels_sch_matches_original_level_summary() {
        let levels = load_levels("assets/LIVELS.SCH");
        let expected = [
            (60, 33, 108, 1, 50, 1, 2, 0),
            (100, 53, 110, 3, 60, 2, 2, 0),
            (150, 60, 108, 7, 20, 3, 3, 0),
            (100, 58, 109, 3, 70, 2, 4, 0),
            (110, 62, 107, 8, 65, 3, 5, 0),
            (180, 64, 111, 8, 40, 4, 3, 2),
            (140, 52, 106, 1, 10, 0, 2, 1),
        ];

        assert_eq!(levels.len(), expected.len());
        for (
            level,
            (
                width,
                height,
                destruction_tile,
                bonus_target,
                destruction_pct,
                monsters,
                bonuses,
                platforms,
            ),
        ) in levels.iter().zip(expected)
        {
            assert_eq!(level.width, width);
            assert_eq!(level.height, height);
            assert_eq!(level.destruction_tile, destruction_tile);
            assert_eq!(level.bonus_target, bonus_target);
            assert_eq!(level.destruction_pct, destruction_pct);
            assert_eq!(level.monsters.len(), monsters);
            assert_eq!(level.bonuses.len(), bonuses);
            assert_eq!(level.platforms.len(), platforms);
        }
    }

    #[test]
    fn gran_mst_matches_original_template_count_and_sprite_bases() {
        let bytes = std::fs::read("assets/GRAN.MST").unwrap();
        let templates = load_monster_defs("assets/GRAN.MST");
        let object_ids: Vec<u8> = templates.iter().map(|t| t.object_id).collect();
        let anchor_indices: Vec<u8> = templates.iter().map(|t| t.anchor_table_index).collect();
        let initial_states: Vec<u8> = templates.iter().map(|t| t.initial_state).collect();
        let sprite_bases: Vec<u8> = templates.iter().map(|t| t.sprite_base).collect();

        let records_len = 1 + 7 * MONSTER_TEMPLATE_RECORD_LEN;
        let trailing = &bytes[records_len..];
        assert_eq!(templates.len(), 7);
        assert_eq!(
            trailing.len(),
            7 + 7 * MONSTER_ANCHOR_RECORD_LEN + 1 + 6 * MONSTER_MOTION_RECORD_LEN
        );
        assert_eq!(object_ids, vec![0x1e, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f]);
        assert_eq!(anchor_indices, vec![4, 5, 6, 3, 2, 1, 0]);
        assert_eq!(initial_states, vec![6, 5, 5, 5, 5, 5, 5]);
        assert_eq!(sprite_bases, vec![0x2e, 0x2e, 0x2d, 0x2d, 0x28, 0x2a, 0x2b]);
        assert_eq!(
            templates
                .iter()
                .map(|t| t.anchor_offset.unwrap())
                .collect::<Vec<_>>(),
            vec![
                (0, 0),
                (-8, 18),
                (39, 17),
                (-12, 16),
                (-7, 10),
                (42, 4),
                (38, 10)
            ]
        );
        assert!(templates
            .iter()
            .all(|t| t.raw.len() == MONSTER_TEMPLATE_RECORD_LEN));
        assert_eq!(
            templates
                .iter()
                .filter(|t| t.motion_record.is_some())
                .count(),
            6
        );
        assert_eq!(
            templates[0].motion_record.unwrap(),
            [
                0x04, 0x05, 0x0f, 0x52, 0x63, 0x00, 0x09, 0xfc, 0xff, 0x0e, 0x00, 0x0a, 0x6d, 0x00,
                0x09, 0x0a,
            ]
        );
    }

    #[test]
    fn gran_mst_motion_fields_match_original_runtime_offsets() {
        let templates = load_monster_defs("assets/GRAN.MST");

        assert_eq!(
            templates
                .iter()
                .map(MonsterTemplate::motion_sequence_ids)
                .collect::<Vec<_>>(),
            vec![
                [5, 4, 3],
                [1, 0, 0],
                [2, 0, 0],
                [3, 0, 0],
                [4, 0, 0],
                [5, 0, 0],
                [6, 0, 0]
            ]
        );
        assert_eq!(templates[0].state6_collision_size_tiles(), Some((5, 4)));
        assert_eq!(templates[0].state6_damage_budget(), Some(0x0a));
        assert_eq!(
            templates
                .iter()
                .map(MonsterTemplate::original_vitality_byte)
                .collect::<Vec<_>>(),
            vec![0x0a, 0xff, 0x06, 0xa8, 0x01, 0x04, 0x02]
        );
        assert_eq!(templates[0].state6_removal_counter(), Some(0x01));
        assert_eq!(
            templates
                .iter()
                .map(MonsterTemplate::initial_countdown_byte)
                .collect::<Vec<_>>(),
            vec![0x01, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04]
        );
        assert_eq!(
            templates
                .iter()
                .map(|template| template.record_byte_05)
                .collect::<Vec<_>>(),
            vec![0x00, 0x02, 0x06, 0x06, 0xbf, 0x00, 0x02]
        );
        assert_eq!(templates[0].state6_death_wakeup_key(), 0x0004);
        assert_eq!(
            templates
                .iter()
                .map(MonsterTemplate::dependent_death_key)
                .collect::<Vec<_>>(),
            vec![0, 6, 2, 1, 0, 0xb8, 0]
        );
        assert_eq!(
            templates
                .iter()
                .map(MonsterTemplate::position_origin_offset)
                .collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            templates
                .iter()
                .map(MonsterTemplate::horizontal_animation_selectors)
                .collect::<Vec<_>>(),
            vec![
                (0x10, 0x04),
                (0x0e, 0x01),
                (0x0f, 0xba),
                (0x00, 0xff),
                (0x00, 0x04),
                (0x00, 0x0e),
                (0x00, 0x50),
            ]
        );
        assert!(templates[1..]
            .iter()
            .all(|template| template.state6_collision_size_tiles().is_none()));
        assert!(templates[1..]
            .iter()
            .all(|template| template.state6_damage_budget().is_none()));
        assert!(templates[1..]
            .iter()
            .all(|template| template.state6_removal_counter().is_none()));
        assert!(templates
            .iter()
            .map(MonsterTemplate::movement_seed)
            .all(|seed| seed
                == MonsterMovementSeed {
                    x_velocity_word: 0,
                    y_velocity_word: 0,
                    x_fraction_word: 0,
                    y_fraction_word: 0,
                }));
        assert_eq!(
            templates
                .iter()
                .map(MonsterTemplate::animation_seed)
                .collect::<Vec<_>>(),
            vec![
                MonsterAnimationSeed {
                    frame: 0x06,
                    frame_min: 0x12,
                    frame_max: 0xa5,
                    counter: 0x01,
                    delay: 0x09,
                    mode: 0x00,
                    step: 0x1c,
                },
                MonsterAnimationSeed {
                    frame: 0x31,
                    frame_min: 0x0a,
                    frame_max: 0x75,
                    counter: 0x9d,
                    delay: 0x02,
                    mode: 0x00,
                    step: 0x1b,
                },
                MonsterAnimationSeed {
                    frame: 0x09,
                    frame_min: 0x04,
                    frame_max: 0x01,
                    counter: 0x02,
                    delay: 0x03,
                    mode: 0x00,
                    step: 0x06,
                },
                MonsterAnimationSeed {
                    frame: 0x0e,
                    frame_min: 0x09,
                    frame_max: 0x90,
                    counter: 0x00,
                    delay: 0x00,
                    mode: 0x00,
                    step: 0x09,
                },
                MonsterAnimationSeed {
                    frame: 0x50,
                    frame_min: 0x02,
                    frame_max: 0x00,
                    counter: 0x10,
                    delay: 0x00,
                    mode: 0x00,
                    step: 0x1e,
                },
                MonsterAnimationSeed {
                    frame: 0xd0,
                    frame_min: 0x01,
                    frame_max: 0x00,
                    counter: 0x00,
                    delay: 0x00,
                    mode: 0x00,
                    step: 0x0e,
                },
                MonsterAnimationSeed {
                    frame: 0x0e,
                    frame_min: 0x09,
                    frame_max: 0x00,
                    counter: 0x80,
                    delay: 0x00,
                    mode: 0x00,
                    step: 0x00,
                },
            ]
        );
        assert_eq!(
            templates
                .iter()
                .map(MonsterTemplate::animation_backup_block)
                .collect::<Vec<_>>(),
            vec![
                [0x04, 0x00, 0x76, 0x06, 0x0c, 0x55, 0x1b],
                [0xa1, 0x06, 0x02, 0x06, 0x8a, 0x46, 0x04],
                [0x0a, 0x76, 0x91, 0x07, 0x84, 0x1b, 0xb2],
                [0x06, 0x88, 0x46, 0x04, 0xfb, 0x02, 0x12],
                [0x07, 0x04, 0x9a, 0x0e, 0x00, 0x30, 0xb0],
                [0x00, 0x30, 0x90, 0x01, 0x00, 0x00, 0x02],
                [0x02, 0x12, 0xa9, 0x01, 0x00, 0x66, 0x07],
            ]
        );

        let first = templates[0].runtime_motion_fields().unwrap();
        assert_eq!(
            first,
            MonsterMotionRuntimeFields {
                anchor_index: 4,
                secondary_anchor_index: 5,
                phase_step: 0x0f,
                limit_or_sentinel: 0x52,
                angle_phase: 0x09,
                base_x_word: -4,
                base_y_word: 14,
                x_word: 0x6d0a,
                y_word: 0x0900,
                random_y_base: 10,
            }
        );
        assert!(!first.uses_absolute_velocity());

        let records = templates
            .iter()
            .filter_map(MonsterTemplate::runtime_motion_fields)
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 6);
        assert_eq!(
            records
                .iter()
                .map(|r| {
                    (
                        r.anchor_index,
                        r.secondary_anchor_index,
                        r.phase_step,
                        r.limit_or_sentinel,
                        r.angle_phase,
                        r.base_x_word,
                        r.base_y_word,
                        r.x_word,
                        r.y_word,
                        r.random_y_base,
                    )
                })
                .collect::<Vec<_>>(),
            vec![
                (4, 5, 0x0f, 0x52, 0x09, -4, 14, 0x6d0a, 0x0900, 10),
                (4, 6, 0x12, 0x5c, 0x09, 39, 15, 0x750a, 0x0900, 12),
                (4, 3, 0x03, -1, 0x20, -12, 10, 0x7d0a, 0x0900, 7),
                (4, 2, 0x03, -1, 0x42, -7, 10, -0x7af6, 0, 7),
                (4, 1, 0x03, -1, 0x61, 42, 10, 0x120a, 0x0900, 7),
                (4, 0, 0x03, -1, 0x02, 38, 10, 0x6a00, 0x0900, 7),
            ]
        );
    }

    #[test]
    fn monster_animation_seed_advances_like_fun_1000_6053_counter_gate() {
        let mut animation = MonsterAnimationSeed {
            frame: 4,
            frame_min: 2,
            frame_max: 6,
            counter: 0,
            delay: 1,
            mode: 1,
            step: 1,
        };

        assert!(!animation.advance_original_tick());
        assert_eq!(animation.frame, 4);
        assert_eq!(animation.counter, 1);

        assert!(animation.advance_original_tick());
        assert_eq!(animation.frame, 5);
        assert_eq!(animation.counter, 0);
    }

    #[test]
    fn monster_animation_seed_wraps_and_bounces_by_original_modes() {
        let mut wrapping = MonsterAnimationSeed {
            frame: 6,
            frame_min: 2,
            frame_max: 6,
            counter: 0,
            delay: 0,
            mode: 1,
            step: 1,
        };
        assert!(wrapping.advance_original_tick());
        assert_eq!(wrapping.frame, 2);
        assert_eq!(wrapping.step, 1);

        let mut bouncing = MonsterAnimationSeed {
            frame: 5,
            frame_min: 2,
            frame_max: 6,
            counter: 0,
            delay: 0,
            mode: 2,
            step: 1,
        };
        assert!(bouncing.advance_original_tick());
        assert_eq!(bouncing.frame, 6);
        assert_eq!(bouncing.step, -1);

        assert!(bouncing.advance_original_tick());
        assert_eq!(bouncing.frame, 5);
        assert_eq!(bouncing.step, -1);
    }

    #[test]
    fn monster_animation_seed_mode3_restores_original_backup_block() {
        let mut animation = MonsterAnimationSeed {
            frame: 6,
            frame_min: 2,
            frame_max: 6,
            counter: 0,
            delay: 0,
            mode: 3,
            step: 1,
        };
        let backup = [0x22, 0x20, 0x24, 0x05, 0x02, 0x02, 0xff];

        assert!(animation.advance_original_tick_with_mode3_backup(Some(backup)));
        assert_eq!(animation.original_block(), backup);
        assert_eq!(animation.step, -1);
    }

    #[test]
    fn monster_animation_seed_setup_matches_fun_1000_06ab_layout() {
        let animation = MonsterAnimationSeed::from_original_setup(3, 2, 0x12, 0x11);

        assert_eq!(
            animation,
            MonsterAnimationSeed {
                frame: 0x11,
                frame_min: 0x11,
                frame_max: 0x12,
                counter: 2,
                delay: 2,
                mode: 3,
                step: 1,
            }
        );
        assert_eq!(animation.original_block(), [0x11, 0x11, 0x12, 2, 2, 3, 1]);
    }

    #[test]
    fn monster_animation_seed_landing_helpers_match_fun_1000_6053_calls() {
        let p1_backup = MonsterAnimationSeed::from_original_landing_backup(0x01);
        assert_eq!(p1_backup.original_block(), [1, 1, 1, 2, 2, 3, 1]);

        let p1_hard_landing = MonsterAnimationSeed::from_original_landing_active(0x12, 0x12);
        assert_eq!(
            p1_hard_landing.original_block(),
            [0x11, 0x11, 0x12, 3, 3, 3, 1]
        );

        let p1_soft_landing = MonsterAnimationSeed::from_original_landing_active(0x12, 0x13);
        assert_eq!(
            p1_soft_landing.original_block(),
            [0x11, 0x11, 0x13, 3, 3, 3, 1]
        );

        let p2_soft_landing = MonsterAnimationSeed::from_original_landing_active(0x25, 0x26);
        assert_eq!(
            p2_soft_landing.original_block(),
            [0x24, 0x24, 0x26, 3, 3, 3, 1]
        );
    }

    #[test]
    fn monster_animation_seed_cleanup_selector_matches_fun_1000_6053_low_memory_branch() {
        let selectors = OriginalAnimationSelectors {
            low_object_frame: 0x6a,
            high_object_frame: 0x6c,
            max_frame: 0x6d,
        };

        assert_eq!(
            MonsterAnimationSeed::from_original_cleanup_selector(0x0e, selectors).original_block(),
            [0x6a, 0x6a, 0x6d, 2, 2, 1, 1]
        );
        assert_eq!(
            MonsterAnimationSeed::from_original_cleanup_selector(0x12, selectors).original_block(),
            [0x6a, 0x6a, 0x6d, 2, 2, 1, 1]
        );
        assert_eq!(
            MonsterAnimationSeed::from_original_cleanup_selector(0x13, selectors).original_block(),
            [0x6c, 0x6c, 0x6d, 2, 2, 1, 1]
        );
    }

    #[test]
    fn original_cleanup_selector_constants_match_lezac_exe_low_memory_table() {
        assert_eq!(
            MonsterAnimationSeed::from_original_cleanup_selector(
                0x12,
                ORIGINAL_CLEANUP_ANIMATION_SELECTORS
            )
            .original_block(),
            [0x45, 0x45, 0x4f, 2, 2, 1, 1]
        );
        assert_eq!(
            MonsterAnimationSeed::from_original_cleanup_selector(
                0x13,
                ORIGINAL_CLEANUP_ANIMATION_SELECTORS
            )
            .original_block(),
            [0x4a, 0x4a, 0x4f, 2, 2, 1, 1]
        );
    }

    #[test]
    fn sprite_selector_entry_matches_fun_1000_5a75_origin_offset_shape() {
        let entry = OriginalSpriteSelectorEntry {
            x_offset: -4,
            origin_height: 12,
            y_word: 0x1234,
        };

        assert_eq!(entry.position_origin_offset(), 4);

        let wrapping_entry = OriginalSpriteSelectorEntry {
            origin_height: -120,
            ..entry
        };
        assert_eq!(wrapping_entry.position_origin_offset(), -120);
    }

    #[test]
    fn monster_motion_fields_apply_original_accumulator_rules() {
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
        let mut accumulator = MonsterMotionAccumulator::default();

        assert_eq!(bounded.apply_to_accumulator(&mut accumulator, false), None);
        assert_eq!(accumulator, MonsterMotionAccumulator { x: 2, y: -2 });

        assert_eq!(bounded.apply_to_accumulator(&mut accumulator, true), None);
        assert_eq!(accumulator, MonsterMotionAccumulator { x: -10, y: 10 });

        let absolute = MonsterMotionRuntimeFields {
            limit_or_sentinel: -1,
            x_word: -7,
            y_word: 11,
            ..bounded
        };
        accumulator = MonsterMotionAccumulator { x: 5, y: 6 };

        assert_eq!(
            absolute.apply_to_accumulator(&mut accumulator, false),
            Some((-7, 11))
        );
        assert_eq!(accumulator, MonsterMotionAccumulator::default());
    }

    #[test]
    fn monster_motion_fields_preprocess_random_branch() {
        let bounded = MonsterMotionRuntimeFields {
            anchor_index: 0,
            secondary_anchor_index: 0,
            phase_step: 0,
            limit_or_sentinel: 10,
            angle_phase: 0,
            base_x_word: 0,
            base_y_word: 0,
            x_word: 1,
            y_word: 2,
            random_y_base: 7,
        };
        let preprocessed = bounded.with_random_preprocess(123, -5);

        assert_eq!(preprocessed.x_word, 123);
        assert_eq!(preprocessed.y_word, 2);

        let absolute = MonsterMotionRuntimeFields {
            limit_or_sentinel: -1,
            x_word: 10,
            y_word: 20,
            ..bounded
        };
        assert_eq!(absolute.with_random_preprocess(123, -5), absolute);
    }

    #[test]
    fn monster_motion_fields_advance_absolute_phase() {
        let absolute = MonsterMotionRuntimeFields {
            anchor_index: 0,
            secondary_anchor_index: 0,
            phase_step: 0x0f,
            limit_or_sentinel: -1,
            angle_phase: 0x75,
            base_x_word: 0,
            base_y_word: 0,
            x_word: 0,
            y_word: 0,
            random_y_base: 0,
        };
        assert_eq!(absolute.with_advanced_phase().angle_phase, 0x04);
        assert_eq!(
            absolute.absolute_trig_phase_indices_after_advance(),
            Some((0x24, 0x04))
        );

        let bounded = MonsterMotionRuntimeFields {
            limit_or_sentinel: 10,
            ..absolute
        };
        assert_eq!(bounded.with_advanced_phase(), bounded);
        assert_eq!(bounded.absolute_trig_phase_indices_after_advance(), None);
    }

    #[test]
    fn monster_motion_fields_preprocess_absolute_branch_with_injected_trig() {
        let absolute = MonsterMotionRuntimeFields {
            anchor_index: 0,
            secondary_anchor_index: 0,
            phase_step: 0x03,
            limit_or_sentinel: -1,
            angle_phase: 0x7e,
            base_x_word: -4,
            base_y_word: 6,
            x_word: 0,
            y_word: 0,
            random_y_base: 0,
        };
        let preprocessed = absolute.with_absolute_preprocess((100, 200), 7, -9);

        assert_eq!(preprocessed.angle_phase, 0x01);
        assert_eq!(preprocessed.x_word, 103);
        assert_eq!(preprocessed.y_word, 197);

        let bounded = MonsterMotionRuntimeFields {
            limit_or_sentinel: 10,
            ..absolute
        };
        assert_eq!(bounded.with_absolute_preprocess((100, 200), 7, -9), bounded);
    }

    #[test]
    fn livels_monster_spawns_use_in_bounds_pixel_coordinates() {
        let levels = load_levels("assets/LIVELS.SCH");

        for (level_idx, level) in levels.into_iter().enumerate() {
            for (spawn_idx, spawn) in level.monsters.into_iter().enumerate() {
                let x = spawn.x_px() as usize;
                let y = spawn.y_px() as usize;
                assert!(
                    x < level.width * 8,
                    "level {level_idx} spawn {spawn_idx} x {x} out of {} raw {:?}",
                    level.width * 8,
                    spawn.raw
                );
                assert!(
                    y < level.height * 8,
                    "level {level_idx} spawn {spawn_idx} y {y} out of {} raw {:?}",
                    level.height * 8,
                    spawn.raw
                );
            }
        }
    }

    #[test]
    fn livels_monster_spawn_controller_fields_match_original_loader_shape() {
        let levels = load_levels("assets/LIVELS.SCH");
        let spawns = levels
            .iter()
            .flat_map(|level| level.monsters.iter().copied())
            .collect::<Vec<_>>();

        assert_eq!(spawns.len(), 15);
        assert!(spawns.iter().all(|spawn| spawn.original_active_flag() == 1));
        assert_eq!(
            spawns
                .iter()
                .map(|spawn| spawn.original_template_selector())
                .collect::<Vec<_>>(),
            vec![1, 2, 1, 4, 2, 1, 3, 1, 1, 3, 3, 1, 4, 4, 1]
        );
        assert_eq!(
            spawns
                .iter()
                .map(|spawn| {
                    (
                        spawn.original_spawn_count(),
                        spawn.original_spawn_budget(),
                        spawn.original_spawn_allocation_param(),
                        spawn.original_spawn_timer(),
                        spawn.original_spawn_timer_reset(),
                        spawn.original_spawn_animation_delay(),
                    )
                })
                .collect::<Vec<_>>(),
            vec![
                (3, 2, 3, 0, 0x5a, 3),
                (1, 1, 4, 0, 0x3c, 2),
                (4, 2, 3, 0, 0x64, 3),
                (2, 1, 3, 0, 0x1e, 2),
                (4, 2, 4, 0, 0x28, 2),
                (4, 2, 3, 0, 0x3c, 2),
                (3, 2, 4, 0, 0x46, 2),
                (1, 1, 3, 0, 0x32, 2),
                (3, 2, 3, 0, 0x3c, 2),
                (6, 2, 4, 0, 0x50, 2),
                (2, 1, 4, 0, 0x50, 2),
                (6, 3, 3, 0, 0x46, 2),
                (9, 3, 4, 0, 0x3c, 2),
                (2, 2, 3, 0, 0x3c, 2),
                (4, 2, 3, 0, 0x3c, 2),
            ]
        );
        assert_eq!(
            spawns
                .iter()
                .map(|spawn| {
                    (
                        spawn.original_runtime_word_0x0e_base(),
                        spawn.original_runtime_word_0x0e_random_modulus(),
                        spawn.original_runtime_word_0x10_base(),
                        spawn.original_runtime_word_0x10_random_modulus(),
                        spawn.original_runtime_word_0x12_base(),
                        spawn.original_runtime_word_0x12_random_modulus(),
                    )
                })
                .collect::<Vec<_>>(),
            vec![
                (200, 101, 0, 1, 0, 1),
                (13, 2, 270, 2, 60, 11),
                (130, 151, 0, 1, 0, 1),
                (200, 301, 0, 1, 0, 1),
                (10, 13, 200, 301, 60, 51),
                (200, 221, 0, 1, 0, 1),
                (10, 9, 200, 351, 50, 41),
                (300, 1, 0, 1, 0, 1),
                (240, 61, 0, 1, 0, 1),
                (10, 8, 240, 261, 60, 51),
                (10, 6, 240, 261, 60, 51),
                (200, 291, 0, 1, 0, 1),
                (11, 13, 300, 151, 50, 61),
                (200, 251, 0, 1, 0, 1),
                (240, 111, 0, 1, 0, 1),
            ]
        );
        assert_eq!(
            spawns
                .iter()
                .map(|spawn| {
                    (
                        spawn.original_health_base(),
                        spawn.original_health_random_modulus(),
                    )
                })
                .collect::<Vec<_>>(),
            vec![
                (1, 1),
                (3, 1),
                (2, 2),
                (3, 2),
                (2, 3),
                (3, 2),
                (2, 2),
                (3, 1),
                (5, 2),
                (2, 3),
                (2, 3),
                (2, 2),
                (3, 5),
                (5, 6),
                (4, 2),
            ]
        );
    }

    #[test]
    fn monster_spawn_runtime_fields_use_original_rng_moduli() {
        let mut raw = [0; 30];
        raw[0x0c..0x0e].copy_from_slice(&0xfffe_u16.to_le_bytes());
        raw[0x0e..0x10].copy_from_slice(&4_u16.to_le_bytes());
        raw[0x10..0x12].copy_from_slice(&10_u16.to_le_bytes());
        raw[0x12..0x14].copy_from_slice(&0_u16.to_le_bytes());
        raw[0x14..0x16].copy_from_slice(&100_u16.to_le_bytes());
        raw[0x16..0x18].copy_from_slice(&5_u16.to_le_bytes());
        raw[0x18] = 250;
        raw[0x19] = 10;

        let spawn = MonsterSpawn { raw };
        let seed = 0x2468_ace0;
        let mut expected_rng = OriginalRng::new(seed);
        let expected = OriginalSpawnRuntimeFields {
            word_0x0e: 0xfffe_u16.wrapping_add(expected_rng.gen_mod(4)),
            word_0x10: 10,
            word_0x12: 100_u16.wrapping_add(expected_rng.gen_mod(5)),
            vitality: 250_u8.wrapping_add(expected_rng.gen_mod(10) as u8),
        };

        let mut rng = OriginalRng::new(seed);
        assert_eq!(spawn.original_spawn_runtime_fields(&mut rng), expected);
        assert_eq!(rng.seed(), expected_rng.seed());
    }

    #[test]
    fn monster_spawn_controller_tick_matches_original_gates() {
        let mut raw = [0; 30];
        raw[0x08] = 1;
        raw[0x09] = 2;
        raw[0x0a] = 3;
        raw[0x1b] = 1;
        raw[0x1c] = 0x3c;
        let mut spawn = MonsterSpawn { raw };

        assert!(spawn.original_tick_spawn_controller());
        assert_eq!(spawn.original_spawn_timer(), 0x3c);
        assert_eq!(spawn.original_spawn_count(), 2);
        assert_eq!(spawn.original_spawn_budget(), 3);

        spawn.original_commit_spawn_success();
        assert_eq!(spawn.original_spawn_count(), 1);
        assert_eq!(spawn.original_spawn_budget(), 2);

        spawn.raw[0x1b] = 0;
        assert!(!spawn.original_tick_spawn_controller());
        assert_eq!(spawn.original_spawn_timer(), 0xff);

        spawn.raw[0x1b] = 1;
        spawn.raw[0x09] = 0;
        assert!(!spawn.original_tick_spawn_controller());
        assert_eq!(spawn.original_spawn_timer(), 0);
    }

    #[test]
    fn monster_spawn_controller_event_combines_original_timer_tables_and_rng() {
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
        let mut spawn = MonsterSpawn { raw };

        let seed = 0x0bad_cafe;
        let mut expected_rng = OriginalRng::new(seed);
        let expected_runtime = OriginalSpawnRuntimeFields {
            word_0x0e: 10 + expected_rng.gen_mod(5),
            word_0x10: 20 + expected_rng.gen_mod(7),
            word_0x12: 30 + expected_rng.gen_mod(11),
            vitality: 4 + expected_rng.gen_mod(3) as u8,
        };

        let mut rng = OriginalRng::new(seed);
        let event = spawn.advance_original_spawn_controller(&mut rng).unwrap();

        assert_eq!(event.runtime_fields, expected_runtime);
        assert_eq!(rng.seed(), expected_rng.seed());
        assert_eq!(spawn.original_spawn_timer(), 0x3c);
        assert_eq!(spawn.original_spawn_count(), 1);
        assert_eq!(spawn.original_spawn_budget(), 2);
        assert_eq!(
            event.allocation_request,
            OriginalSpawnAllocationRequest {
                x_px: 0x0150,
                y_px: 0x00a8,
                template_selector: 4,
                allocation_param: 3,
                selector_table_byte_0x80: 0x0d,
                selector_table_byte_0x81: 0x0d,
                animation_seed: MonsterAnimationSeed::from_original_setup(1, 2, 0x38, 0x36),
            }
        );

        assert!(spawn.advance_original_spawn_controller(&mut rng).is_none());
        assert_eq!(rng.seed(), expected_rng.seed());
    }

    #[test]
    fn monster_spawn_allocation_request_uses_injected_original_tables() {
        let mut raw = [0; 30];
        raw[0x00..0x02].copy_from_slice(&0x0150_u16.to_le_bytes());
        raw[0x02..0x04].copy_from_slice(&0x00a8_u16.to_le_bytes());
        raw[0x0b] = 4;
        raw[0x1a] = 3;
        raw[0x1d] = 2;
        let spawn = MonsterSpawn { raw };

        assert_eq!(
            spawn.original_spawn_allocation_request((0x1f, 0x0b), (0x12, 0x1a)),
            OriginalSpawnAllocationRequest {
                x_px: 0x0150,
                y_px: 0x00a8,
                template_selector: 4,
                allocation_param: 3,
                selector_table_byte_0x80: 0x1f,
                selector_table_byte_0x81: 0x0b,
                animation_seed: MonsterAnimationSeed::from_original_setup(1, 2, 0x1a, 0x12),
            }
        );
    }

    #[test]
    fn monster_spawn_allocation_request_maps_to_fun_1000_2f9f_call_shape() {
        let request = OriginalSpawnAllocationRequest {
            x_px: 0x0150,
            y_px: 0x00a8,
            template_selector: 4,
            allocation_param: 3,
            selector_table_byte_0x80: 0x1f,
            selector_table_byte_0x81: 0x0b,
            animation_seed: MonsterAnimationSeed::from_original_setup(1, 2, 0x1a, 0x12),
        };

        assert_eq!(
            request.original_object_allocation_call(),
            OriginalObjectAllocationCall {
                param_1: 3,
                param_2: 0,
                param_3: 4,
                param_4: 0x12,
                param_5_x_velocity: 0,
                param_6_y_velocity: 0,
                param_7_x_px: 0x0150,
                param_8_y_px: 0x00a8,
            }
        );
    }

    #[test]
    fn original_object_allocation_call_attempt_uses_param_4_selector() {
        let call = OriginalObjectAllocationCall {
            param_1: 3,
            param_2: 0,
            param_3: 4,
            param_4: 0x12,
            param_5_x_velocity: 0x1000,
            param_6_y_velocity: -0x1000,
            param_7_x_px: 0x0150,
            param_8_y_px: 0x00a8,
        };
        let selector_entry = OriginalSpriteSelectorEntry {
            x_offset: 0,
            origin_height: 0x14,
            y_word: 0,
        };

        assert_eq!(
            call.attempt(0, Some(selector_entry)),
            Some(OriginalObjectAllocationFields {
                next_active_count: 1,
                x_velocity_word: 0x07ff,
                y_velocity_word: -0x07ff,
                position_origin_offset: -4,
            })
        );
        assert_eq!(call.attempt(0x1e, Some(selector_entry)), None);
        assert_eq!(call.attempt(0, None), None);
    }

    #[test]
    fn monster_spawn_allocation_request_resolves_original_table_indices() {
        let mut raw = [0; 30];
        raw[0x0b] = 2;
        raw[0x1d] = 3;
        let spawn = MonsterSpawn { raw };
        let selector_table_pairs = [(0, 0), (4, 0x11), (3, 0x1f)];
        let animation_ranges = [(0, 0), (1, 2), (3, 4), (0x20, 0x25), (0x30, 0x31)];

        let request = spawn
            .original_spawn_allocation_request_from_tables(&selector_table_pairs, &animation_ranges)
            .unwrap();
        assert_eq!(request.selector_table_byte_0x80, 3);
        assert_eq!(request.selector_table_byte_0x81, 0x1f);
        assert_eq!(
            request.animation_seed,
            MonsterAnimationSeed::from_original_setup(1, 3, 0x25, 0x20)
        );

        assert!(spawn
            .original_spawn_allocation_request_from_tables(
                &selector_table_pairs[..2],
                &animation_ranges
            )
            .is_none());
        assert!(spawn
            .original_spawn_allocation_request_from_tables(
                &selector_table_pairs,
                &animation_ranges[..3]
            )
            .is_none());
    }

    #[test]
    fn monster_spawn_allocation_request_uses_lezac_exe_low_memory_tables() {
        let mut raw = [0; 30];
        raw[0x00..0x02].copy_from_slice(&0x0120_u16.to_le_bytes());
        raw[0x02..0x04].copy_from_slice(&0x00a0_u16.to_le_bytes());
        raw[0x0b] = 4;
        raw[0x1a] = 3;
        raw[0x1d] = 2;
        let spawn = MonsterSpawn { raw };

        let request = spawn
            .original_spawn_allocation_request_from_original_tables()
            .unwrap();
        assert_eq!(
            request,
            OriginalSpawnAllocationRequest {
                x_px: 0x0120,
                y_px: 0x00a0,
                template_selector: 4,
                allocation_param: 3,
                selector_table_byte_0x80: 0x0d,
                selector_table_byte_0x81: 0x0d,
                animation_seed: MonsterAnimationSeed::from_original_setup(1, 2, 0x38, 0x36),
            }
        );
        assert_eq!(request.original_object_allocation_call().param_4, 0x36);
    }

    #[test]
    fn lezac_exe_low_memory_tables_match_checked_in_constants() {
        let exe = include_bytes!("../assets/LEZAC.EXE");
        let data_offset_1aa2_0000 = 0x770 + 0x1aa20 - 0x10000;

        assert_eq!(
            &exe[data_offset_1aa2_0000 + 0x42..data_offset_1aa2_0000 + 0x5a],
            &ORIGINAL_PLAYER_CONTACT_SELECTORS_0X42
        );
        assert_eq!(
            &exe[data_offset_1aa2_0000 + 0x52..data_offset_1aa2_0000 + 0x5a],
            &ORIGINAL_DROP_THRESHOLDS_0X52
        );
        for (idx, &(min, max)) in ORIGINAL_ANIMATION_RANGES_0X58.iter().enumerate() {
            let offset = data_offset_1aa2_0000 + 0x58 + idx * 2;
            assert_eq!([exe[offset], exe[offset + 1]], [min, max]);
        }
        assert_eq!(
            &exe[data_offset_1aa2_0000 + 0x77..data_offset_1aa2_0000 + 0x8a],
            &ORIGINAL_LOW_OBJECT_DAMAGE_SELECTORS_0X77
        );
        for (idx, &(a, b)) in ORIGINAL_SPAWN_SELECTOR_PAIRS_0X80.iter().enumerate() {
            let offset = data_offset_1aa2_0000 + 0x80 + idx * 2;
            assert_eq!([exe[offset], exe[offset + 1]], [a, b]);
        }
        assert_eq!(exe[data_offset_1aa2_0000 + 0x6a], 0x45);
        assert_eq!(exe[data_offset_1aa2_0000 + 0x6c], 0x4a);
        assert_eq!(exe[data_offset_1aa2_0000 + 0x6d], 0x4f);
    }

    #[test]
    fn original_object_allocation_velocity_word_clamps_like_fun_1000_2f9f() {
        assert_eq!(original_object_allocation_velocity_word(0), 0);
        assert_eq!(original_object_allocation_velocity_word(0x07ff), 0x07ff);
        assert_eq!(original_object_allocation_velocity_word(-0x07ff), -0x07ff);
        assert_eq!(original_object_allocation_velocity_word(0x0800), 0x07ff);
        assert_eq!(original_object_allocation_velocity_word(-0x0800), -0x07ff);
        assert_eq!(original_object_allocation_velocity_word(i16::MAX), 0x07ff);
        assert_eq!(original_object_allocation_velocity_word(i16::MIN), -0x07ff);
    }

    #[test]
    fn original_destructible_tile_hit_matches_fun_1000_5afd_range_and_tables() {
        let spawn_words = [0x1000, 0x1001, 0x1002];
        let score_deltas = [-20, 30, 40];

        assert_eq!(
            original_destructible_tile_hit(b'g', 0x1234, 0x0001, &spawn_words, &score_deltas),
            Some(OriginalTileHitResult {
                tile_id: b'g',
                spawn_word_0x2072: 0x1000,
                score_delta_0x2074: -20,
                triggers_above_tile_damage_scan: true,
                replacement_tile: 0,
                clears_tile_attribute: true,
            })
        );
        assert_eq!(
            original_destructible_tile_hit(b'i', 0x8000, 0x0000, &spawn_words, &score_deltas),
            Some(OriginalTileHitResult {
                tile_id: b'i',
                spawn_word_0x2072: 0x1002,
                score_delta_0x2074: 40,
                triggers_above_tile_damage_scan: false,
                replacement_tile: 0xff,
                clears_tile_attribute: false,
            })
        );
        assert_eq!(
            original_destructible_tile_hit(b'f', 0, 0, &spawn_words, &score_deltas),
            None
        );
        assert_eq!(
            original_destructible_tile_hit(b's', 0, 0, &spawn_words, &score_deltas),
            None
        );
    }

    #[test]
    fn original_reverse_lookup_11_byte_record_index_matches_fun_1000_3a56() {
        let refs = [0x1111, 0x2222, 0x1111, 0x3333];

        assert_eq!(
            original_reverse_lookup_11_byte_record_index(&refs, 0x1111),
            Some(3)
        );
        assert_eq!(
            original_reverse_lookup_11_byte_record_index(&refs, 0x3333),
            Some(4)
        );
        assert_eq!(
            original_reverse_lookup_11_byte_record_index(&refs, 0x4444),
            None
        );
        assert_eq!(original_reverse_lookup_11_byte_record_index(&[], 0), None);
    }

    #[test]
    fn original_attribute_effect_lookup_matches_fun_1000_3a7e_and_3b18() {
        let low_records = [
            OriginalAttributeEffectRecord {
                attr_ref: 0x8123,
                effect_a: 1,
                effect_b: 2,
            },
            OriginalAttributeEffectRecord {
                attr_ref: 0x8123,
                effect_a: 3,
                effect_b: 4,
            },
        ];
        let high_records = [
            OriginalAttributeEffectRecord {
                attr_ref: 0xc123,
                effect_a: 5,
                effect_b: 6,
            },
            OriginalAttributeEffectRecord {
                attr_ref: 0xd000,
                effect_a: 7,
                effect_b: 8,
            },
        ];

        assert_eq!(
            original_attribute_effect_lookup(0x8123, &low_records, &high_records, false),
            Some(OriginalAttributeEffectLookup {
                one_based_index: 2,
                effect: 3,
            })
        );
        assert_eq!(
            original_attribute_effect_lookup(0x8123, &low_records, &high_records, true),
            Some(OriginalAttributeEffectLookup {
                one_based_index: 2,
                effect: 4,
            })
        );
        assert_eq!(
            original_attribute_effect_lookup(0xc123, &low_records, &high_records, false),
            Some(OriginalAttributeEffectLookup {
                one_based_index: 1,
                effect: 5,
            })
        );
        assert_eq!(
            original_attribute_effect_lookup(0x0123, &low_records, &high_records, false),
            None
        );
        assert_eq!(
            original_attribute_effect_lookup(0x9999, &[], &high_records, false),
            None
        );
    }

    #[test]
    fn original_object_allocation_next_count_matches_fun_1000_2f9f_capacity_gate() {
        assert_eq!(original_object_allocation_next_count(0), Some(1));
        assert_eq!(original_object_allocation_next_count(0x1d), Some(0x1e));
        assert_eq!(original_object_allocation_next_count(0x1e), None);
        assert_eq!(original_object_allocation_next_count(0xff), None);
    }

    #[test]
    fn original_object_allocation_position_origin_offset_matches_fun_1000_2f9f() {
        let selector_entry = OriginalSpriteSelectorEntry {
            x_offset: -4,
            origin_height: 0x13,
            y_word: 7,
        };

        assert_eq!(
            original_object_allocation_position_origin_offset(0x1f, None),
            Some(0)
        );
        assert_eq!(
            original_object_allocation_position_origin_offset(0x12, Some(selector_entry)),
            Some(-3)
        );
        assert_eq!(
            original_object_allocation_position_origin_offset(0x12, None),
            None
        );
        assert_eq!(
            original_object_allocation_position_origin_offset(0x011f, None),
            None
        );
    }

    #[test]
    fn original_object_allocation_attempt_combines_recovered_fun_1000_2f9f_rules() {
        let selector_entry = OriginalSpriteSelectorEntry {
            x_offset: 0,
            origin_height: 0x0c,
            y_word: 0,
        };

        assert_eq!(
            original_object_allocation_attempt(0x1d, 0x0800, -0x0800, 0x12, Some(selector_entry)),
            Some(OriginalObjectAllocationFields {
                next_active_count: 0x1e,
                x_velocity_word: 0x07ff,
                y_velocity_word: -0x07ff,
                position_origin_offset: 4,
            })
        );
        assert_eq!(
            original_object_allocation_attempt(0x1d, 1, 2, 0x1f, None),
            Some(OriginalObjectAllocationFields {
                next_active_count: 0x1e,
                x_velocity_word: 1,
                y_velocity_word: 2,
                position_origin_offset: 0,
            })
        );
        assert_eq!(
            original_object_allocation_attempt(0x1e, 1, 2, 0x1f, None),
            None
        );
        assert_eq!(
            original_object_allocation_attempt(0, 1, 2, 0x12, None),
            None
        );
    }

    #[test]
    fn livels_bonus_spawns_use_in_bounds_pixel_coordinates() {
        let levels = load_levels("assets/LIVELS.SCH");

        for (level_idx, level) in levels.into_iter().enumerate() {
            for (bonus_idx, spawn) in level.bonuses.into_iter().enumerate() {
                let x = spawn.x_px() as usize;
                let y = spawn.y_px() as usize;
                assert!(
                    x < level.width * 8,
                    "level {level_idx} bonus {bonus_idx} x {x} out of {} raw {:?}",
                    level.width * 8,
                    spawn.raw
                );
                assert!(
                    y < level.height * 8,
                    "level {level_idx} bonus {bonus_idx} y {y} out of {} raw {:?}",
                    level.height * 8,
                    spawn.raw
                );
            }
        }
    }

    #[test]
    fn livels_seven_byte_records_include_original_player_start_flags() {
        let levels = load_levels("assets/LIVELS.SCH");

        assert_eq!(levels[0].bonuses[0].map_cell_ref(), 0);
        assert_eq!(levels[0].bonuses[0].player_start_mask(), 2);
        assert!(levels[0].bonuses[0].starts_player(1));
        assert_eq!(
            (levels[0].bonuses[0].x_px(), levels[0].bonuses[0].y_px()),
            (0x0118, 0x00a8)
        );
        assert_eq!(levels[0].bonuses[1].player_start_mask(), 1);
        assert!(levels[0].bonuses[1].starts_player(0));
        assert_eq!(
            (levels[0].bonuses[1].x_px(), levels[0].bonuses[1].y_px()),
            (0x0068, 0x00a8)
        );

        assert_eq!(levels[2].bonuses[0].player_start_mask(), 0);
        assert_eq!(levels[2].bonuses[0].map_cell_ref(), 0x001f);
        assert!(!levels[2].bonuses[0].starts_player(0));
        assert!(!levels[2].bonuses[0].starts_player(1));
    }

    #[test]
    fn livels_seven_byte_record_flag_values_match_shipped_data() {
        let levels = load_levels("assets/LIVELS.SCH");
        let mut raw6_values = Vec::new();

        for level in levels {
            for spawn in level.bonuses {
                if !raw6_values.contains(&spawn.entity_flags()) {
                    raw6_values.push(spawn.entity_flags());
                }
            }
        }
        raw6_values.sort_unstable();

        assert_eq!(raw6_values, vec![0, 1, 2]);
    }

    #[test]
    fn original_teleport_effect_request_matches_fun_1000_5999_call_shape() {
        let records = [
            BonusSpawn {
                raw: [0x34, 0x12, 0x20, 0x00, 0x30, 0x00, 1],
            },
            BonusSpawn {
                raw: [0x78, 0x56, 0x40, 0x00, 0x50, 0x00, 0],
            },
        ];

        let request = original_teleport_effect_request(&records, 0x9234, 0x6c, 0x6d).unwrap();
        assert_eq!(request.target_x_px, 0x20);
        assert_eq!(request.target_y_px, 0x30);
        assert_eq!(
            request.allocation_call,
            OriginalObjectAllocationCall {
                param_1: 5,
                param_2: 8,
                param_3: 0x0b,
                param_4: 0x6c,
                param_5_x_velocity: 0,
                param_6_y_velocity: 0,
                param_7_x_px: 0x20,
                param_8_y_px: 0x30,
            }
        );
        assert_eq!(
            request.animation_seed,
            MonsterAnimationSeed::from_original_setup(1, 2, 0x6d, 0x6c)
        );

        assert!(original_teleport_effect_request(&records, 0, 0x6c, 0x6d).is_none());
        assert!(original_teleport_effect_request(&records, 0x9999, 0x6c, 0x6d).is_none());
    }

    #[test]
    fn original_death_debris_effect_request_matches_lab_1000_76f4_call_shape() {
        let request = original_death_debris_effect_request(
            0x0120,
            0x00a0,
            0x0123,
            -0x0045,
            OriginalAnimationSelectors {
                low_object_frame: 0x6a,
                high_object_frame: 0x6c,
                max_frame: 0x6d,
            },
        );

        assert_eq!(
            request.allocation_call,
            OriginalObjectAllocationCall {
                param_1: 5,
                param_2: 0x0f,
                param_3: 0x0b,
                param_4: 0x0d,
                param_5_x_velocity: 0x0123,
                param_6_y_velocity: -0x0045,
                param_7_x_px: 0x0120,
                param_8_y_px: 0x00a0,
            }
        );
        assert_eq!(
            request.allocated_animation_seed,
            MonsterAnimationSeed::from_original_setup(2, 2, 0x6d, 0x6a)
        );
    }

    #[test]
    fn original_directional_tile_hit_effect_request_matches_fun_1000_6053_call_shape() {
        let request =
            original_directional_tile_hit_effect_request(0x0120, 0x00a0, 3, 0x0134, 4, 199)
                .unwrap();

        assert_eq!(
            request.allocation_call,
            OriginalObjectAllocationCall {
                param_1: 5,
                param_2: 0x0c,
                param_3: 10,
                param_4: 0x0134,
                param_5_x_velocity: -0x00ef,
                param_6_y_velocity: 0,
                param_7_x_px: 0x012a,
                param_8_y_px: 0x00aa,
            }
        );
        assert_eq!(
            request.cleared_animation_seed,
            MonsterAnimationSeed::from_original_setup(0, 0, 0, 0)
        );
        assert_eq!(request.next_effect_count_0x208e, 5);

        assert_eq!(
            original_directional_tile_hit_effect_request(0x0120, 0x00a0, 1, 0, 4, 0),
            None
        );
        assert_eq!(
            original_directional_tile_hit_effect_request(0x0120, 0x00a0, 1, 1, 0x0e, 0),
            None
        );
        assert_eq!(
            original_directional_tile_hit_effect_request(0x0120, 0x00a0, 5, 1, 4, 0),
            None
        );
    }

    #[test]
    fn livels_teleport_tiles_have_seven_byte_map_cell_targets() {
        let levels = load_levels("assets/LIVELS.SCH");
        let mut checked = 0usize;

        for (level_idx, level) in levels.iter().enumerate() {
            for (tile_idx, &tile) in level.tiles.iter().enumerate() {
                if tile != 0x45 {
                    continue;
                }
                checked += 1;
                let attr_ref = level.attrs[tile_idx] & 0x7fff;
                assert_ne!(attr_ref, 0, "level {level_idx} teleport has zero map ref");
                assert!(
                    level
                        .bonuses
                        .iter()
                        .any(|bonus| bonus.map_cell_ref() == attr_ref),
                    "level {level_idx} teleport tile {tile_idx} ref {attr_ref:#06x} has no target"
                );
            }
        }

        assert_eq!(checked, 10);
    }

    #[test]
    fn livels_platform_records_have_in_bounds_source_tiles() {
        let levels = load_levels("assets/LIVELS.SCH");

        for (level_idx, level) in levels.into_iter().enumerate() {
            for (platform_idx, spawn) in level.platforms.into_iter().enumerate() {
                let sx = spawn.source_x_px() as usize;
                let sy = spawn.source_y_px() as usize;
                assert!(
                    sx < level.width * 8,
                    "level {level_idx} platform {platform_idx} source x {sx} out of {} raw {:?}",
                    level.width * 8,
                    spawn.raw
                );
                assert!(
                    sy < level.height * 8,
                    "level {level_idx} platform {platform_idx} source y {sy} out of {} raw {:?}",
                    level.height * 8,
                    spawn.raw
                );
                let (dx, dy) = spawn.destination_tile();
                assert!(
                    dx < level.width,
                    "level {level_idx} platform {platform_idx} destination x {dx} out of {} raw {:?}",
                    level.width,
                    spawn.raw
                );
                assert!(
                    dy < level.height,
                    "level {level_idx} platform {platform_idx} destination y {dy} out of {} raw {:?}",
                    level.height,
                    spawn.raw
                );
            }
        }
    }

    #[test]
    fn platform_destination_tiles_match_original_mixed_encoding() {
        let levels = load_levels("assets/LIVELS.SCH");

        assert_eq!(levels[5].platforms[0].source_tile(), (2, 2));
        assert_eq!(levels[5].platforms[0].destination_tile(), (136, 37));
        assert_eq!(levels[5].platforms[1].source_tile(), (21, 21));
        assert_eq!(levels[5].platforms[1].destination_tile(), (9, 37));
        assert_eq!(levels[6].platforms[0].source_tile(), (0, 0));
        assert_eq!(levels[6].platforms[0].destination_tile(), (125, 21));
    }

    #[test]
    fn platform_action_records_match_original_reference_ranges_and_tile_substitutions() {
        let levels = load_levels("assets/LIVELS.SCH");

        assert_eq!(
            levels[5].platforms[0].affected_map_cell_ref_range(),
            (20, 20)
        );
        assert_eq!(levels[5].platforms[0].trigger_map_cell_ref(), 0x4088);
        assert_eq!(
            levels[5].platforms[0].tile_substitutions(),
            [(37, 97), (38, 96), (0, 0), (0, 0)]
        );

        assert_eq!(
            levels[5].platforms[1].affected_map_cell_ref_range(),
            (173, 173)
        );
        assert_eq!(levels[5].platforms[1].trigger_map_cell_ref(), 0x4009);
        assert_eq!(
            levels[5].platforms[1].tile_substitutions(),
            [(37, 93), (0, 0), (0, 0), (0, 0)]
        );

        assert_eq!(levels[6].platforms[0].affected_map_cell_ref_range(), (0, 0));
        assert_eq!(levels[6].platforms[0].trigger_map_cell_ref(), 1000);
        assert_eq!(
            levels[6].platforms[0].tile_substitutions(),
            [(21, 0), (0, 0), (0, 0), (0, 0)]
        );
    }

    #[test]
    fn platform_action_applies_original_reference_range_tile_substitutions() {
        let mut level = finalize(
            4,
            2,
            1,
            0,
            0,
            vec![37, 38, 37, 39, 37, 38, 37, 37],
            vec![20, 20, 21, 20, 19, 20, 20 | 0x8000, 20],
            0,
            0,
            Vec::new(),
            Vec::new(),
            vec![PlatformSpawn {
                raw: [20, 0, 20, 0, 0x88, 0x40, 37, 38, 0, 0, 97, 96, 0, 0],
            }],
        );

        assert!(level.apply_platform_action(0x4088));
        assert_eq!(level.tiles, vec![97, 96, 37, 39, 37, 96, 97, 97]);
        assert!(!level.apply_platform_action(0x4009));
    }

    #[test]
    fn platform_action_uses_last_matching_original_record() {
        let platforms = vec![
            PlatformSpawn {
                raw: [1, 0, 1, 0, 0x88, 0x40, 10, 0, 0, 0, 11, 0, 0, 0],
            },
            PlatformSpawn {
                raw: [1, 0, 1, 0, 0x88, 0x40, 10, 0, 0, 0, 12, 0, 0, 0],
            },
        ];

        let selected = original_platform_action_record(&platforms, 0x4088).unwrap();
        assert_eq!(selected.tile_substitutions()[0], (10, 12));
        assert!(original_platform_action_record(&platforms, 0x4009).is_none());

        let mut level = finalize(
            2,
            1,
            1,
            0,
            0,
            vec![10, 10],
            vec![1, 1 | 0x8000],
            0,
            0,
            Vec::new(),
            Vec::new(),
            platforms,
        );
        assert!(level.apply_platform_action(0x4088));
        assert_eq!(level.tiles, vec![12, 12]);
    }

    #[test]
    fn monster_defs_use_fixed_38_byte_records_before_trailing_tables() {
        let mut bytes = vec![2];
        let mut first = [0u8; MONSTER_TEMPLATE_RECORD_LEN];
        first[0] = 0x1e;
        first[1] = 4;
        first[3] = 0x10;
        first[5] = 6;
        first[0x15] = 7;
        let mut second = [0u8; MONSTER_TEMPLATE_RECORD_LEN];
        second[0] = 0x1f;
        second[1] = 5;
        second[3] = 0x0e;
        second[5] = 2;
        second[0x15] = 8;
        bytes.extend_from_slice(&first);
        bytes.extend_from_slice(&second);
        bytes.extend_from_slice(&[0x2e, 0x2e, 0x2d, 0x2d]);

        let path = std::env::temp_dir().join(format!("lezac-gran-test-{}.mst", std::process::id()));
        std::fs::write(&path, bytes).unwrap();
        let templates = load_monster_defs(path.to_str().unwrap());
        std::fs::remove_file(path).unwrap();

        assert_eq!(templates.len(), 2);
        assert_eq!(templates[0].raw.len(), MONSTER_TEMPLATE_RECORD_LEN);
        assert_eq!(templates[1].raw.len(), MONSTER_TEMPLATE_RECORD_LEN);
        assert_eq!(templates[0].object_id, 0x1e);
        assert_eq!(templates[0].anchor_table_index, 4);
        assert_eq!(templates[0].sprite_base, 0x2e);
        assert_eq!(templates[0].speed, 4);
        assert_eq!(templates[0].record_byte_05, 6);
        assert_eq!(templates[0].initial_state, 7);
        assert_eq!(templates[0].damage, 6);
        assert_eq!(templates[0].flags, 0);
        assert_eq!(templates[1].object_id, 0x1f);
        assert_eq!(templates[1].anchor_table_index, 5);
        assert_eq!(templates[1].sprite_base, 0x2e);
        assert_eq!(templates[1].speed, 5);
        assert_eq!(templates[1].record_byte_05, 2);
        assert_eq!(templates[1].initial_state, 8);
        assert_eq!(templates[1].damage, 2);
        assert_eq!(templates[1].flags, 0);
    }
}
