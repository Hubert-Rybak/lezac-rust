# Larax & Zaco - Complete Game Specification

Reverse-engineered from LEZAC.EXE (1996, Stefano Zanobi). DOS 16-bit real-mode, VGA mode 13h (320×200, 256 colors).

---

## 1. FILE FORMATS

### 1.1 SPR File Format (PROVA.SPR, BOMOMIMK.SPR, FONTS.SPR)

**Structure:** `[count:u8]` followed by `count` sprites, each: `[width:u8] [height:u8] [width×height raw pixel bytes]`

- **No header padding.** The first byte is the sprite count, byte 1 is the first sprite's width.
- Pixel value `0x00` = transparent (not drawn).
- All other pixel values are VGA palette indices (0x01-0xFF are opaque).
- Sprites have **variable dimensions** — each sprite has its own width×height prefix.
- No compression — raw pixel data.

**PROVA.SPR** (Player sprites): 91 sprites. Dimensions vary: mostly 16×16, but also 48×20, 22×18, 16×20, 13×13, 8×8, 10×12, 16×10, 15×5, 20×6, 21×7, 12×10, 10×5, and 1×1.

**BOMOMIMK.SPR** (Bomb/monster/misc sprites): 91 sprites. Similar variable dimensions.

**FONTS.SPR** (Font glyphs): 68 sprites.
- Sprites 0-25: Large/decorative letters A-Z, each 10×10
- Sprites 26-51: Small letters A-Z, each 8×8
- Sprites 52-61: Small digits 0-9, each 8×8
- Sprites 62-67: Small punctuation/symbols (`.` `:` `;` `,` `!` `'`), each 8×8
- Font color: palette index 0x01 (blue) with 0xFF outlines (dark red/shadow).

**Character mapping for text rendering:**
- `FUN_1000_136e` receives a one-based first-letter sprite index.
- Shipped menu/info/high-score calls pass `0x1b`, so normal UI text maps
  letters to zero-based sprites 26-51 and digits to 52-61.
- Passing first-letter index `1` maps letters to the large 10×10 range.

### 1.2 BOMPAL.PAL — VGA Palette

768 bytes = 256 RGB triplets. Each component is 6-bit (0-63 range, standard VGA DAC).

To convert to 8-bit: `value_8bit = (value_6bit << 2) | (value_6bit >> 4)`

Key palette entries:
| Index | Color | Use |
|-------|-------|-----|
| 0x00 | Black (#000000) | Transparent / background |
| 0x01 | Blue (#0000AA) | Font text |
| 0x04 | Red (#AA0000) | Player body |
| 0x06 | Brown (#AA5555) | Player skin |
| 0x0C | Light red (#FF5555) | Player pants/details |
| 0x37 | Blue (#0041FF) | Player eyes |
| 0x41 | Peach (#FF9E7D) | Skin highlights |
| 0x44 | Yellow (#FFFF7D) | Shoes/accents |
| 0xFF | Dark red (#550000) | Used in sprites (NOT transparency!) |

### 1.3 LIVELS.SCH — Level Data

7 levels stored sequentially with no global header. Each level:

```
Level {
    width:            u16 LE    // tile map width (60-180)
    height:           u16 LE    // tile map height (33-64)
    destruction_tile:  u8        // tile id counted for destruction % (106-111)
    bonus_required:   u16 LE    // minimum bonuses to collect
    destruction_pct:  u8        // minimum % of buildings to destroy
    
    tile_comp_size:   u16 LE    // compressed tile data size
    tile_comp_data:   [u8; tile_comp_size]
    
    attr_comp_size:   u16 LE    // compressed attribute data size  
    attr_comp_data:   [u8; attr_comp_size]
    
    scroll_x:         u16 LE    // initial camera X position
    scroll_y:         u16 LE    // initial camera Y position
    
    monster_count:    u8
    monster_data:     [MonsterEntry; monster_count]  // 30 bytes each
    
    bonus_count:      u8
    bonus_data:       [BonusEntry; bonus_count]      // 7 bytes each
    
    platform_count:   u8
    platform_data:    [PlatformEntry; platform_count] // 14 bytes each
}
```

Decoded entity record fields currently verified against the original
`LIVELS.SCH`:

```text
MonsterEntry (30 bytes):
  0x00..0x02  x_px: u16 LE
  0x02..0x04  y_px: u16 LE
  0x08        original active flag checked by the loader/update spawn path
  0x09        original per-record spawn count, decremented after a spawn
  0x0a        original spawn budget/counter, decremented after a spawn
  0x0b        original template selector; indexes a low-memory table at `0x80`
  0x0c..0x0e  base copied to spawned runtime word `0x0e` after RNG addition
  0x0e..0x10  random modulus for spawned runtime word `0x0e`
  0x10..0x12  base copied to spawned runtime word `0x10` after RNG addition
  0x12..0x14  random modulus for spawned runtime word `0x10`
  0x14..0x16  base copied to spawned runtime word `0x12` after RNG addition
  0x16..0x18  random modulus for spawned runtime word `0x12`
  0x18        spawned runtime vitality/health base
  0x19        random modulus added to the spawned vitality/health base
  0x1a        byte passed through to the original `FUN_1000_2f9f` allocation call
  0x1b        original spawn timer, decremented every frame by the loader/update path
  0x1c        original timer reset copied into `0x1b` when the spawn path runs
  0x1d        animation delay passed to `FUN_1000_06ab` for the spawned object
  remaining fields are still being named. The current live bridge still uses
              byte `0x04` as a temporary template seed for direct spawned
              monsters until the full original spawn-controller lifecycle is
              wired.

The recovered spawn path computes spawned runtime words `0x0e`, `0x10`, and
`0x12` as `base + FUN_1920_13a8(random_modulus)`, then computes spawned
vitality/health as byte `0x18 + FUN_1920_13a8(byte 0x19)`.
Each controller tick decrements timer byte `0x1b` before testing it; when it
reaches zero and active flag/count/budget are all non-zero, the timer is reset
from `0x1c`. Count byte `0x09` and budget byte `0x0a` are decremented only
after the original allocation path reports a successful spawn.
The allocation request shape is now modeled with recovered low-memory tables
from `assets/LEZAC.EXE`: `record[0x0b]` selects the pair at `0x80/0x81`, the
first byte indexes animation range bytes at `0x58/0x59`, and `record[0x1d]`
supplies the `FUN_1000_06ab` delay.
The spawn path's `FUN_1000_2f9f` call shape is:
`(record[0x1a], 0, record[0x0b], animation_min, 0, 0, x_px, y_px)`.
The call helper feeds param 4 through the recovered allocation-attempt
position-origin rule as a word-sized selector argument, matching
`FUN_1000_2f9f`'s `int param_4` signature and its use of that argument as the
sprite selector id.
The table constants are checked against the executable's `1aa2:0000` data
segment mapping in tests; similarly named `FUN_1000_07fa` arguments are screen
fill coordinates, not selector-table initialization.
`assets/LEZAC.EXE` is checked in as the binary source for these recovered
low-memory constants; no DOS runtime dump is currently checked in.
The original allocation helper `FUN_1000_2f9f` also clamps its stored velocity
words to the signed range `-0x7ff..0x7ff` before writing the runtime object
fields. Allocation succeeds only while the active object count byte is below
`0x1e`; on success the original increments that count and sets status word
`0x2072` to `1`, otherwise it leaves the count unchanged and sets `0x2072` to
`0`. For the allocated object's position-origin byte, sprite selector id `0x1f`
stores `0`; all other selector ids use the recovered selector-entry rule
`0x10 - selector_entry[1]`. These recovered rules are also exposed together as
a pure allocation-attempt helper for later live spawn-controller wiring.

BonusEntry (7 bytes):
  0x00..0x02  map_cell_ref: u16 LE, compared with `(tile_attr & 0x7fff)` by
              `FUN_1000_5999`
  0x02..0x04  x_px: u16 LE
  0x04..0x06  y_px: u16 LE
  0x06        original flag byte; shipped values are 0, 1, or 2.
              Low bits are original player-start flags:
              bit 0 starts player 1, bit 1 starts player 2
              `FUN_1000_5999` ignores this byte for teleporter targeting.

PlatformEntry (14 bytes):
  0x00..0x02  affected map-cell-reference range start
  0x02..0x04  affected map-cell-reference range end
  0x04..0x06  trigger map-cell reference; also encodes destination x:
               if high byte is 0x40, low byte is tile x
               otherwise little-endian pixel x divided by 8
  0x06        destination tile y in shipped records; also first source tile id
  0x06..0x0a  up to four source tile ids
  0x0a..0x0e  up to four replacement tile ids
```

`FUN_1000_5740` (the 14-byte platform action path) requests PROEFS.SON offset
`0x27` at priority `6`, finds the last platform record whose trigger reference
matches the activated tile attribute (`attr & 0x7fff`), then scans all map cells
whose attribute reference is inside the record's affected range. For each such
cell, non-zero source tile ids in `0x06..0x09` are replaced with the matching
replacement byte in `0x0a..0x0d`. The live Rust path now applies these
substitutions when pressing down on a `0x27` special platform with a non-zero
attribute reference.

**Level summary:**
| Level | Width | Height | Bonus Req | Destruction % | Monsters | Bonuses | Platforms |
|-------|-------|--------|-----------|--------------|----------|---------|----------|
| 0 | 60 | 33 | 1 | 50% | 1 | 2 | 0 |
| 1 | 100 | 53 | 3 | 60% | 2 | 2 | 0 |
| 2 | 150 | 60 | 7 | 20% | 3 | 3 | 0 |
| 3 | 100 | 58 | 3 | 70% | 2 | 4 | 0 |
| 4 | 110 | 62 | 8 | 65% | 3 | 5 | 0 |
| 5 | 180 | 64 | 8 | 40% | 4 | 3 | 2 |
| 6 | 140 | 52 | 1 | 10% | 0 | 2 | 1 |

### 1.4 Dual-Run RLE Compression (used by LIVELS.SCH and SFONLEF.ZBG)

Each 3-byte triplet encodes TWO runs:

```rust
fn decompress_dual_rle(input: &[u8], output_size: usize) -> Vec<u8> {
    let mut output = Vec::with_capacity(output_size);
    let mut i = 0;
    while output.len() < output_size && i + 2 < input.len() {
        let control = input[i];
        let value1 = input[i + 1];
        let value2 = input[i + 2];
        i += 3;
        
        let count1 = ((control >> 4) + 1) as usize;
        let count2 = ((control & 0x0F) + 1) as usize;
        
        let remaining = output_size - output.len();
        output.extend(std::iter::repeat(value1).take(count1.min(remaining)));
        
        if output.len() < output_size {
            let remaining = output_size - output.len();
            output.extend(std::iter::repeat(value2).take(count2.min(remaining)));
        }
    }
    output
}
```

Maximum run length per value: 16 (nibble value 15 + 1).

### 1.5 Tile Map

Each tile is 8×8 pixels. The tile map is a 1D array of bytes (row-major, width×height).

**Tile type categories:**
| Tile Value | Description |
|-----------|-------------|
| 0x00 (0) | Empty/air |
| 0x01 (1) | Solid wall/border (indestructible) |
| 0x02-0x07 | Various solid blocks |
| 0x15 (21) | Pillar/column (structural, may be destructible) |
| 0x31 (49) | Building block type 1 |
| 0x32 (50) | Building block type 2 |
| 0x33 (51) | Building block type 3 (most common building) |
| 0x3B (59) | Building accent/cap |
| 0x44 (68) | Ground/terrain |
| 0x51 (81) | Platform block |
| 0x57 (87) | Decorative element |
| 0x59-0x5C (89-92) | Roof/building top decorations |
| 0x5D (93) | Special platform/bracket |

**Attribute map:** Stored as `width×height` 16-bit LE words (decompressed size = tile_count × 2).

- Attribute value `0x0000`: No special attribute (default)
- Values `0x0001-0x00FF`: Group/building ID for connecting destructible structures
- Values `0x4000-0x7FFF`: Entity references (bit 14 set = entity marker). Lower bits identify specific entity types (bonuses, special blocks)
- Values `0x7Fxx`: Scoring/value associations for tiles
- Bit 15 (`0x8000`): "Processed" flag — set at runtime when a building group has been identified for destruction

### 1.6 SFONLEF.ZBG — Background Image

34,292 bytes total. The file starts with a 2-byte gradient header
`[start_index:u8] [ramp_count:u8]`, followed by `ramp_count × 6` bytes of
VGA 6-bit RGB ramp endpoints `[r0,g0,b0,r1,g1,b1]`. The remaining bytes are
dual-run RLE compressed pixel data and decompress to exactly 64,000 bytes
(320×200 VGA screen).

The background is rendered behind the game field and scrolls with parallax.

### 1.7 CARO.CAR — Title Card Image

8,450 bytes. Format: `[padding:u8] [width:u8 = 132] [raw_pixels: 132×64 bytes]`

- Uncompressed raw pixel data, 132×64 pixels.
- Displayed centered on screen at X offset `(320-132)/2 = 94`.
- Contains the "LARAX & ZACO" logo with explosion graphics.

### 1.8 RECS.DAT — High Score Records

92 bytes = 1 byte header + 7 entries × 13 bytes each.
On native builds this format is read from and written to `RECS.DAT`. On web
builds the same encoded bytes are hex-stored in browser `localStorage`, with the
embedded shipped `RECS.DAT` used as the fallback table.

```
RECS {
    count: u8           // Number of entries (7)
    entries: [Record; 7]
}

Record {
    score:   u32 LE     // Score value (e.g., 10000)
    level:   u8         // Level reached or flags (0x08 in default)
    name:    [u8; 8]    // Player name, space-padded ASCII
}
```

Default entries have score=10000, names: "lara", "stefano", "leo", "andrea", "daniel", "filippu", "luciano".
The decompiled ranking path compares the high and low score words separately,
so the Rust port preserves this as a full 32-bit little-endian score.

### 1.9 PROEFS.SON — PC Speaker Sound Effects

782 bytes of sound effect data. Sound effects are addressed by **byte offset** within this file (not by index).

Each sound effect is a sequence of `[frequency_divider:u16 LE] [duration:u16 LE] [flags:u8] [flags2:u8]` entries (6 bytes each). The frequency divider is for the 8253 PIT timer (1,193,180 / divider = Hz).

Known sound offset IDs (stored in memory at 0x2074 before calling the sound function):
- `0x08`: Bonus pickup / high-score confirm
- `0x12`: Jump
- `0x1A`: Teleport/special destination activation
- `0x21`: Menu select
- `0x24`: Bomb place  
- `0x27`: Falling/collapsing or special-tile effect
- `0x2D`: Player hurt
- `0x35`: Explosion
- `0x3D`: Level complete
- `0x78`: Death/game-over style effect

Priority system: Each sound has a priority level (`0x799F`). `FUN_1000_165a` accepts a request when no sound is active (`0x79C4 == 0`) or when `current_priority - 1 < requested_priority`, so equal or higher priority sounds preempt the current sound. Observed gameplay priorities include place bomb `2`, jump `3`, teleport/hurt `4`, explosion/bonus pickup/collapse `5`, special-tile effect `6`, level complete `10`, and high-score confirm/death/game-over style effects `11`.

### 1.10 GRAN.MST — Monster Type Definitions

399 bytes. First byte = 7, followed by seven 38-byte monster behavior template records. The remaining 132 bytes are trailing tables:
- 7 sprite-base bytes: `2e 2e 2d 2d 28 2a 2b`
- 7 X/Y word pairs copied to the original offset table:
  `(38,10), (42,4), (-7,10), (-12,16), (0,0), (-8,18), (39,17)`
- 1 motion-record count byte: `6`
- 6 × 16-byte motion/animation records

The Rust port uses GRAN.MST byte 0 as the original object/entity id (`0x1e` for the first record, `0x1f` for the other six), carries that id, byte `0x01` anchor-table index, the X/Y offset pair selected through that index, the full X/Y offset table, the movement seed words, and the fixed-record motion sequence ids onto spawned runtime monsters, resolves those ids to decoded motion records, uses the seven trailing sprite-base bytes for monster sprites, and preserves the counted 16-byte motion/animation records on `MonsterTemplate`. The exact per-field structure is still being named, but the fixed records contain speed-like values and movement/animation parameters.
Across the shipped LIVELS.SCH data, 14 spawned monsters resolve to object id
`0x1f`. The only non-`0x1f` shipped monster resolves to object id `0x1e` at
level index 2, spawn index 2, runtime position `(880,248)` after the port's
current Y-origin adjustment.

The shipped monster sprite bases are wrapped by the same-size runs visible in
`BOMOMIMK.SPR` so placeholder animation does not advance into unrelated sprite
dimensions: `0x28` has 3 frames, `0x2a` has 1, `0x2b` has 6, `0x2d` has 4, and
`0x2e` has 3.

For fixed 38-byte template records, bytes `0x0e`, `0x0f`, and `0x10` are motion sequence ids. The original `FUN_1000_6053` path for object id `0x1f` passes those bytes to `FUN_1000_5872` in order, skipping zero ids. In the shipped data those triples are:
`[5,4,3], [1,0,0], [2,0,0], [3,0,0], [4,0,0], [5,0,0], [6,0,0]`.
`FUN_1000_5872` treats ids greater than `0x80` as reversed motion ids and subtracts `0x80` before lookup; runtime monsters preserve that reverse flag when resolving motion records.
The `0x1f` motion branch is gated by the first sequence id: the original sets
countdown byte `0x02` to `0xfa` and calls `FUN_1000_5872` only when byte
`0x0e` is nonzero, with the second and third calls reached only after that
first id exists. The live Rust path now mirrors that gate and countdown side
effect.
For the state-6 object-id `0x1e` path, `FUN_1000_5cb0` instead reads the word
at fixed-record offset `0x0e` as a tile rectangle size: low byte width, high
byte height. The only shipped state-6 object therefore has a `5 × 4` tile scan
rectangle.
The same routine scans the row above that rectangle and the left/right columns
with the solid threshold `tile != 0 && tile <= 0x4c`, and scans the row below
with the ceiling threshold `tile != 0 && tile <= 0x52`.
After the scan, the deterministic velocity part is ordered as: if the bottom
scan is clear, add `0x40` to Y velocity; then if top blocks upward motion or
bottom blocks downward motion, set `y_velocity = -(y_velocity / 2)`; then if
left blocks leftward motion or right blocks rightward motion, set
`x_velocity = -(x_velocity / 2)`.
The live Rust update now uses this deterministic scan/velocity/8.8 position
advance for the shipped object-id `0x1e`, state-6 entity. The randomized
29-frame impulse, damage from scanned `0x75` (`'u'`) tiles, removal byte side
effect, original removal transition fields, dependent object-id `0x1f`
wake-up transitions, and original removal sound request are now live in the
Rust path; the remaining gap here is the exact original post-countdown
`FUN_1000_5a75` selector value copied from low memory.
The deterministic damage scan inside `FUN_1000_5cb0` checks the rectangle rows
and every other column starting at the left edge; each `0x75` (`'u'`) tile found
increments a damage counter.
For state-6 templates, fixed-record byte `0x24` is the damage budget used by
that counter; the shipped state-6 template stores `0x0a` there. `FUN_1000_5cb0`
decrements fixed-record byte `0x02` when the counter exceeds the current budget,
subtracts the counter from byte `0x24` with byte wrapping, and removes the
object when byte `0x02` becomes `0xff`.
More generally, byte `0x24` is a mutable vitality/damage-budget byte in
`FUN_1000_6053`: pickup branches can set it to `100` or add `0x21`, damage
branches add the signed damage byte at `0x661e`, and some death paths test
whether the signed result drops below zero. Spawned runtime monsters now carry
the original shipped `0x24` value separately from the state-6-specific budget;
the seven template values are `[0x0a,0xff,0x06,0xa8,0x01,0x04,0x02]`.
The live state-6 Rust path now applies those two bytes and marks the monster
dead on the `0xff` removal counter. The original death transition path
`FUN_1000_5bcc` then changes the object id to `0x0e`, changes state byte
`0x15` to `2`, sets countdown byte `0x02` to `0x3c`, clears animation byte
`0x1b`, and requests PROEFS.SON offset `0x3d` at priority `0x0c`; the live Rust
path now records those transition fields and plays that sound request when the
state-6 damage scan removes the object. Before changing the dying object,
`FUN_1000_5bcc` also scans active object-id `0x1f` entities: when their byte
`0x25` equals the dying object's word at offset `0x12`, it changes them to
object id `0x0e`, state `2`, clears byte `0x1b`, and sets countdown byte
`0x02` to `40 + FUN_1920_13a8(10)`. The live Rust path now carries those
fixed-record dependency keys and applies the same dependent transition.
For object ids `0x0c..0x1d` whose state byte is not `5`, the original bottom
branch decrements countdown byte `0x02` by `(frame_counter & 1)`, so the
transition advances only on odd frames. When that countdown reaches `0` or
wraps to `0xff`, the normal cleanup path sets state byte `0x15` to `5`, zeroes
X/Y velocity words at offsets `0x06` and `0x08`, changes object id to `0`, and
sets countdown byte `0x02` to `0x12`. The live Rust path applies this countdown
and cleanup for recovered object-id `0x0e` death transitions and keeps the
carried original `0x02` byte in sync with the death timer mirror. After cleanup,
the original `LAB_1000_76f4` effect branch emits `object_id - 0x0c + 1` small
effects for object ids below `0x13`. Each effect consumes the first
`FUN_1920_13a8(600) - 300` roll as the Y velocity word and the second as the X
velocity word, then requests `FUN_1000_2f9f(5, 0x0f, 0x0b, 0x0d, xvel, yvel,
x, y)`. If allocation succeeds, the new effect animation is initialized with
`FUN_1000_06ab(2, 2, [0x6d], [0x6a], active_block)`. For recovered object-id
`0x0e` death transitions, the live Rust path now emits three matching debris
effects and consumes the same RNG axis order.
The separate `FUN_1000_56b6` helper scans four caller-selected tiles in its
fixed pointer-walk order. It sets `0x2072` to the last scanned `0x75` (`'u'`)
tile address and subtracts `2` from signed byte `0x661e` for each such tile.
For non-zero tiles below `0x4d`, it also subtracts byte `0x208c` while the
incoming `0x2072` threshold is less than the current loop value (`4,3,2,1`).
The `FUN_1000_6053` caller applies that damage byte for object ids `1..8`: it
selects low-memory table byte `0x77 + object_id * 2 + slot`, where `slot` is
`1` for Y velocity `< 1` and `2` otherwise, stores `byte[0x1a] - 4` into
runtime byte `0x19`, and adds signed `0x661e` to vitality byte `0x24`. If that
signed sum is negative, the object transitions to object id `0x0c`, state `2`,
countdown byte `0x19`, and cleared animation byte `0x1b`; otherwise the summed
vitality byte is retained. The selector bytes are recovered from the original
executable's low-memory `0x77+` table.
For player contact with object ids `0x13..0x1d`, the same function records
`object_id - 0x12` for the player, changes the object to id `0x0b`, state `5`,
selects `FUN_1000_5a75` input from low-memory `0x42 + (object_id - 0x12) * 2`,
sets countdown byte `0x02` to `0x1a`, clears animation byte `0x1b`, clears X
velocity, and requests `FUN_1920_13a8(3)` before assigning randomized Y
velocity.
The randomized impulse branch of `FUN_1000_5cb0` runs only when the global
70 Hz frame counter is divisible by `0x1d`. In that branch, it consumes
`FUN_1920_13a8(100)` for a conditional sound request (`roll > 0x46` and even
frame), then `FUN_1920_13a8(800)` for X velocity (`±(150 + roll)`), and, when
the bottom scan is blocked, `FUN_1920_13a8(0x05dc)` for Y velocity
`-300 - roll`. The X sign branch checks the signed stack word at
`param_1 - 4` and uses the negative impulse when that word is `< 1`. In the
caller frame (`FUN_1000_6053`) that stack slot is the selected nearest live
player X delta (`player_x - object_x`), after comparing player 1 and player 2
by Manhattan distance. The live Rust path uses that selected X delta for the
impulse sign. The conditional sound uses PROEFS.SON offset `0x69` at priority
`4`.
The separate `local_33 == 2` motion branch in `FUN_1000_6053` uses the grounded
flag from the sampled bottom collision. While grounded and Y velocity is
non-negative, positive Y velocity is snapped to zero and the Y position word is
floored with `& 0xfff8`; otherwise Y velocity increases by `0x40` and caps at
`0x7ff`. The same branch calls `FUN_1000_5b86` horizontal friction only while
grounded.

Fixed-record byte `0x01` is not movement speed; `FUN_1000_6053` reads it as an X/Y offset-table index before calculating current object position. In the shipped templates the indices are `4, 5, 6, 3, 2, 1, 0`.

Fixed-record byte `0x02` is the original mutable countdown/removal counter.
`FUN_1000_6053` decrements it in the death/countdown paths, state-6 damage
handling uses it as the adjacent removal counter, and other branches reset it
to transition values such as `0xfa`, `0x1a`, `0x19`, `0x12`, or `100`. The
shipped GRAN.MST initial values are `[1,4,4,4,4,4,4]`, and spawned runtime
monsters carry the decoded byte separately from the state-6-specific removal
counter.

Fixed-record byte `0x05` is preserved as an unnamed source byte. The available
`FUN_1000_6053` decompilation does not contain a direct `pbVar20[5]` read, and
the Rust port no longer treats it as contact damage. The shipped values are
`[0x00,0x02,0x06,0x06,0xbf,0x00,0x02]`; the original role still needs another
evidence source before it can be named.

Fixed-record bytes `0x03` and `0x04` are horizontal animation range selectors
in the `local_33 == 3` branch of `FUN_1000_6053`: byte `0x03` is used when X
velocity is negative, byte `0x04` when X velocity is positive, and each indexes
the low-memory animation range table at `0x58/0x59`.

Fixed-record byte `0x14` is a signed position-origin adjustment. At the top of
`FUN_1000_6053`, the original subtracts this byte from the anchor-adjusted
object position before state dispatch, then adds it back before writing the
runtime position. Spawned runtime monsters carry the decoded byte; all shipped
GRAN.MST templates store `0` in this byte.

Fixed-record words at offsets `0x06`, `0x08`, `0x0a`, and `0x0c` seed the object movement state loaded by `FUN_1000_6053`: signed X velocity, signed Y velocity, X fractional accumulator, and Y fractional accumulator. In the shipped templates these all start at zero.

Fixed-record byte `0x15` is the initial state byte read by `FUN_1000_6053` as
`local_33`. The shipped values are `[6,5,5,5,5,5,5]`: the lone object-id
`0x1e` template starts in state `6`, which dispatches to `FUN_1000_5cb0`, while
the six object-id `0x1f` templates start in state `5`.

`FUN_1000_08a5` copies each 16-byte motion record to `0x79ea + n * 0x10`, then adds the active entity-table base to copied offsets 0 and 1. `FUN_1000_432a` reads copied offset 0 as an index into the X/Y offset table. `FUN_1000_5872` then reads runtime motion fields three bytes into that copied record:
- copied offset `0x00`: X/Y offset-table anchor index
- copied offset `0x01`: secondary anchor/index byte adjusted by the same loader base
- copied offset `0x02` (`0x79ec`): phase step added to copied offset `0x06`
- copied offset `0x03` (`0x79ed`): signed limit/sentinel; `-1` means load absolute velocity
- copied offset `0x06` (`0x79f0`): angle/phase accumulator used by `FUN_1000_432a`
- copied offsets `0x07..0x0a` (`0x79f1`, `0x79f3`): signed base X/Y words added to the offset-table anchor and trig result
- copied offset `0x0b` (`0x79f5`): signed little-endian word used as X delta after runtime preprocessing
- copied offset `0x0d` (`0x79f7`): signed little-endian word used as Y delta after runtime preprocessing
- copied offset `0x0f` (`0x79f9`): signed base added to the random Y word in the non-`-1` branch

Fixed-record bytes `0x16..0x1c` are the original animation seed block consumed
at the top of `FUN_1000_6053`: current frame byte, frame min/max bounds,
counter, delay, mode byte, and signed frame step. The shipped GRAN.MST values
are now decoded and carried onto runtime monsters. The counter/frame update is:
if mode is non-zero, increment the counter; when it exceeds delay, reset it,
add the signed step to the frame byte, bounce the step for mode `2` at the
frame bounds, otherwise wrap to the min frame when the max is exceeded. Mode
`3` additionally restores seven bytes from the backup block at `0x1d..0x23`
over the active animation block at `0x16..0x1c`; other branches save the active
block back to that backup with the opposite copy direction. The Rust port now
decodes and carries both blocks, models the restore direction, and uses this
counter path for live original-backed monster sprites. Live rendering still
needs the full state transitions that rewrite the active/backup blocks.
The animation setup helper `FUN_1000_06ab(mode, delay, max, min, dest)` writes
the seven-byte block `[min, min, max, delay, delay, mode, 1]`, so the Rust port
also exposes that constructor for the named animation state. The two landing
animation setup paths in `FUN_1000_6053` are also named: the backup idle block
uses `FUN_1000_06ab(3, 2, idle, idle, backup)`, and the active landing block
uses `FUN_1000_06ab(3, 3, max, first - 1, active)`.
The cleanup/death animation setup at the bottom of `FUN_1000_6053` uses
`FUN_1000_06ab(1, 2, [0x6d], selector, active)`, where `selector` is the
low-memory byte at `0x6a` for object ids below `0x13` and `0x6c` otherwise.
Those low-memory byte values are only read in the available decompiled export;
the repo does not contain the original image bytes needed to recover the actual
numeric frame values.
The selector application helper `FUN_1000_5a75(selector)` copies a 4-byte
selector-table entry into the active offset table, then stores
`0x10 - entry[1]` into object byte `0x14`. The Rust port pins this arithmetic
as `OriginalSpriteSelectorEntry::position_origin_offset`; the selector table
contents themselves remain unavailable without the original binary image.

`FUN_1000_432a` can overwrite the `0x79f5`/`0x79f7` words before movement uses them, so the Rust port exposes the initial copied words as `MonsterMotionRuntimeFields` while the exact state-machine calls and preprocessing are still being mapped.

`MonsterMotionRuntimeFields::apply_to_accumulator` pins the `FUN_1000_5872`
accumulator behavior: non-`-1` records add signed X/Y words, wrap by the
record limit when the absolute accumulator exceeds that limit, and `-1`
records return absolute X/Y words while clearing the accumulators.
Runtime monsters keep a `MonsterMotionAccumulator` and can apply their resolved
sequence fields into the original signed 8.8 motion state. The live gameplay
update now uses this decoded motion-record path for object id `0x1f`, the branch
where `FUN_1000_6053` calls `FUN_1000_5872`; other object ids still use the
older bridge behavior until their state machines are mapped.
An opt-in `advance_original_motion_once` method now applies the resolved motion
fields, advances the signed 8.8 state one tick, and syncs the visible float
position.
For non-`-1` motion records, the port also exposes a deterministic
`with_random_preprocess` helper for the `FUN_1000_432a` random branch: replace
the X word with an injected random word and replace the Y word with
`random_y_base + injected_random_y`.
For `-1` records, `with_advanced_phase` pins the deterministic phase update:
`angle_phase = (angle_phase + phase_step) & 0x7f`.
The same absolute branch uses trig-table phase `(angle_phase + 0x20) & 0x7f`
for X and `angle_phase` for Y after that advance.
`with_absolute_preprocess` pins the rest of the deterministic `-1` branch when
the trig offsets are supplied: advance the phase, then set X/Y words to
`anchor + base + trig`.
Runtime monsters can now mutate their resolved motion records with injected
random/trig values before applying them, mirroring the original per-frame
`FUN_1000_432a` rewrite step. The live `0x1f` update path now feeds bounded
records with high-word values from `OriginalRng` and feeds absolute records with
a 128-phase `f64` sine table scaled to signed 8.8 motion words. The original
initialization path builds a 128-entry table of 6-byte Turbo Pascal real values
at `0x7bda` in `FUN_1000_26e8`; `FUN_1000_432a` loads those real entries with
`FUN_1920_0f0f`, applies the Pascal real helper path, and converts them back to
integer offsets with `FUN_1920_0f13`. The Rust bridge now pins the same
128-phase quadrant and anti-symmetry shape, but the exact Pascal real
encoding/rounding and remaining state-machine call sites still need recovery
for full 1:1 motion.
`advance_original_motion_with_preprocess` pins the runtime ordering used by the
live `0x1f` branch: preprocess copied records, apply `FUN_1000_5872`-style
motion fields, then advance the signed 8.8 position.
The live game keeps a shared runtime copy of the decoded motion records and
preprocesses that table once per frame before monsters update, matching the
original call order where `FUN_1000_432a` runs before the `FUN_1000_6053`
entity loop.

### 1.11 Original Random Source

`FUN_1920_13a8` advances the runtime RNG with the Turbo Pascal-style 32-bit
LCG `seed = seed * 0x08088405 + 1`, returns the high word, and applies modulo
when the caller supplies a non-zero range. The Rust port exposes this as
`OriginalRng`. The original seed path stores DOS time fields as low word `CX`
and high word `DX`; new Rust games seed from equivalent system-time fields.
`OriginalRng::gen_centered(n)` captures the common original pattern
`FUN_1920_13a8(n) - n/2`.
Bomb-box bonus quantities now use it for their original
`FUN_1920_13a8` ranges. Debris velocity now uses the original
`FUN_1920_13a8(600) - 300` word range, scaled through the port's signed 8.8
motion convention. Incidental monster-drop selection also consumes
`OriginalRng`. The original object-id `0x0c` drop branch rolls
`FUN_1920_13a8(100)`, compares the result against a low-memory threshold table
starting at `0x52`, and turns the object into bonus id `local_14` / object id
`local_14 + 0x13` when the roll is at or above the first threshold. That branch
applies selector id `local_14 + 0x3e`, resets countdown byte `0x02` to `100`,
clears animation byte `0x1b`, and subtracts `200` from the Y velocity word. The
threshold bytes are recovered from `assets/LEZAC.EXE` as
`[0x28, 0x41, 0x47, 0x4e, 0x53, 0x59, 0x5d, 0x64]`.
The current JollyCloud bridge uses `OriginalRng` for its horizontal `-40..39`
spawn offsets. Tests guard against direct host-RNG calls returning to the game
and monster modules. Live monster-motion bounded-record preprocessing now also
consumes `OriginalRng` words; the exact original random range/state coupling is
still being named.
The `FUN_1000_6053` pickup effect-id branch is pinned for ids `2..6`: id `2`
sets vitality byte `0x24` to `100`, id `3` adds `0x21` and caps at `100`, id
`4` sets invincibility byte `0x79b2` to `0x2e`, id `5` sets bomb timer
`0x1b68` to `200` and adds medium `FUN_1920_13a8(10)+1` and large
`FUN_1920_13a8(4)+1`, and id `6` sets the same timer while adding medium
`FUN_1920_13a8(0x0d)+1`, large `FUN_1920_13a8(5)+2`, and super
`FUN_1920_13a8(2)+1`. All non-zero effect ids request PROEFS.SON offset `8` at
priority `5` after adding the score-table word for that effect id.

---

## 2. GAME ARCHITECTURE

### 2.1 State Machine

```
Boot → Init Subsystems → Title Screen (FUN_1000_26e8)
  → Menu (FUN_1000_2361)
    → '1': 1 player mode
    → '2': 2 player mode  
    → 'i': Info screen (shareware registration)
    → 'z': Instructions (controls)
    → 'r': Records (high scores)
    → 'l': Toggle language (Italian/English)
    → ESC: Exit
  → Level Intro ("now entering level N")
  → Gameplay (FUN_1000_6053 = main game loop)
    → Level Complete → Next Level
    → Player Death → Continue? → Restart/Continue
    → Game Over → Final Score → Menu
  → All Levels Complete → Final Score → Menu
```

### 2.2 Memory Layout (Key Variables)

| Address | Size | Description |
|---------|------|-------------|
| 0x79B7 | 1 | Current level number (0-6) |
| 0x79B8 | 1 | Number of players (1 or 2) |
| 0x79CC | 1 | Language: 0=Italian, 1=English |
| 0xC204 | 2 | Level width in tiles |
| 0x2086 | 2 | Bonus count required |
| 0x79B3 | 1 | Destruction % required |
| 0x79B4 | 1 | Level destruction tile id |
| 0x78BA | 2 | Total tiles (width × height) |
| 0xC1E0 | 4 | Far pointer to tile map |
| 0x6612 | 4 | Far pointer to attribute map |
| 0x2088 | 2 | Bonuses collected this level |
| 0x79AE | 1 | Player 1 bonus pickup type |
| 0x79AF | 1 | Player 2 bonus pickup type |
| 0x79E6 | 1 | Player 1 active (1=yes) |
| 0x79E7 | 1 | Player 2 active (1=yes) |
| 0x79E8 | 1 | Player 1 hit count |
| 0x79E9 | 1 | Player 2 hit count |
| 0x2080 | 2 | Building group count |
| 0x207E | 2 | Active entity count |
| 0x78C2 | 2 | Game tick/frame counter |
| 0x79CA | 1 | Can complete level flag |
| 0x208D | 1 | Active projectile count |

### 2.3 Coordinate System

- **Tile coordinates**: Integer grid, (col, row). Each tile is 8×8 pixels.
- **Pixel coordinates**: Fixed-point. The low 8 bits are fractional, so pixel = value >> 8. Movement speeds are stored as 8.8 fixed-point.
- **Screen**: 320×200 VGA. The playfield scrolls horizontally and vertically.
- **View/camera**: Determined by `scroll_x`, `scroll_y` (also stored in memory). Camera follows the active player.

---

## 3. PLAYER MECHANICS

### 3.1 Player Structure

Each player is a ~40-byte structure (at addresses 0x1B88 for P1, 0x1BAE for P2):

| Offset | Size | Description |
|--------|------|-------------|
| 0x00 | 1 | State/type (0=normal, 0x0B=pickup, 0x0C=death, etc.) |
| 0x01 | 1 | Entity slot index |
| 0x02 | 1 | Animation timer/frame counter |
| 0x03 | 1 | Left-facing sprite base |
| 0x04 | 1 | Right-facing sprite base |
| 0x06 | 2 | Horizontal velocity (8.8 fixed-point, signed) |
| 0x08 | 2 | Vertical velocity (8.8 fixed-point, signed) |
| 0x0A | 1 | Horizontal sub-pixel accumulator |
| 0x0C | 1 | Vertical sub-pixel accumulator |
| 0x0E | 2 | Bomb cooldown / firing state |
| 0x10 | 2 | Additional state |
| 0x14 | 1 | Sprite height offset |
| 0x15 | 1 | Player index (0=P1, 1=P2; or mode 5=death) |
| 0x16 | 1 | Current sprite frame |
| 0x17 | 1 | Animation range start |
| 0x18 | 1 | Animation range end |
| 0x19 | 1 | Animation sub-counter |
| 0x1A | 1 | Animation speed |
| 0x1B | 1 | Animation mode (0=off, 1=once, 2=bounce, 3=loop+action) |
| 0x1C | 1 | Animation direction (+1 or -1) |
| 0x1D+ | variable | Animation sprite table |
| 0x24 | 1 | Energy (health, 0-100) |
| 0x25 | 1 | Monster type that last hit player |

### 3.2 Physics

**Gravity:** Applied every frame when not standing on ground. Vertical velocity increases by `+0x40` (64 in 8.8 fixed-point = 0.25 pixels/frame²). Terminal velocity: `0x7FF` (~8 pixels/frame).

**Jumping:** When the jump key is pressed while standing on ground (vertical velocity = 0), vertical velocity is set to `-0x350` (-848 in 8.8 = ~3.3 pixels/frame upward).

**Horizontal movement:**
- Base speed: velocity changes by `±0x40` per frame (acceleration)
- Maximum horizontal speed: `0x400` (4 pixels/frame in 8.8)
- Friction/deceleration: `FUN_1000_5b86` snaps horizontal velocity to zero
  when `abs(velocity) < 0x2b`; otherwise it steps the signed 8.8 word toward
  zero by `0x2a`.
- Animation speed: `4 - abs(velocity) / 0x100` (faster movement = faster animation)

**Collision detection (4-point):**
The game checks 4 tiles around the player's position:
1. Above-left and above-right: ceiling check
2. Below-left and below-right: ground check
3. Left and right: wall check

Tile values ≤ 0x4C are considered solid for walking. Tiles > 0x4C with specific values (0x52+) have special behavior. Value `0x27` ('') triggers special platform activation (down key). Value `0x45` ('E') triggers teleport.

**Bounce off walls:** When hitting a wall, horizontal velocity is halved and reversed: `velocity = -velocity / 2`.

**Landing on ground:** When falling and hitting ground, vertical velocity is set to `0` if it is below `0x641` in 8.8 fixed-point. Faster falls bounce with `velocity = -velocity / 4`.

### 3.3 Controls

**Player 1:**
- Z: Move left
- X: Move right
- C: Move down (crouch / activate platforms)
- M: Jump
- N: Fire / place bomb

**Player 2:**
- Arrow keys: Movement
- 0 (numpad): Fire / place bomb

**Shared:**
- Left+Right together: Cycle bomb type
- E/R: Adjust screen width (1-player only)
- S: Toggle background rendering
- ESC: Quit game

### 3.4 Player Sprites (PROVA.SPR)

| Sprites | Description |
|---------|-------------|
| 0-7 | Player 1 walking right (8 frames) |
| 8-15 | Player 1 walking left (8 frames) |
| 16-18 | Player 1 additional animations |
| 19-23 | Player 2 walking right |
| 24-27 | Player 2 walking left |
| 28-35 | Player 2 additional animations |
| 36-38 | Jump/crouch frames |
| 39 | Large explosion sprite (48×20) |
| 40-43 | Explosion sequence (22×18) |
| 44-45 | Additional effects (16×20) |
| 46 | Another large sprite (48×20) |
| 47-52 | 1×1 placeholder pixels |
| 53-56 | Object sprites (16×16), exact use still being named |
| 57 | Small object/effect (8×8) |
| 58-61 | Bomb type sprites from `bomb_type + 0x39` |
| 62-66 | Terrain/block tiles (16×10) |
| 67-78 | Various game objects (16×16) |
| 79-84 | Status bar elements (15×5) |
| 85-88 | More status elements (20×6) |
| 89 | Life indicator (21×7) |
| 90 | Heart/life (12×10) |

---

## 4. BOMB MECHANICS

### 4.1 Bomb Types

Players carry bombs as their weapon. There are 4 bomb types, cycled by pressing left+right simultaneously:

| Type | Power | Blast Radius | Sprite | Description |
|------|-------|-------------|--------|-------------|
| 1 | Low | Small | 0x0D (13) | Basic bomb |
| 2 | Medium | Medium | 0x0E (14) | Improved bomb |
| 3 | High | Large | 0x0F (15) | Power bomb |
| 4 | Maximum | Very large (200 ticks) | 0x10 (16) | Super bomb |

### 4.2 Bomb Lifecycle

1. **Placement:** Player presses fire while standing. Bomb is placed at player's feet as a projectile entity.
2. **Arming:** 4-frame delay (`pbVar20[0x0E] = 4`), bomb moves with initial velocity matching player's momentum. The Rust port models this with `Bomb::arm_timer = 4/70s` and copies the placing player's velocity.
3. **Sprite:** The original passes `bomb_type + 0x39` to entity creation and HUD drawing, mapping one-based bomb types 1-4 to PROVA.SPR sprites 58-61.
4. **Detonation:** After timer expires, the bomb explodes.
5. **Explosion:** Creates explosion sprite (sprite 0x39/57 area). Destroys surrounding tiles in a radius proportional to bomb power.
6. **Terrain destruction:** Each destroyed tile is replaced with empty (0x00). Building groups connected to destroyed tiles are evaluated — if the support structure is compromised, the entire group falls.

### 4.3 Bomb Inventory

Bomb counts per type stored at `(player_index * 4 + bomb_type + 0x1B67)`. Starting bomb count is 99 (displayed in HUD). Special bonuses can award more bombs.
`FUN_1000_5715(cap, add, dest)` is the original capped byte-add helper used by
bomb-box bonuses: it performs an 8-bit wrapping add into the destination byte,
then clamps to `cap` only if the wrapped byte is greater than `cap`.

### 4.4 Explosion & Destruction Physics (FUN_1000_370e)

The explosion system is the most complex part of the game:

1. When a bomb explodes at tile position `(x, y)`, the game reads the attribute value at that tile.
2. If the attribute < 0x4000 (building group), the game performs a **flood-fill** to find all connected tiles with the same attribute value:
   - Searches in all 4 directions (up, down, left, right)
   - Expands the search rectangle until no more matching tiles are found
3. Creates a "destruction zone" entity (stored in the building group table at 0x6611+, 15 bytes per entry):
   - Position, extent, velocity, tile count
   - The zone then animates: tiles crack (value changes to 0x76-0x79), then collapse
4. If attribute ≥ 0x4000 (entity/object reference), creates an entity with specific behavior

**Destruction scoring:** Building destruction earns points. A destruction bonus appears if the player destroys enough buildings to meet the level's destruction requirement.

---

## 5. ENTITY SYSTEM

### 5.1 Entity Types

The game tracks several entity categories:

**Building groups** (max 250, at 0x6611, 15 bytes each):
- Position, extent rectangle
- Attribute value (with 0x8000 flag for "processed")
- Horizontal/vertical velocity
- Tile count, state flags

**Moving entities** (monsters/projectiles, at 0x2093, 11 bytes each):
- Tile position pointer
- Velocity (horizontal, vertical)
- Tile type, state, timer
- Damage tracking

**Projectiles** (max 30, at 0x1BAE+, 38 bytes each):
- Position, velocity, sprite
- Owner, type, lifetime

### 5.2 Monster Behavior (FUN_1000_5102)

Monsters are building-group entities that move through the tile map:

1. **Movement:** Each monster has horizontal (dx) and vertical (dy) velocity stored as signed 8-bit values with fractional accumulation.
   The port now has a pinned axis-step primitive matching the original signed
   8.8 update: add the velocity low byte to a fractional accumulator, then add
   the signed high byte plus carry to the pixel position. Runtime monsters also
   retain an `OriginalMotionState` with pixel position, fraction accumulators,
   and signed X/Y velocity words.
2. **Gravity:** Monsters are affected by gravity. If not on ground, signed-byte vertical velocity `local_e += 4` per frame and caps near `0x7B`, equivalent in the Rust port to `0.25 px/frame²` and `~7.7 px/frame`.
3. **Wall bouncing:** When a monster hits a wall, it reverses direction and sets a directional flag.
4. **Patrol behavior:** Monsters track their current column within their building group boundaries. They bounce between the left and right edges of their territory.
5. **Chase behavior:** When a player is within range, the monster adjusts its velocity toward the player (homing within territory bounds).
6. **Damage dealing:** When a monster is within 10 pixels on both X and Y deltas, the original increments that player's hit counter (`0x79e8`/`0x79e9`). The outer player loop subtracts that counter from energy once per 70 Hz frame, so each overlapping monster costs 1 energy per frame.
7. **Crumbling:** Monsters can fall if their support tiles are destroyed. They accumulate fall damage.

---

## 6. SCORING, LIVES & ENERGY

### 6.1 Starting Values

- **Lives:** 3 per player (stored as animation state; displayed as green 'AA' characters)
- **Energy:** 100 per life (offset 0x24 in player structure)
- **Starting score:** 0 (stored as 32-bit value at player score offset)
- **Starting bombs:** 99 of type 1 (basic)
- **Starting level:** 0

### 6.2 Score Values

Scoring is based on a lookup table at offset `0x34` in memory (indexed by event type × 2):
- Collecting a bonus: Points from bonus type table
- Level-complete destruction bonus: destroyed building-tile count × 10
- Killing monsters: Points from monster type table
- Bonus pickup types 2-6 give escalating rewards
- Level-complete leftover bomb bonus: medium bombs × 100, large bombs × 500,
  super bombs × 2000

The Rust port stores player scores as `u32` and uses saturating accumulation
for all known score awards, including powerup collection.

### 6.3 Energy & Damage

- Energy starts at 100 and decreases when hit.
- Monster contact uses the original hit-counter shape: each monster within
  10 pixels on both axes increments the player's per-frame hit count, and the
  outer update subtracts one energy point per counted hit.
- Hard landings use the decompiled `0x641` vertical-velocity threshold: below
  it, vertical velocity is cleared; at or above it, the player bounces with
  `velocity = -velocity / 4`. No landing energy decrement has been recovered in
  that original branch.
- When energy ≤ 0, the player dies (state = 0x0C).
- On death, the player can press fire to continue (respawn at current position) or wait to restart the level from the beginning.
- Cannot re-enter a level if not enough bonuses remain to complete it.
- Continuing restores bomb inventory minimums from the original restart path:
  basic bombs at least 100, medium bombs at least 10, and large bombs at least 2.

### 6.4 Bonus Types

When a player touches a bonus entity (tile types 0x13-0x1D):
| Bonus | Effect |
|-------|--------|
| Type 0 | Generic present/collectible |
| Type 1 | Generic bonus token |
| Type 2 | Full energy restore (set to 100) |
| Type 3 | Partial energy restore (+33, cap at 100) |
| Type 4 | Invincibility for 46 frames |
| Type 5 | Bomb supply: basic set to 200, medium +1..10, large +1..4 |
| Type 6 | Large bomb supply: basic set to 200, medium +1..13, large +2..6, super +1..2 |

Collectible sprites use the matching `BOMOMIMK.SPR` tile range: bonus id 0 draws sprite `0x13`, id 1 draws `0x14`, id 2 draws `0x15`, and so on for the known bonus variants.

The Rust port now also handles tile-map bonus pickups directly: overlapping
tiles `0x13..0x1D` are cleared, counted toward the level bonus requirement, and
apply the same effect mapping as falling/spawned powerups.

Seven-byte records whose low `raw[6]` bits mark P1/P2 starts are not spawned as
pickups. The original start-position scan tests that flag byte directly, while
`FUN_1000_5999` ignores it and uses only the matching map-cell reference plus
X/Y coordinate words. Shipped records only use flag values `0`, `1`, and `2`,
so all non-start records carry `0` and spawn as baseline level-goal pickups.

Teleporter targeting follows the original `FUN_1000_5999` path: the source tile
attribute is masked with `0x7fff` and matched against a seven-byte entity
record `map_cell_ref`; the matched record's pixel X/Y words become the
destination coordinates. Shipped teleport tiles all have one of these map-cell
targets.
The same original path requests PROEFS.SON offset `0x1a` at priority `4`, then
spawns a target effect with `FUN_1000_2f9f(5, 8, 0x0b, [0x6c], 0, 0, x, y)`.
On successful allocation, it initializes that object's animation with
`FUN_1000_06ab(1, 2, [0x6d], [0x6c], active_block)`.

Dropped bonus entities use the normal falling-object gravity branch:
`local_10 += 0x40` in 8.8 fixed-point, capped at `0x7FF`.
`OriginalMotionState` has the same word-level gravity helper pinned for future
entity-state integration.

### 6.5 Level Completion

A level is completed when:
1. Bonuses collected (`0x2088`) ≥ required bonuses (`0x2086`)
2. Building destruction percentage (`0x79B5`) ≥ required percentage (`0x79B3`)

The decompiled completion gate checks the original `0x79c5` and `0x79c6`
flags, set by `FUN_1000_3184` when those two requirements are satisfied. The
`FUN_1000_6053` branch that sees a sampled tile `> 0x6c` sets a side-effect flag
for the destruction event; it is not a player-overlap exit requirement.
The separate continue-availability flag (`0x79ca`) is set when collected
bonuses plus remaining required bonus sources can still meet `0x2086`.

Destroying a tile matching the level header's required bonus tile increments
`0x2088`. The Rust port also counts live level-goal pickup entities as
remaining required bonus sources for continue availability.
The `FUN_1000_5afd` tile-hit helper is pinned as a pure contract: only tile
bytes `0x67..0x72` (`'g'..'r'`) are accepted, the zero-based tile offset indexes
the original words at low-memory table offsets `0x1a` and `0x02`, an adjacent
non-zero attribute below `0x8000` sets the caller's damage-scan flag, and the
sampled tile is replaced with `0` for normal attributes or `0xff` for high
attributes.
When that caller has a non-zero `0x2072` selector word and fewer than `0x0e`
active effects, it can spawn a directional tile-hit effect with
`FUN_1000_2f9f(5, 0x0c, 10, 0x2072, -0x28 - FUN_1920_13a8(200), 0, x, y)`.
The four sampled directions use offsets `(-2,-2)`, `(-2,10)`, `(10,10)`, and
`(10,-2)` from the current object position, then clear the new object's
animation block with `FUN_1000_06ab(0, 0, 0, 0, active_block)` and increment
effect counter `0x208e` after successful allocation.
`FUN_1000_3a56` is pinned as a reverse lookup over active 11-byte records: it
starts at `(count - 1) * 0x0b`, compares the word at record offset `0x02`
against `0x2072`, and writes the matching one-based record index to `0x2074`.
`FUN_1000_3a7e` and `FUN_1000_3b18` are pinned as related attribute-effect
lookups. Attributes without the high bit clear the effect byte. High-bit
attributes with `(attr & 0x7fff) < 0x4000` search the active 15-byte record
table in reverse; larger high-bit attributes search the active 11-byte record
table in reverse. The two helpers differ only by whether they copy the first or
second effect byte from the matched record, then write the one-based record
index to `0x2074`.

---

## 7. RENDERING

### 7.1 Screen Layout

```
+---320 pixels wide---+
|                      |  Top: Game viewport
|    Playing field     |  (scrolls with player)
|    (tiles + sprites) |  
|                      |  
+----------------------+ y=160 (approx)
|   HUD / Status Bar   |  Bottom: Fixed HUD
+----------------------+ y=200
```

The HUD shows:
- Energy bar (yellow)
- Score (numeric)
- Bomb count per type
- Required bonuses and destruction %
- Lives remaining (green characters)

The in-game HUD renders destruction progress as numeric font fields only; the
original `FONTS.SPR` punctuation range has no percent glyph.

### 7.2 Tile Rendering

Each tile is 8×8 pixels. Tiles are drawn from the tile sprite set (sprite indices in BOMOMIMK.SPR).

The tile at position (col, row) is drawn at screen position:
- `screen_x = col * 8 - camera_x`
- `screen_y = row * 8 - camera_y`

Only tiles within the visible viewport are drawn.

### 7.3 Background Scrolling

The background (SFONLEF.ZBG) scrolls at a different rate than the foreground (parallax):
- Background position is derived from camera position
- The background wraps/repeats as needed
- Can be toggled on/off with the 'S' key

### 7.4 Sprite Drawing

```rust
fn draw_sprite(screen: &mut [u8; 64000], sprite: &Sprite, x: i32, y: i32) {
    for row in 0..sprite.height {
        for col in 0..sprite.width {
            let pixel = sprite.pixels[row * sprite.width + col];
            if pixel != 0 {  // 0 = transparent
                let sx = x + col as i32;
                let sy = y + row as i32;
                if sx >= 0 && sx < 320 && sy >= 0 && sy < 200 {
                    screen[(sy * 320 + sx) as usize] = pixel;
                }
            }
        }
    }
}
```

### 7.5 Palette Animation

FUN_1000_079d performs real-time palette cycling for certain color indices.
After the `0x78C2` frame word is incremented, every fifth frame rewrites 6
consecutive palette entries (starting at index 0xE6 = 230) as a red ramp. The
first entry uses global phase byte `0x79AD`; each following entry adds 7 to the
6-bit red component, wrapping to `0x14` after `0x3F`. The caller then advances
`0x79AD` by 7 with the same wrap.

The Rust port keeps indexed pixel data for loaded textures and refreshes sprite,
title, and background textures from the animated palette on the matching
five-frame cadence.

---

## 8. GAME LOOP (FUN_1000_6053)

The main game loop processes each active entity per frame:

```
For each entity:
    1. Update animation (frame counter, sprite cycling)
    2. Set up player controls based on entity type (P1 vs P2 key mappings)
    3. Read entity position from position table
    4. Check distance to players (for monster AI / collision)
    5. Based on entity state (local_33 / local_11):
       - State 0: Normal player movement
           - Apply gravity
           - Process left/right/jump/fire input
           - Handle bomb cycling (left+right together)
           - Process bomb placement (fire button)
           - Check 4 surrounding tiles for collisions/pickups
           - Award score and bonuses
       - State 2: Falling/dead entity
           - Only gravity, no input
       - State 3: Monster movement
           - Apply gravity, patrol, wall bounce
           - Chase player if in range
       - State 4: Projectile movement
           - Move according to velocity
           - Check collisions with terrain
           - Spawn explosion effects on impact
    6. Apply velocity to position (fixed-point accumulation)
    7. Check for tile-based damage/interaction
    8. Update position in entity table
    9. If entity lifetime expired:
       - Trigger death/destruction effects
       - Spawn debris particles
       - Update scores
```

### 8.1 Frame Timing

The game uses VGA vertical retrace for timing (`FUN_1920_0f0f/0efb/0f01/0f13`
wait for vsync). At 70 Hz VGA refresh, this gives approximately 14ms per frame.
The word frame counter at `0x78C2` increments each frame and wraps at 16 bits.

---

## 9. LEVEL LOADING (FUN_1000_0c33)

1. Open LIVELS.SCH file
2. Seek to current level's data (sequential read, skip previous levels)
3. Read level header (width, height, destruction tile, bonus_req, destruction_pct)
4. Allocate tile map: `malloc(width * height + 16)`
5. Allocate attribute map: `malloc(width * height * 2 + 16)`
6. Read compressed tile size, decompress tile data with dual-run RLE
7. Read compressed attr size, decompress attribute data
8. Read initial scroll position
9. Read monster definitions (count + 30 bytes each)
10. Read bonus definitions (count + 7 bytes each)
11. Read platform definitions (count + 14 bytes each)
12. Initialize game state variables
13. Place players at spawn positions
14. Start game loop

---

## 10. TEXT RENDERING (FUN_1000_136e)

Text is rendered character-by-character using FONTS.SPR:

```rust
fn render_text(text: &str, x: i32, y: i32, first_letter_one_based: u8, spacing: u8) {
    let first_letter = first_letter_one_based as usize - 1;
    for (i, ch) in text.bytes().enumerate() {
        let sprite_idx = if ch >= b'a' && ch <= b'z' {
            first_letter + (ch - b'a') as usize
        } else if ch >= b'0' && ch <= b'9' {
            first_letter + 26 + (ch - b'0') as usize
        } else if ch == b' ' {
            continue; // skip spaces
        } else {
            // Map punctuation to sprite indices 36+
            first_letter + 36 + map_symbol(ch)
        };
        draw_sprite(sprite_idx, x + (i as i32 * spacing as i32), y);
    }
}
```

The original routine's normal small-font callers pass first-letter index `0x1b`
and spacing `9`, which corresponds to zero-based sprite 26 for `a`.

The Rust port's Info and Instructions states now page through the extracted
original English/Italian text blocks from the `1aa2:*` string area instead of
rendering shortened summaries. Instruction-page `*` markers are preserved as
blank leading lines where the original text includes them.
Level-complete score lines use the active language (`PLAYER ... SCORE` or
`GIOCATORE ... PUNTEGGIO`).
The level-complete screen also renders the original localized destruction bonus
line and per-player leftover bomb bonus, and applies those bonuses once when
entering the level-complete state.
Game-over score lines follow the same active-language rule.

The Records state renders each `RECS.DAT` entry's name, reached level byte, and
score.
New-record name entry follows the original filter: after uppercasing keyboard
input, only `A-Z` and spaces are accepted; accepted letters are stored lowercase.
Submitting an empty name writes the space-padded 8-byte name buffer directly
rather than substituting a default name.
The visible entry field mirrors the same 8-byte buffer with period placeholders
for empty slots.
When loading records, the Rust port trims only right-padding spaces so leading
spaces in the original 8-byte name field are preserved.

Special text encoding (from game strings):
- `\` = newline
- `:` = period `.`
- `;` = colon `:`
- `=` = comma `,`
- `>` = exclamation `!`
- `?` = apostrophe `'`
- `*` = blank line

---

## 11. SPECIAL MECHANICS

### 11.1 Special Platforms

Pressing down (C for P1, down arrow for P2) on a special platform tile (type 0x27/39) activates it — the player drops through or teleports.

### 11.2 Building Collapse

When a building's support structure is destroyed (foundation tiles removed by explosion), the remaining connected tiles above begin to fall:
1. The building group's vertical velocity increases each frame (gravity)
2. The tiles animate through crack states (0x76 → 0x77 → 0x78 → 0x79)
3. After reaching state 0x79, the tiles either:
   - Land on solid ground (stop, become permanent)
   - Continue falling off-screen (removed)
4. Falling buildings can crush and kill monsters/players

### 11.3 Invincibility Timer

Collecting bonus type 4 sets invincibility to `0x2E` (46) frames. During this
time, the player flashes and monsters cannot deal damage. The Rust port stores
this as a fixed 70 Hz frame counter (`Player::invincible_frames`) rather than
seconds.

### 11.4 Screen Width Adjustment

In single-player mode, pressing E/R adjusts the visible tile columns. The game
stores the visible width at `0xC1EA` and computes screen offsets accordingly.
Default visible width is `0x28` (40 tiles = 320 pixels). Pressing E decrements
that width by one tile while it is above `0x15`; pressing R increments it by one
tile while it is below `0x28`.

---

## 12. INITIALIZATION SEQUENCE

1. Set video mode 13h (INT 10h, AX=0013h)
2. Set keyboard repeat rate (OUT 60h)
3. Allocate memory buffers:
   - 0x318 (792) bytes for sprite buffer
   - 57,000 bytes for back buffer
   - 0x5334 (21,300) bytes for tile rendering buffer
   - 60,000 bytes for general purpose buffer
4. Load palette (BOMPAL.PAL) 
5. Load font sprites (FONTS.SPR)
6. Load player sprites (PROVA.SPR)
7. Load bomb/monster sprites (BOMOMIMK.SPR)
8. Load background (SFONLEF.ZBG), decompress to back buffer
9. Generate procedural background variations using FUN_1000_01fc:
   - Fills 64,000 bytes with palette-modulated values
   - Uses pseudo-random number generator seeded from VGA timing
   - Creates a 7-color cycling pattern
10. Load title card (CARO.CAR)
11. Load sound effects (PROEFS.SON)
12. Load records (RECS.DAT)
13. Enter title/menu screen

---

## 13. KEY CONSTANTS

```rust
// Tile size in pixels
const TILE_SIZE: u32 = 8;

// Screen dimensions
const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 200;

// Physics (8.8 fixed-point)
const GRAVITY: i16 = 0x40;           // 64 = 0.25 px/frame²
const JUMP_VELOCITY: i16 = -0x350;   // -848 = -3.31 px/frame
const MAX_FALL_SPEED: i16 = 0x7FF;   // 2047 = ~8 px/frame
const LAND_BOUNCE_MIN: i16 = 0x641;  // 1601 = threshold before landing bounce
const MOVE_ACCEL: i16 = 0x40;        // 64 = 0.25 px/frame²
const MAX_MOVE_SPEED: i16 = 0x400;   // 1024 = 4 px/frame
const WALL_BOUNCE_FACTOR: i32 = 2;   // velocity halved on wall bounce
const LAND_BOUNCE_FACTOR: i32 = 4;   // velocity quartered on landing

// Game limits
const MAX_BUILDING_GROUPS: usize = 250;
const MAX_PROJECTILES: usize = 30;
const MAX_ENTITIES: usize = 1600;
const MAX_LEVELS: usize = 7;

// Player defaults
const STARTING_LIVES: u8 = 3;
const STARTING_ENERGY: u8 = 100;
const STARTING_BOMBS: u8 = 99;
const STARTING_SCORE: u32 = 0;

// Timer values
const INVINCIBILITY_DURATION: u8 = 46;
const DEATH_ANIMATION_FRAMES: u8 = 18;  // 0x12 (Rust port: 18.0 / 70.0 seconds)
const LEVEL_COMPLETE_DELAY: u8 = 100;   // 0x64 (Rust port: 100.0 / 70.0 seconds)
```
