# Larax & Zaco — Rust Port: Progress & TODO

## Current Status

A playable Rust port of the 1996 DOS game "Larax & Zaco" by Zanobi Software,
built with macroquad 0.4. Compiles on native (Linux/macOS/Windows) and WASM targets.
The game renders levels with tiles, sprites, HUD, player movement, bombs, monsters,
powerups, and multi-screen scrolling. Font rendering is now correct.

## What Works

### Asset Loading
- **BOMPAL.PAL** — 256-color VGA palette (6-bit → 8-bit), fully decoded ✅
- **PROVA.SPR** — Player sprites (91 sprites, variable dimensions up to 48×20) ✅
- **BOMOMIMK.SPR** — Misc/monster sprites (91 sprites, variable dimensions) ✅
- **FONTS.SPR** — Font sprites (68 sprites: 26×10×10 large + 42×8×8 small) ✅
- **LIVELS.SCH** — Level tilemap data, 3-byte triplet RLE, 7 levels loaded ✅
- **RECS.DAT** — High score records (7 entries × 13 bytes) ✅
- **SPR format** fully reverse-engineered: 1-byte header (count) + per-sprite
  width/height + raw pixel data. Sprites have **varying** dimensions within
  a single file. No 0xFF row terminator — all bytes are raw palette indices. ✅

### Font Rendering
- Character mapping decoded from decompiled `FUN_1000_136e`: ✅
  - Sprites 0–25: 10×10 large/decorative font (a–z)
  - Sprites 26–51: 8×8 small font (a–z)
  - Sprites 52–61: 8×8 digits (0–9)
  - Sprites 62–67: 8×8 punctuation (`.` `,` `-` `=` `!` `'`)
- Original game strings use custom encoding (`:`→`.`, `;`→`,`, `>`→`!`, `?`→`'`)
- Variable-width text rendering accounting for per-sprite dimensions ✅

### Gameplay
- Title screen → Main menu → Level intro → Playing → Level complete flow ✅
- Player movement, gravity, jumping, collision with tile map ✅
- Bomb placement, fuse timer, explosion with terrain destruction ✅
- Monster AI: Walker, Chaser, Floater, Jumper types ✅
- Powerup drops from killed monsters ✅
- Energy/health system, lives, respawning ✅
- Destruction percentage tracking for level completion ✅
- Bonus collection tracking for level completion ✅
- HUD: energy bar, score, bomb count, lives, bonus/destruction targets ✅
- 2-player support (shared screen) ✅
- Italian/English language toggle ✅
- Info, Instructions, Records screens ✅
- Scrolling background (parallax) ✅

### Rendering
- VGA Mode 13h pixel-perfect 320×200 render target ✅
- Aspect-ratio-preserving upscale to any window size ✅
- Tile rendering with palette-based colors ✅
- Sprite rendering with transparency (palette index 0) ✅
- Horizontal sprite flipping for facing direction ✅
- Debris particles from explosions ✅

---

## What Needs Work

### 🔴 CARO.CAR Title Screen (Partially Decoded)
- File format: 768-byte embedded palette + 3-byte triplet RLE pixel data
- **Problem**: RLE decompression produces only ~33k of 64k pixels
- **Hypothesis**: The title screen uses VGA Mode X (4-plane) storage.
  Plane 0 decompresses perfectly to 16000 pixels from the first section.
  The full planar interleaving/reconstruction hasn't been implemented.
- The embedded palette has a few values >63 (possibly 8-bit vs 6-bit encoding)
- SFONLEF.ZBG also contains what appears to be the title screen image data
  (RLE-compressed from offset 80, with 13 gradient palette entries defining
  custom color ramps), but the gradient palette builder isn't implemented.
- **Workaround**: Currently renders partial title with embedded palette.

### 🔴 SFONLEF.ZBG Background
- File has: 2-byte header + 13 gradient palette entries (6 bytes each, defining
  start/end RGB color ramps) + 3-byte triplet RLE compressed 320×200 image
- The gradient entries define a custom 256-color palette built by interpolation
- **Current code** incorrectly skips 48 bytes and treats rest as raw pixels.
  Needs rewrite to: parse gradient palette, decompress RLE, apply custom palette.
- During gameplay this serves as the scrolling parallax background.

### 🟡 GRAN.MST Monster/Entity Data
- File is 399 bytes, parsed per-level with variable-length records
- Current code treats it as 7 fixed 57-byte records (incorrect)
- Actual format per level: 1 byte (num_entities) + num_entities × 38 bytes
  (entity records) + sprite indices + position pairs + extra sprite data
- Monster spawning currently uses procedural placement (find valid ground
  positions and distribute monsters evenly). Works but doesn't match original.
- Need to re-implement sequential file reading with proper record parsing.

### 🟡 LIVELS.SCH Entity Data
- Level tiles (foreground + background) load correctly via 3-byte RLE ✅
- Entity data between levels is present but **not parsed**
- The data section after the two tile layers contains entity spawn positions,
  special tile types, teleporters, etc.
- Current level parser finds next-level boundary heuristically; entity data
  is stored but ignored.

### 🟡 Gameplay Fidelity
- **Tile rendering**: Uses solid color rectangles mapped via palette. The original
  game likely uses tile sprites from one of the SPR files or has pattern fills.
- **Sprite indices**: Player/monster sprite index calculations are approximate.
  Need to verify against decompiled animation tables.
- **Bomb sprites**: BombType::sprite_index() returns 0–3, which may not match
  actual sprite indices in BOMOMIMK.SPR.
- **Powerup sprites**: Similar index mapping uncertainty.
- **Physics**: Gravity, jump velocity, movement speed are hand-tuned approximations.
  Need to extract exact values from decompiled constants.
- **Monster AI**: Behavior patterns (patrol, chase, jump timing) are approximations.
  The original uses lookup tables and timer-based state machines.
- **Special platforms**: The "down" key activates special platforms/teleporters
  in the original. Not implemented.
- **Screen width adjustment**: E/R keys to change play area width (original feature
  for varying monitor sizes). Not implemented.

### 🟡 Sound
- **PROEFS.SON**: PC speaker sound effect definitions (frequency/duration pairs)
- SoundManager is a stub — `play()` does nothing
- Could generate PCM audio from the frequency/duration data and play via
  macroquad's audio system

### 🟢 Polish & Cleanup
- WASM asset loading: `load_file()` panics on WASM. Need to use macroquad's
  async `load_file()` API and restructure initialization.
- The large 10×10 font sprites (indices 0–25) aren't used anywhere yet.
  They may be for the title/credit screens or in-game popups.
- High score saving (write back to RECS.DAT)
- Game over → new high score entry screen
- Proper level restart (currently reuses modified tile data after explosions)

---

## Key Technical Findings

| File | Format | Status |
|------|--------|--------|
| BOMPAL.PAL | 768 bytes, 256×RGB, 6-bit/channel | ✅ Done |
| PROVA.SPR | 1B count + per-sprite (w,h,pixels) | ✅ Done |
| BOMOMIMK.SPR | Same as above | ✅ Done |
| FONTS.SPR | Same; 26×10×10 + 42×8×8 | ✅ Done |
| LIVELS.SCH | 8B header + 2× RLE tile layers + entity data | Tiles ✅, Entities ❌ |
| RECS.DAT | 7 × 13-byte records | ✅ Done |
| GRAN.MST | Variable-length per-level entity defs | ❌ Needs rewrite |
| CARO.CAR | 768B palette + RLE (Mode X?) | ❌ Partial |
| SFONLEF.ZBG | Gradient palette defs + RLE image | ❌ Needs rewrite |
| PROEFS.SON | PC speaker frequency/duration pairs | ❌ Stub only |

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
