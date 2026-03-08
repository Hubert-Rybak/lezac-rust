# Larax & Zaco - Rust Port

A faithful 1:1 Rust port of **Larax & Zaco** (1996) by Stefano Zanobi (Zanobi Software),
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

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

To serve the WASM build, copy the resulting `.wasm` file and create an HTML wrapper,
or use a tool like `basic-http-server`.

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
| Fire   | N        | 0        |

- Press **Left + Right** together to change bomb type
- **S** toggles background scenery
- **E/R** adjusts playing field width
- **ESC** quits to menu

## Game Assets

The `assets/` directory contains the original game data files:

| File | Description |
|------|-------------|
| `BOMPAL.PAL` | VGA palette (256 colors, 6-bit RGB) |
| `PROVA.SPR` | Player sprite sheet (91 sprites, 16×16) |
| `BOMOMIMK.SPR` | Bomb/monster sprites (91 sprites, 16×16) |
| `FONTS.SPR` | Font characters (68 chars, 10×10) |
| `LIVELS.SCH` | Level data (7 levels, RLE compressed) |
| `SFONLEF.ZBG` | Parallax background image |
| `GRAN.MST` | Monster definitions (7 × 57 bytes) |
| `PROEFS.SON` | PC Speaker sound effects |
| `RECS.DAT` | High score records |
| `CARO.CAR` | Title screen image |

## Reverse Engineering Notes

### SPR Format
Header: `[num_sprites: u8, width: u8, height: u8, 0x00]`
Pixels stored row-by-row; `0xFF` = fill rest of row with transparent (0x00).

### Level Format (LIVELS.SCH)
Each level: `[w:u16LE, h:u16LE, extra:u16LE, bonus_target:u8, destruction_pct:u8]`
Followed by two RLE-compressed tile layers (foreground + background).
RLE: reads 3 bytes `[counts, val1, val2]` → `(counts>>4)+1` copies of val1,
then `(counts&0xF)+1` copies of val2.

### Credits

Original game: **Stefano Zanobi** (Zanobi Software, 1996)
Rust port: Decompiled and ported from `LEZAC.EXE` using Ghidra
