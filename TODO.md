# Larax & Zaco — Rust Port: Progress & TODO

## Current Status

A playable Rust port of the 1996 DOS game "Larax & Zaco" by Zanobi Software,
built with macroquad 0.4. Compiles on native (Linux/macOS/Windows) and the
`wasm32-unknown-unknown` target.
The game renders levels with tiles, sprites, HUD, player movement, bombs, monsters,
powerups, and multi-screen scrolling on native builds. WASM builds embed the
shipped asset files; high-score saving persists natively and in browser localStorage.
Font rendering is now correct.

## What Works

### Asset Loading
- **BOMPAL.PAL** — 256-color VGA palette (6-bit → 8-bit), fully decoded ✅
- **PROVA.SPR** — Player sprites (91 sprites, variable dimensions up to 48×20) ✅
- **BOMOMIMK.SPR** — Misc/monster sprites (91 sprites, variable dimensions) ✅
- **FONTS.SPR** — Font sprites (68 sprites: 26×10×10 large + 42×8×8 small) ✅
- **LIVELS.SCH** — Level tilemap data, 3-byte triplet RLE, 7 levels loaded ✅
- **RECS.DAT** — High score records (7 entries × 13 bytes:
  `score:u32`, `level:u8`, `name[8]`) ✅
- **SFONLEF.ZBG** — Gradient palette + parallax background pixels ✅
- **SPR format** fully reverse-engineered: 1-byte header (count) + per-sprite
  width/height + raw pixel data. Sprites have **varying** dimensions within
  a single file. No 0xFF row terminator — all bytes are raw palette indices. ✅

### Font Rendering
- Character mapping decoded from decompiled `FUN_1000_136e`: ✅
  - Sprites 0–25: 10×10 large/decorative font (a–z)
  - Sprites 26–51: 8×8 small font (a–z)
  - Sprites 52–61: 8×8 digits (0–9)
  - Sprites 62–67: 8×8 punctuation (`.` `:` `;` `,` `!` `'`)
- The decompiled menu/info/high-score calls pass one-based first-letter sprite
  index `0x1b`, so the original normal UI text uses the 8×8 range.
- Original game strings use custom encoding (`:`→`.`, `;`→`:`, `=`→`,`, `>`→`!`, `?`→`'`)
- Variable-width text rendering accounting for per-sprite dimensions ✅

### Gameplay
- Title screen → Main menu → Level intro → Playing → Level complete flow ✅
- Player movement, gravity, jumping, collision with tile map ✅
- Down-key crouch plus special platform/teleporter activation ✅
- Bomb placement with 4-frame arming momentum, fuse timer, explosion with terrain destruction ✅
- Original starting inventory: 99 basic bombs only ✅
- Bomb-box inventory awards now route through the recovered `FUN_1000_5715`
  byte-wrapping capped-add helper ✅
- Shipped object-id `0x1f` monsters now advance through decoded original
  motion records in the live update; live preprocessing consumes the original
  RNG helper and a 128-phase trig table approximation. The shipped non-`0x1f`
  object-id `0x1e` state-6 path is live; remaining movement fidelity work is
  exact Pascal real trig scaling plus non-shipped/unmapped object-state paths 🟡
- Shipped monster object-id coverage is pinned: 14 spawns use object id `0x1f`
  and the one remaining non-`0x1f` spawn is object id `0x1e` in level index 2,
  spawn index 2 at runtime position `(880,248)` ✅
- Powerup drops from killed monsters ✅
- Bomb box bonus quantities match original random ranges ✅
- Original `FUN_1920_13a8`/`FUN_1920_13f7` RNG stepping is pinned as a pure
  helper (`seed = seed * 0x08088405 + 1`, high-word modulo) ✅
- New games seed the original RNG helper from DOS-style time fields
  (`DX:CX`) instead of a fixed zero seed ✅
- The common centered RNG pattern (`FUN_1920_13a8(n) - n/2`) is centralized in
  `OriginalRng::gen_centered` ✅
- Bomb box bonus quantities now consume the original RNG helper instead of
  `macroquad::rand` ✅
- Explosion/collapse debris velocities now consume the original RNG helper with
  the `FUN_1920_13a8(600) - 300` word range ✅
- Incidental monster-drop selection consumes the original RNG helper; the
  original `FUN_1000_6053` threshold-selection algorithm is pinned as a
  parameterized helper, including the `object_id = bonus_id + 0x13`,
  `selector_id = bonus_id + 0x3e`, countdown `100`, animation clear, and
  `-200` Y-velocity side effects; the original low-memory threshold bytes at
  `0x52+` are recovered from `assets/LEZAC.EXE` and checked in as constants ✅
- JollyCloud bridge spawn offsets consume the original RNG helper instead of
  host randomness ✅
- The `FUN_1000_6053` pickup effect-id branch is pinned for ids `2..6`,
  including vitality set/add-cap, `0x2e` invincibility, yellow/green bomb-box
  RNG ranges, inventory-dirty flag, and sound offset `8` priority `5` ✅
- Game/monster modules have a regression guard preventing direct
  `macroquad::rand`/`gen_range` use from returning ✅
- Dropped bonus gravity uses the original 8.8 falling-entity scale ✅
- Energy/health system, lives, respawning ✅
- Monster-player contact uses the original `< 10 px` X/Y delta check ✅
- Monster contact damage now follows the original hit-counter tick shape:
  one energy point per overlapping monster per 70 Hz frame ✅
- Fire-to-continue respawns at the death position ✅
- Continue is blocked when too few bonuses remain to finish the level ✅
- Continue restores original bomb minimums: basic at least 100, medium at least
  10, and large at least 2 ✅
- Waiting after all active players die restarts the level from the beginning ✅
- Destruction percentage tracking for level completion ✅
- Score awards use saturating `u32` accumulation consistently, including
  powerup collection ✅
- Level completion applies and displays the original destruction bonus
  (destroyed building tiles × 10) and leftover bomb bonus
  (medium × 100, large × 500, super × 2000) ✅
- Level-completion bonus check uses collected required bonuses only; live
  remaining level-goal bonuses are counted separately for continue feasibility ✅
- Only level-goal pickups increment required bonus progress ✅
- Tile-map bonus pickups (`0x13..0x1D`) clear the tile, count toward the level
  goal, and apply the known bonus effects ✅
- Level-goal bonuses persist; incidental monster/cloud drops still expire ✅
- Level completion no longer depends on overlapping high special markers
  (`tile > 0x6c`); those tiles are destruction side-effect markers, while the
  original completion gate is the collected-bonus and destruction pair at
  `0x79c5 && 0x79c6` ✅
- HUD: energy bar, score, bomb count, lives, bonus/destruction targets, two-player rows ✅
- HUD destruction progress uses numeric font fields only; `FONTS.SPR` has no
  percent glyph ✅
- 2-player support (shared screen, two-player HUD and shared scroll target) ✅
- Italian/English language toggle ✅
- Level-complete score lines follow the active Italian/English language ✅
- Game-over score lines follow the active Italian/English language ✅
- Info and Instructions screens now page through the extracted original
  English/Italian text blocks instead of the previous shortened summaries ✅
- Instruction text preserves extracted `*` blank-line markers and original
  Italian `PIU'ABBASTANZA` spacing ✅
- Records screen shows the stored name, reached level, and score from `RECS.DAT` ✅
- New-record name entry matches the original A-Z/space input filter and stores
  accepted letters lowercase ✅
- Empty high-score names serialize as the original space-padded name buffer
  instead of a synthetic fallback name ✅
- High-score entry renders the original 8-slot period placeholder buffer ✅
- Record loading preserves leading spaces in the original 8-byte name field ✅
- Scrolling background (parallax) ✅
- PC-speaker-style playback for `PROEFS.SON` sound effects, including
  menu selection, jump/teleport/pickup/collapse/hurt/death/level-complete, and
  high-score confirm triggers with original-style priority preemption ✅
- Proper level restart/new-game reset of destroyed tiles and level-scoped counters ✅

### Rendering
- VGA Mode 13h pixel-perfect 320×200 render target ✅
- Aspect-ratio-preserving upscale to any window size ✅
- Tile rendering with BOMOMIMK sprite indices and palette-color fallback ✅
- Sprite rendering with transparency (palette index 0) ✅
- Player sprites use the original PROVA.SPR directional frame ranges ✅
- Explosion rendering uses the original PROVA.SPR 40–43 sequence ✅
- Debris particles from explosions ✅

---

## What Needs Work

### 🔴 Title/Background Asset Fidelity
- **CARO.CAR** now matches the 8,450-byte raw title card format:
  `[padding:u8] [width:u8=132] [pixels:132×64]` ✅

### ✅ SFONLEF.ZBG Background
- File has: 2-byte header + 13 gradient palette entries (6 bytes each, defining
  start/end RGB color ramps) + 3-byte triplet RLE compressed 320×200 image
- The gradient entries define a custom 256-color palette built by interpolation
- Current code parses the 2-byte ramp header, builds the 13-entry gradient
  palette, and decompresses pixels from the correct 80-byte data offset ✅
- During gameplay this serves as the scrolling parallax background.

### 🟡 GRAN.MST Monster/Entity Data
- File is 399 bytes: 1 byte count, 7 × 38-byte behavior template records,
  then trailing sprite/offset/animation tables
- Current code parses the fixed 38-byte template records ✅
- Fixed-record byte 0 is now decoded as the original object/entity id instead
  of being mistaken for a gravity flag, and spawned monsters retain it ✅
- Current code uses the first trailing table bytes as per-type monster sprite bases ✅
- Placeholder monster animation now wraps shipped sprite bases within same-size
  `BOMOMIMK.SPR` runs so it does not advance into unrelated sprite dimensions ✅
- Trailing data is now partitioned like `FUN_1000_08a5`: 7 sprite bytes,
  7 X/Y offset pairs, a motion-count byte, and 6 × 16-byte motion/animation
  records ✅
- Spawned monsters retain the decoded object id, the anchor-index-selected X/Y
  offset pair, and the full GRAN.MST X/Y offset table for motion preprocessing ✅
- Spawned monsters retain the decoded fixed-record motion sequence ids from
  their selected GRAN.MST template ✅
- Spawned monsters resolve decoded motion sequence ids to the corresponding
  16-byte runtime motion records ✅
- High-bit motion sequence ids are resolved with the original reverse flag
  semantics from `FUN_1000_5872` ✅
- Runtime monsters can apply resolved `FUN_1000_5872` motion records into their
  original-style motion accumulator/state ✅
- Runtime monsters have a live object-id `0x1f` update branch that preprocesses
  motion records, applies `FUN_1000_5872`-style fields, advances signed 8.8
  position, and syncs visible coordinates ✅
- The live object-id `0x1f` motion branch now matches the original first-id
  gate and resets countdown byte `0x02` to `0xfa` before applying motion
  records ✅
- Live monster motion uses a shared runtime copy of the decoded motion records
  and advances/preprocesses that table once per frame before monsters consume
  it, matching the `FUN_1000_432a` → `FUN_1000_6053` call order ✅
- Live monster-motion random preprocessing now consumes `OriginalRng` words for
  bounded records instead of passing zero placeholder inputs ✅
- Live monster-motion absolute preprocessing now uses a 128-phase `f64` sine
  table approximation instead of passing a zero trig table, with quadrant and
  anti-symmetry invariants pinned; `FUN_1000_26e8` is recovered as the original
  128-entry × 6-byte real table initializer at `0x7bda`, but bit-exact Turbo
  Pascal real helper behavior/scaling remains unresolved 🟡
- Runtime monsters can preprocess their resolved motion records in place with
  injected random/trig inputs, matching the original `FUN_1000_432a` record
  rewrite step before `FUN_1000_5872` consumes them ✅
- Runtime monsters have a tested original-motion tick that preprocesses records,
  applies them, advances signed 8.8 position, and syncs visible coordinates ✅
- Shipped level monster spawns are covered by an integration test that verifies
  decoded object ids, anchor offsets, and resolved motion records are present ✅
- Initial copied words at the original `FUN_1000_5872` runtime offsets are
  named: anchor index bytes, phase fields, base words, limit/sentinel, X word,
  Y word, and random-Y base ✅
- Fixed-record bytes `0x16..0x1c` are decoded and carried as the original
  animation seed block (frame, frame bounds, counter, delay, mode, step) ✅
- Fixed-record bytes `0x1d..0x23` are decoded and carried as the original
  mode-3 animation backup block ✅
- The top-of-`FUN_1000_6053` animation counter/frame update is pinned as a
  pure helper for wrap and bounce modes, and mode-3's seven-byte restore from
  the backup block is pinned with the original copy direction ✅
- `FUN_1000_06ab` animation setup is pinned as the seven-byte
  `[min, min, max, delay, delay, mode, 1]` block constructor ✅
- The two `FUN_1000_6053` landing animation setup call shapes are named as
  pure helpers for backup-idle and active landing blocks ✅
- The `FUN_1000_6053` cleanup animation selector branch is pinned as a
  parameterized helper: object ids `< 0x13` use low-memory byte `0x6a`, others
  use `0x6c`, and both use max byte `0x6d` ✅
- `FUN_1000_5a75` selector application now has its recoverable byte `0x14`
  arithmetic pinned: the original stores `0x10 - selector_entry[1]`; the actual
  selector table bytes appear runtime-loaded or otherwise outside the static
  initialized bytes recovered from `assets/LEZAC.EXE` 🟡
- `FUN_1000_5872` motion accumulator behavior is pinned as a pure helper ✅
- The non-`-1` random preprocessing branch of `FUN_1000_432a` is pinned with
  injected random words ✅
- The `-1` motion-record phase advance from `FUN_1000_432a` is pinned ✅
- The deterministic absolute-branch trig phase pairing is pinned: X uses
  `(phase + 0x20) & 0x7f`, Y uses `phase` after advance ✅
- The deterministic shell of the `-1` trig preprocessing branch is pinned with
  injected trig offsets ✅
- Fixed-record bytes `0x0e..0x10` are decoded as the motion sequence ids passed
  to `FUN_1000_5872` for original object id `0x1f` ✅
- Real shipped spawn coverage for the remaining non-`0x1f` branch is pinned:
  only one object-id `0x1e` entity exists in LIVELS.SCH, so that branch can be
  ported and verified as a focused case ✅
- Fixed-record byte `0x01` is decoded as the offset-table anchor index used by
  `FUN_1000_6053`, and is no longer used as placeholder movement speed ✅
- Fixed-record byte `0x02` is decoded as the original mutable
  countdown/removal counter used by death/countdown and state-6 damage paths;
  runtime monsters carry it and shipped initial values are `[1,4,4,4,4,4,4]` ✅
- Fixed-record bytes `0x03` and `0x04` are decoded as the negative/positive
  horizontal animation range selectors used by the `local_33 == 3` branch ✅
- Fixed-record byte `0x14` is decoded as the signed position-origin adjustment
  that `FUN_1000_6053` subtracts before state dispatch and adds back before
  storing position, and runtime monsters carry it; all shipped templates store
  `0` ✅
- Fixed-record byte `0x15` is decoded and carried as the original state selector
  (`local_33` in `FUN_1000_6053`): shipped templates are `[6,5,5,5,5,5,5]`,
  with the lone object-id `0x1e` entity entering the `FUN_1000_5cb0` state-6
  path ✅
- For that state-6 `0x1e` path, bytes `0x0e..0x0f` are decoded separately as
  the `FUN_1000_5cb0` tile scan rectangle size; the shipped object uses `5 × 4`
  tiles ✅
- The `FUN_1000_5cb0` state-6 rectangle collision scan is pinned as a pure
  helper: top/left/right use `tile <= 0x4c`, bottom uses `tile <= 0x52` ✅
- The deterministic `FUN_1000_5cb0` velocity response is pinned as a pure
  helper: bottom-clear gravity adds `0x40` before blocked axes bounce with
  `velocity = -(velocity / 2)` ✅
- The deterministic shell of `FUN_1000_6053` state `3` is pinned as a pure
  helper with injected collision/path decisions and random X word: grounded
  landing alignment, gravity cap, target/random horizontal velocity selection,
  edge/path inversion, and animation-refresh gating ✅
- The deterministic shell of `FUN_1000_6053` state `4` is pinned as a pure
  helper with injected target/random velocity words: support/top bounce, frame
  period gate, target-distance threshold, homing velocity selection, and
  `FUN_1920_13a8(range * 2) - range` random velocity selection ✅
- Live object-id `0x1e`, state-6 monsters now use the pinned deterministic
  `FUN_1000_5cb0` scan, velocity response, and signed 8.8 position advance ✅
- The deterministic `FUN_1000_5cb0` damage scan is pinned: it counts `0x75`
  (`'u'`) tiles in every other column of the state-6 rectangle ✅
- The separate `FUN_1000_56b6` four-tile damage/contact scan is pinned with its
  `0x75` tracking, `0x4d` solid threshold, loop gate, and signed damage delta ✅
- The `FUN_1000_6053` low-object damage response for object ids `1..8` is
  pinned as a parameterized helper: selector table byte `0x77 + id * 2 + slot`,
  byte `0x19 = byte[0x1a] - 4`, signed vitality update through byte `0x24`, and
  death transition to object id `0x0c`, state `2`, countdown `0x19` on negative
  vitality ✅
- The player-contact transform branch for object ids `0x13..0x1d` now uses the
  recovered low-memory selector bytes at `0x42`, including the transition to
  object id `0x0b`, state `5`, countdown `0x1a`, animation clear, X-velocity
  clear, and `FUN_1920_13a8(3)` Y-velocity randomization request ✅
- State-6 damage byte handling is pinned: fixed-record byte `0x24` is the
  budget, byte `0x02` is decremented when damage exceeds the budget, byte
  `0x24` wraps after subtraction, and `0xff` in byte `0x02` removes the object ✅
- Runtime monsters now carry fixed-record byte `0x24` as the original mutable
  vitality/damage-budget byte for every spawned template; state-6 still consumes
  the same byte through its dedicated budget path ✅
- Live object-id `0x1e`, state-6 monsters now apply the scanned `u` damage
  count to those bytes and mark the monster dead at the original `0xff` removal
  counter ✅
- The state-6 removal sound request from `FUN_1000_5bcc` is live: PROEFS.SON
  offset `0x3d` at priority `0x0c` plays when the damage scan removes the
  object ✅
- The state-6 removal transition fields from `FUN_1000_5bcc` are captured:
  object id `0x0e`, state byte `2`, countdown byte `0x3c`, and animation byte
  `0` ✅
- The dependent-object scan in `FUN_1000_5bcc` is live: object-id `0x1f`
  entities whose fixed-record byte `0x25` matches the dying object's word at
  offset `0x12` enter the same object `0x0e`/state `2` transition with the
  original `40 + FUN_1920_13a8(10)` countdown ✅
- Recovered object-id `0x0e` death transitions now tick countdown byte `0x02`
  by `(frame_counter & 1)` and clean up to object id `0`, state `5`, timer
  `0x12`, with zeroed X/Y velocity words ✅
- Recovered death transitions keep the carried original countdown byte `0x02`
  synchronized with the death timer mirror, including cleanup reset to `0x12` ✅
- The post-countdown effect branch now emits the recovered `0x0e` death
  transition's three debris effects, consumes the original Y-then-X
  `FUN_1920_13a8(600) - 300` velocity rolls, and pins the
  `FUN_1000_2f9f(5, 0x0f, 0x0b, 0x0d, xvel, yvel, x, y)` allocation plus
  `FUN_1000_06ab(2, 2, [0x6d], [0x6a], active_block)` animation shape ✅
- The randomized `FUN_1000_5cb0` impulse branch is live-wired: it runs on
  `frame % 0x1d == 0`, consumes RNG for sound, X impulse, and optional
  bottom-blocked Y impulse in the original order, and plays PROEFS.SON offset
  `0x69` when the original sound predicate hits ✅
- The state-6 random impulse direction source is mapped: `FUN_1000_5cb0` checks
  caller stack word `param_1 - 4`, which is `FUN_1000_6053`'s selected nearest
  live player X delta, and the port now uses that delta instead of current X
  velocity ✅
- Spawned monsters retain the decoded fixed-record anchor-table index ✅
- Fixed-record byte `0x05` is preserved as an unnamed source byte and its
  shipped values are pinned as `[0x00,0x02,0x06,0x06,0xbf,0x00,0x02]`; it is no
  longer used as placeholder contact damage, and no direct `FUN_1000_6053` read
  has been recovered yet 🟡
- Fixed-record words `0x06`, `0x08`, `0x0a`, and `0x0c` are decoded as the
  original movement seed words loaded by `FUN_1000_6053` ✅
- Spawned monsters retain the decoded fixed-record movement seed words ✅
- Monster spawning uses parsed `LIVELS.SCH` monster records ✅
- Need to map all template fields and trailing tables into original movement
  and animation behavior.

### 🟡 LIVELS.SCH Entity Data
- Level tiles (foreground + background) load correctly via 3-byte RLE ✅
- The data section after the two tile layers contains monster, bonus, and
  platform/teleporter records.
- Current code parses monster, bonus, and platform record blocks ✅
- Monster spawn records use 16-bit pixel coordinates ✅
- Monster spawn records now expose the recovered original spawn-controller
  fields from the loader/update path: active flag `0x08`, spawn count `0x09`,
  spawn budget `0x0a`, low-memory template selector `0x0b`, runtime word
  base/random pairs `0x0c..0x18`, vitality base/random bytes `0x18..0x19`,
  allocation-call byte `0x1a`, timer/reset `0x1b..0x1c`, and animation delay
  `0x1d` ✅
- Spawn record runtime word/vitality randomization is modeled with `OriginalRng`
  using the original `FUN_1920_13a8(modulus)` shape ✅
- Spawn-controller timer/count/budget mutation is pinned: timer byte `0x1b`
  decrements before the zero check, resets from `0x1c` for allocation attempts,
  and count/budget bytes decrement after successful allocation ✅
- Spawn-controller event advancement now combines the original timer gate,
  recovered low-memory allocation tables, `FUN_1920_13a8` runtime-field RNG,
  and post-success count/budget commit into one tested helper ✅
- The live game loop now owns mutable spawn-controller records, advances them
  during play, and materializes successful original spawn events into runtime
  monsters with recovered animation/vitality fields ✅
- Spawn allocation request construction now has recovered original low-memory
  selector bytes `0x80/0x81` and animation range bytes `0x58/0x59` from
  `assets/LEZAC.EXE`; original table indexing and missing-table failure cases
  remain pinned ✅
- Spawn allocation requests now expose the recovered eight-argument
  `FUN_1000_2f9f` call shape used by the original spawn-controller path ✅
- The recovered `FUN_1000_2f9f` call shape can now run through the pure
  allocation-attempt helper using param 4 as the sprite selector id ✅
- Audited the checked-in Ghidra exports for selector-table writes/data
  definitions; only reads were found, and the apparent `FUN_1000_07fa(...,0x58,...)`
  references are screen fill coordinates rather than table initialization ✅
- Added the original executable to the repo and recovered the low-memory
  selector/threshold tables by mapping the `1aa2:0000` data segment from
  `assets/LEZAC.EXE` ✅
- `FUN_1000_2f9f` allocation velocity word clamping to `-0x7ff..0x7ff` is
  pinned as a pure helper ✅
- `FUN_1000_2f9f` allocation capacity gate is pinned: active object counts
  below `0x1e` allocate and increment, while `0x1e+` reports failure ✅
- `FUN_1000_2f9f` allocation position-origin byte is pinned: selector id
  `0x1f` writes zero, other selectors use `0x10 - selector_entry[1]` ✅
- The modeled `FUN_1000_2f9f` param 4 selector is word-sized, matching the
  decompiled `int param_4` signature and callers that pass `0x2072` as a word ✅
- The recovered `FUN_1000_2f9f` capacity, velocity clamp, and position-origin
  rules are exposed together as a pure allocation-attempt helper ✅
- Seven-byte level entity records use 16-bit pixel coordinates (`raw[2..4]` x,
  `raw[4..6]` y); `raw[6]` is the original flag byte used by the player-start
  scan, and shipped values are limited to `0`, `1`, and `2` ✅
- Seven-byte level entity records expose `raw[0..2]` as the original
  `FUN_1000_5999` map-cell reference (`tile_attr & 0x7fff`) ✅
- Teleporter targets follow `FUN_1000_5999`: `tile_attr & 0x7fff` must match a
  seven-byte map-cell reference; Rust-only platform/furthest-tile fallbacks were
  removed ✅
- The `FUN_1000_5999` teleport target effect allocation and animation setup
  shape is pinned with injected low-memory selector bytes `0x6c/0x6d` ✅
- Seven-byte player-start records are no longer spawned as collectible
  powerups ✅
- Platform records now expose the decompiled 14-byte action layout: affected
  map-cell-reference range, trigger map-cell reference/mixed destination-x,
  destination y, and four source→replacement tile substitutions ✅
- Pressing down on a `0x27` special platform with a non-zero attribute now runs
  the decoded platform action substitution table and plays the original
  PROEFS.SON offset `0x27` request at priority `6` ✅
- Platform actions now pin the `FUN_1000_5740` last-matching-trigger-record
  selection rule explicitly ✅
- Player start positions now use the original seven-byte records; non-start
  records carry flag `0` in shipped data and spawn as baseline level-goal
  pickups ✅

### 🟡 Gameplay Fidelity
- **Tile rendering**: Uses BOMOMIMK sprites by tile index with a palette-color
  fallback for missing sprite indices ✅
- **Palette cycling**: Textures retain indexed pixels and refresh the original
  `0xE6..0xEB` red-ramp animation on the original post-increment five-frame
  cadence ✅
- **Sprite indices**: Player directional ranges now match PROVA.SPR; monster
  sprite bases and GRAN.MST animation seed bytes are decoded, and the original
  counter/wrap/bounce/mode-3 restore helper is now used by live original-backed
  monsters; remaining work is the original state transitions that rewrite the
  active/backup animation blocks.
- **Bomb sprites**: placed bombs and HUD selection now use original
  `bomb_type + 0x39` PROVA.SPR sprites 58–61 ✅
- **Explosion sprites**: active explosions now use the PROVA.SPR 40–43
  animation sequence ✅
- **Powerup sprites**: known bonus ids now draw from the original
  `BOMOMIMK.SPR` collectible tile range `0x13..` ✅
- **Physics**: Player gravity/jump/movement and the original `0x641`
  landing-bounce cutoff use fixed-point-derived constants ✅
  Monster gravity now uses the `FUN_1000_5102` signed-byte accumulator scale ✅
  Monster signed 8.8 axis stepping is pinned with tests ✅
  Runtime monsters retain an original-style signed 8.8 motion state ✅
  The `FUN_1000_5b86` horizontal ground-friction helper is pinned as a signed
  8.8 velocity threshold/step primitive ✅
  Original falling-object gravity is pinned at the word level (`+0x40`, cap `0x7ff`) ✅
  The `local_33 == 2` motion branch is pinned as a pure helper: grounded
  downward velocity snaps to zero and floors Y to an 8-pixel boundary, otherwise
  gravity applies, and `FUN_1000_5b86` friction runs only while grounded ✅
  Monster movement still needs exact state-machine extraction.
- **Monster AI**: all shipped monster spawns now route through decoded original
  motion/state-6 branches; the older patrol/chase/jump behavior remains only as
  a fallback for missing templates or non-shipped/dev data. Remaining fidelity
  work is exact original state-machine extraction and still-unnamed fields.
- **Level completion gate**: completion follows the decompiled `0x79c5 &&
  0x79c6` shape; high tiles (`>0x6c`) are treated as destruction side-effect
  markers, not as a player-overlap exit requirement ✅
- **Required bonus counter**: destruction of the level header's required tile
  now increments the collected counter used for `0x2088`; remaining sources are
  used for continue feasibility, not for completion ✅
- **Tile-hit helper**: the `FUN_1000_5afd` destructible/drop-source tile range
  (`'g'..'r'`), table indexing, attribute clear/high-attribute replacement, and
  caller damage-scan flag are pinned as a pure helper ✅
- **Directional tile-hit effect**: the surrounding `FUN_1000_6053` effect spawn
  now has its word-sized `FUN_1000_2f9f(5, 0x0c, 10, 0x2072, ...)` call shape,
  four direction offsets, animation clear, and `0x208e` effect-count gate pinned
  as a pure helper ✅
- **11-byte record lookup**: `FUN_1000_3a56` is pinned as a reverse scan that
  returns the one-based index of the last record whose offset-`0x02` word
  matches `0x2072` ✅
- **Attribute effect lookup**: `FUN_1000_3a7e`/`3b18` are pinned as high-bit
  attribute lookups over reverse-scanned 15-byte vs 11-byte record tables, with
  the only difference being which effect byte is copied ✅
- **Special platforms**: down-key drop-through and teleporter activation are implemented ✅
- **Crouch**: down-key crouch uses the PROVA.SPR jump/crouch frame group when
  not activating a special platform ✅
- **Single-player width controls**: E/R now adjust the visible width one tile at
  a time with original `0x15..0x28` bounds ✅
- **Frame counter**: the live frame counter now wraps as the original 16-bit
  word at `0x78c2` ✅
- **Invincibility bonus**: bonus type 4 now assigns the exact `0x2e` frame
  countdown instead of extending a longer timer ✅
- **Screen width adjustment**: E/R keys adjust visible playfield width in one-player mode ✅

### 🟢 Polish & Cleanup
- WASM asset loading now embeds shipped files with `include_bytes!`, so
  `wasm32-unknown-unknown` builds do not panic during startup asset reads ✅
- WASM high-score saving/loading persists the original encoded `RECS.DAT`
  bytes in browser `localStorage`, falling back to embedded defaults when no
  browser record table exists ✅
- The large 10×10 font sprites (indices 0–25) are documented and mapped by the
  original first-letter offset, but no shipped `FUN_1000_136e` call found so far
  uses that range for the current menu/info/high-score text.
- High score serialization helper writes original `RECS.DAT` layout and
  preserves the full 32-bit score word pair used by the decompiled ranking path ✅
- Game over/all-levels-complete → new high score entry screen with `RECS.DAT` save ✅

---

## Key Technical Findings

| File | Format | Status |
|------|--------|--------|
| BOMPAL.PAL | 768 bytes, 256×RGB, 6-bit/channel | ✅ Done |
| PROVA.SPR | 1B count + per-sprite (w,h,pixels) | ✅ Done |
| BOMOMIMK.SPR | Same as above | ✅ Done |
| FONTS.SPR | Same; 26×10×10 + 42×8×8 | ✅ Done |
| LIVELS.SCH | 8B header + 2× RLE tile layers + entity data | Tiles/entities/spawn-controller fields ✅, low-memory spawn tables recovered, live delayed spawning wired ✅ |
| RECS.DAT | 7 × 13-byte records | ✅ Done |
| GRAN.MST | Count + 7×38-byte templates + trailing tables | Templates/sprite bases/runtime motion/animation blocks ✅, shipped `0x1f` and `0x1e` paths live; full original state machine still incomplete |
| CARO.CAR | Raw 132×64 title card with 2-byte header | ✅ Done |
| SFONLEF.ZBG | Gradient palette defs + RLE image | ✅ Done |
| PROEFS.SON | PC speaker frequency/duration pairs | ✅ Done |

## Decompilation Reference

- Full Ghidra decompilation: `/home/exedev/larax-zaco/decompiled.c` (11202 lines)
- Key functions identified:
  - `FUN_1000_136e` — Text rendering (char→sprite mapping)
  - `FUN_1000_12db` — Single sprite draw to VGA
  - `FUN_18ac_05dc` — SPR file loader
  - `FUN_182d_0000` — 3-byte triplet RLE decompressor
  - `FUN_1000_030b` — CARO.CAR title screen loader (with palette fade-in)
  - `FUN_1000_5102` — Main game loop / entity update
  - `FUN_1000_6053` — Main game loop (outer)
