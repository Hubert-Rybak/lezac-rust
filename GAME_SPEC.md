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
- Sprites 0-25: Letters A-Z, each 10×10
- Sprites 26-35: Digits 0-9, each 8×8
- Sprites 36-67: Punctuation/symbols, each 8×8
- Font color: palette index 0x01 (blue) with 0xFF outlines (dark red/shadow).

**Character mapping for text rendering:**
- `'a'-'z'` → font sprite index `(char - 'a')` (letters 0-25)
- `'0'-'9'` → font sprite index `(char - '0') + 26` (digits 26-35)
- Other characters → font sprite indices 36+ (symbols)

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
    palette_variant:  u8        // palette rotation index (106-111)
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

34,292 bytes of dual-run RLE compressed data. Decompresses to exactly 64,000 bytes (320×200 VGA screen). No header — the entire file is compressed pixel data.

The background is rendered behind the game field and scrolls with parallax.

### 1.7 CARO.CAR — Title Card Image

8,450 bytes. Format: `[padding:u8] [width:u8 = 132] [raw_pixels: 132×64 bytes]`

- Uncompressed raw pixel data, 132×64 pixels.
- Displayed centered on screen at X offset `(320-132)/2 = 94`.
- Contains the "LARAX & ZACO" logo with explosion graphics.

### 1.8 RECS.DAT — High Score Records

92 bytes = 1 byte header + 7 entries × 13 bytes each.

```
RECS {
    count: u8           // Number of entries (7)
    entries: [Record; 7]
}

Record {
    score:   u16 LE     // Score value (e.g., 10000)
    unknown: u16 LE     // Reserved/zero
    level:   u8         // Level reached or flags (0x08 in default)
    name:    [u8; 8]    // Player name, space-padded ASCII
}
```

Default entries have score=10000, names: "lara", "stefano", "leo", "andrea", "daniel", "filippu", "luciano".

### 1.9 PROEFS.SON — PC Speaker Sound Effects

782 bytes of sound effect data. Sound effects are addressed by **byte offset** within this file (not by index).

Each sound effect is a sequence of `[frequency_divider:u16 LE] [duration:u16 LE] [flags:u8] [flags2:u8]` entries (6 bytes each). The frequency divider is for the 8253 PIT timer (1,193,180 / divider = Hz).

Known sound offset IDs (stored in memory at 0x2074 before calling the sound function):
- `0x08`: Walk/step
- `0x12`: Jump
- `0x21`: Menu select
- `0x24`: Bomb place  
- `0x27`: Pickup
- `0x35`: Explosion
- `0x78`: Hit/damage

Priority system: Each sound has a priority level (0x799f). Higher priority sounds preempt lower ones. Priority values: 1-11.

### 1.10 GRAN.MST — Monster Type Definitions

399 bytes. First byte = 7 (likely number of monster types or related to 7 levels). Contains global monster behavior templates that define how different monster types move, chase, and animate. The exact per-field structure is complex (30+ bytes per type with behavior flags, speed, sprite references, animation data).

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
| 0x79B4 | 1 | Palette variant for current level |
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
- Friction/deceleration: When no direction key pressed, velocity gradually returns to 0
- Animation speed: `4 - abs(velocity) / 0x100` (faster movement = faster animation)

**Collision detection (4-point):**
The game checks 4 tiles around the player's position:
1. Above-left and above-right: ceiling check
2. Below-left and below-right: ground check
3. Left and right: wall check

Tile values ≤ 0x4C are considered solid for walking. Tiles > 0x4C with specific values (0x52+) have special behavior. Value `0x27` ('') triggers special platform activation (down key). Value `0x45` ('E') triggers teleport.

**Bounce off walls:** When hitting a wall, horizontal velocity is halved and reversed: `velocity = -velocity / 2`.

**Landing on ground:** When falling and hitting ground, vertical velocity is negated and divided by 4: `velocity = -velocity / 4` (slight bounce). If the result would be very small, velocity is set to 0.

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
| 53-56 | Bomb sprites (16×16) |
| 57 | Small bomb (8×8) |
| 58 | Bonus item (13×13) |
| 59-60 | Special items (16×16) |
| 61 | Collectible (10×12) |
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
2. **Arming:** 4-frame delay (`pbVar20[0x0E] = 4`), bomb moves with initial velocity matching player's momentum.
3. **Detonation:** After timer expires, the bomb explodes.
4. **Explosion:** Creates explosion sprite (sprite 0x39/57 area). Destroys surrounding tiles in a radius proportional to bomb power.
5. **Terrain destruction:** Each destroyed tile is replaced with empty (0x00). Building groups connected to destroyed tiles are evaluated — if the support structure is compromised, the entire group falls.

### 4.3 Bomb Inventory

Bomb counts per type stored at `(player_index * 4 + bomb_type + 0x1B67)`. Starting bomb count is 99 (displayed in HUD). Special bonuses can award more bombs.

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
2. **Gravity:** Monsters are affected by gravity. If not on ground, `dy += 4` per frame.
3. **Wall bouncing:** When a monster hits a wall, it reverses direction and sets a directional flag.
4. **Patrol behavior:** Monsters track their current column within their building group boundaries. They bounce between the left and right edges of their territory.
5. **Chase behavior:** When a player is within range, the monster adjusts its velocity toward the player (homing within territory bounds).
6. **Damage dealing:** When a monster overlaps a player (distance < 10 pixels both axes), it deals damage.
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
- Destroying buildings: Points proportional to tiles destroyed
- Killing monsters: Points from monster type table
- Bonus pickup types 2-6 give escalating rewards

### 6.3 Energy & Damage

- Energy starts at 100 and decreases when hit.
- Monster contact deals damage based on monster type and approach velocity.
- Falling from height deals damage.
- When energy ≤ 0, the player dies (state = 0x0C).
- On death, the player can press fire to continue (respawn at current position) or wait to restart the level from the beginning.
- Cannot re-enter a level if not enough bonuses remain to complete it.

### 6.4 Bonus Types

When a player touches a bonus entity (tile types 0x13-0x1D):
| Bonus | Effect |
|-------|--------|
| Type 2 | Full energy restore (set to 100) |
| Type 3 | Partial energy restore (+33, cap at 100) |
| Type 4 | Invincibility for 46 frames |
| Type 5 | Random bomb supply (random qty of random types) |
| Type 6 | Large bomb supply (more types, more qty) |

### 6.5 Level Completion

A level is completed when:
1. Bonuses collected (`0x2088`) + remaining collectible bonuses ≥ required bonuses (`0x2086`)
2. Building destruction percentage ≥ required percentage (`0x79B3`)
3. The player enters the exit area

The destruction percentage is calculated by counting tiles matching the level's palette variant value vs total building tiles.

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

FUN_1000_079d performs real-time palette cycling for certain color indices. 6 consecutive palette entries (starting at index 0xE6 = 230) are rotated each frame to create animated effects (flowing water, glowing items). The rotation adds 7 to the red component each step, wrapping at 63.

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

The game uses VGA vertical retrace for timing (`FUN_1920_0f0f/0efb/0f01/0f13` wait for vsync). At 70 Hz VGA refresh, this gives approximately 14ms per frame. The frame counter at 0x78C2 increments each frame.

---

## 9. LEVEL LOADING (FUN_1000_0c33)

1. Open LIVELS.SCH file
2. Seek to current level's data (sequential read, skip previous levels)
3. Read level header (width, height, palette, bonus_req, destruction_pct)
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
fn render_text(text: &str, x: i32, y: i32, font_offset: usize, spacing: u8) {
    for (i, ch) in text.bytes().enumerate() {
        let sprite_idx = if ch >= b'a' && ch <= b'z' {
            font_offset + (ch - b'a') as usize
        } else if ch >= b'0' && ch <= b'9' {
            font_offset + 26 + (ch - b'0') as usize
        } else if ch == b' ' {
            continue; // skip spaces
        } else {
            // Map punctuation to sprite indices 36+
            font_offset + 36 + map_symbol(ch)
        };
        draw_sprite(sprite_idx, x + (i as i32 * spacing as i32), y);
    }
}
```

Special text encoding (from game strings):
- `\` = newline
- `;` = colon `:` 
- `>` = period `.`
- `=` = comma `,`
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

Collecting bonus type 4 grants invincibility for `0x2E` (46) frames. During this time, the player flashes and monsters cannot deal damage.

### 11.4 Screen Width Adjustment

In single-player mode, pressing E/R adjusts the visible tile columns. The game stores the visible width at 0xC1EA and computes screen offsets accordingly. Default visible width is `0x28` (40 tiles = 320 pixels).

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
const DEATH_ANIMATION_FRAMES: u8 = 18;  // 0x12
const LEVEL_COMPLETE_DELAY: u8 = 100;   // 0x64
```
