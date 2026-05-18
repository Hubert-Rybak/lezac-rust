# Larax & Zaco - Rust Port

An in-progress faithful Rust port of **Larax & Zaco** (1996) by Stefano Zanobi (Zanobi Software),
decompiled from the original DOS executable `LEZAC.EXE` using Ghidra.

## About the Original Game

Larax & Zaco is a platform-strategy game where you collect bonuses and destroy
buildings by strategically placing bombs. The game features:

- 7 levels of increasing difficulty
- 4 types of bombs with different power levels
- 7 types of power-ups dropped by defeated monsters
- 1-2 player simultaneous gameplay
- Parallax scrolling backgrounds
- Italian and English language support

## Building

### Native (Linux/macOS/Windows)

```bash
cargo build --release
```

### WebAssembly (WASM)

WASM builds compile with `wasm32-unknown-unknown`. The shipped data files are
embedded for `wasm32`, so startup does not depend on browser filesystem access.
High scores use the original `RECS.DAT` binary layout on native builds and are
stored in browser `localStorage` on web builds.

## Fidelity Status

The port uses the shipped asset formats, original RNG helper, decoded level
records, original player starts/teleport references, original score record
layout, and live original-style monster motion for the shipped `0x1f` and
`0x1e` monster paths. Remaining 1:1 blockers are tracked in `TODO.md`; the
largest current blockers are missing low-memory selector table contents from
the original binary and bit-exact Turbo Pascal real-number trig conversion.

### macOS Cross-compilation

```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

### Windows Cross-compilation

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## Controls

| Action | Player 1 | Player 2 |
|--------|----------|----------|
| Left   | Z        | ← Arrow  |
| Right  | X        | → Arrow  |
| Down   | C        | ↓ Arrow  |
| Jump   | M        | ↑ Arrow  |
| Fire   | N        | Numpad 0 |

- Press **Left + Right** together to change bomb type
- **S** toggles background scenery
- **E/R** adjusts playing field width
- **ESC** exits the game

## Game Assets

The `assets/` directory contains the original game data files:

| File | Description |
|------|-------------|
| `BOMPAL.PAL` | VGA palette (256 colors, 6-bit RGB) |
| `PROVA.SPR` | Player/object sprite sheet (91 variable-size sprites) |
| `BOMOMIMK.SPR` | Tile/monster/misc sprite sheet (91 variable-size sprites) |
| `FONTS.SPR` | Font sprites (26 large 10×10 + 42 small 8×8) |
| `LIVELS.SCH` | Level data (7 levels, RLE compressed) |
| `SFONLEF.ZBG` | Gradient palette + RLE parallax background image |
| `GRAN.MST` | Monster templates (7 × 38-byte records + trailing tables) |
| `PROEFS.SON` | PC Speaker sound effects |
| `RECS.DAT` | High score records (`score:u32`, `level:u8`, `name[8]`) |
| `CARO.CAR` | Raw 132×64 title screen image |

## Reverse Engineering Notes

### SPR Format
Header: `[num_sprites:u8]`, followed by one record per sprite:
`[width:u8] [height:u8] [pixels:width*height]`.
Sprite dimensions vary within each file. Pixel value `0` is transparent;
`0xFF` is a real palette color, not a row terminator.

### Level Format (LIVELS.SCH)
Each level: `[w:u16LE, h:u16LE, destruction_tile:u8, bonus_target:u16LE, destruction_pct:u8]`
Followed by two RLE-compressed tile layers (foreground + background).
RLE: reads 3 bytes `[counts, val1, val2]` → `(counts>>4)+1` copies of val1,
then `(counts&0xF)+1` copies of val2.

### Credits

Original game: **Stefano Zanobi** (Zanobi Software, 1996)
Rust port: Decompiled and ported from `LEZAC.EXE` using Ghidra
