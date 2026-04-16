use crate::assets::rle_decompress;

/// Solid-for-walking tile threshold from FUN_1000_6053.
pub const TILE_SOLID_MAX: u8 = 0x4C;
/// Solid-for-ceiling threshold (slightly higher).
pub const TILE_CEIL_MAX: u8 = 0x52;

#[derive(Clone, Copy, Debug)]
pub struct MonsterSpawn { pub raw: [u8; 30] }

#[derive(Clone, Copy, Debug)]
pub struct BonusSpawn { pub raw: [u8; 7] }

#[derive(Clone, Copy, Debug)]
pub struct PlatformSpawn { pub raw: [u8; 14] }

#[derive(Clone)]
pub struct Level {
    pub width: usize,
    pub height: usize,
    pub palette_variant: u8,
    pub bonus_target: u16,
    pub destruction_pct: u8,
    pub tiles: Vec<u8>,           // w*h bytes
    pub attrs: Vec<u16>,          // w*h u16s
    pub orig_tiles: Vec<u8>,      // pristine copy for restart
    pub initial_variant_count: u32,
    pub scroll_x: u16,
    pub scroll_y: u16,
    pub monsters: Vec<MonsterSpawn>,
    pub bonuses: Vec<BonusSpawn>,
    pub platforms: Vec<PlatformSpawn>,
}

impl Level {
    pub fn tile_at(&self, x: usize, y: usize) -> u8 {
        if x < self.width && y < self.height { self.tiles[y * self.width + x] } else { 1 }
    }
    pub fn set_tile(&mut self, x: usize, y: usize, v: u8) {
        if x < self.width && y < self.height { self.tiles[y * self.width + x] = v; }
    }
    pub fn attr_at(&self, x: usize, y: usize) -> u16 {
        if x < self.width && y < self.height { self.attrs[y * self.width + x] } else { 0 }
    }

    /// Solid for walking/collision: non-zero and ≤ 0x4C.
    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        let t = self.tile_at(x, y);
        t != 0 && t <= TILE_SOLID_MAX
    }

    pub fn count_variant_tiles(&self) -> u32 {
        self.tiles.iter().filter(|&&t| t == self.palette_variant).count() as u32
    }

    pub fn reset(&mut self) {
        self.tiles.clone_from(&self.orig_tiles);
    }
}

fn read_u16(d: &[u8], p: &mut usize) -> u16 {
    let v = u16::from_le_bytes([d[*p], d[*p+1]]);
    *p += 2; v
}
fn read_u8(d: &[u8], p: &mut usize) -> u8 { let v = d[*p]; *p += 1; v }

fn read_rle_block(d: &[u8], p: &mut usize, output_size: usize) -> Vec<u8> {
    let comp_size = read_u16(d, p) as usize;
    let end = (*p + comp_size).min(d.len());
    let out = rle_decompress(&d[*p..end], output_size);
    *p = end;
    out
}

pub fn load_levels(path: &str) -> Vec<Level> {
    let d = std::fs::read(path).expect("LIVELS.SCH");
    let mut out = Vec::new();
    let mut p = 0;
    while p + 8 <= d.len() {
        let w = read_u16(&d, &mut p) as usize;
        let h = read_u16(&d, &mut p) as usize;
        if w == 0 || h == 0 || w > 500 || h > 500 { break; }
        let palette_variant = read_u8(&d, &mut p);
        let bonus_target = read_u16(&d, &mut p);
        let destruction_pct = read_u8(&d, &mut p);
        let n = w * h;
        let tiles = read_rle_block(&d, &mut p, n);
        let attr_bytes = read_rle_block(&d, &mut p, n * 2);
        let mut attrs = Vec::with_capacity(n);
        for i in 0..n {
            attrs.push(u16::from_le_bytes([attr_bytes[i*2], attr_bytes[i*2+1]]));
        }
        if p + 4 > d.len() { break; }
        let scroll_x = read_u16(&d, &mut p);
        let scroll_y = read_u16(&d, &mut p);
        if p >= d.len() { break; }

        let mcount = read_u8(&d, &mut p) as usize;
        let mut monsters = Vec::with_capacity(mcount);
        for _ in 0..mcount {
            if p + 30 > d.len() { break; }
            let mut raw = [0u8; 30];
            raw.copy_from_slice(&d[p..p+30]);
            monsters.push(MonsterSpawn { raw });
            p += 30;
        }

        if p >= d.len() { out.push(finalize(w, h, palette_variant, bonus_target, destruction_pct, tiles, attrs, scroll_x, scroll_y, monsters, vec![], vec![])); break; }
        let bcount = read_u8(&d, &mut p) as usize;
        let mut bonuses = Vec::with_capacity(bcount);
        for _ in 0..bcount {
            if p + 7 > d.len() { break; }
            let mut raw = [0u8; 7];
            raw.copy_from_slice(&d[p..p+7]);
            bonuses.push(BonusSpawn { raw });
            p += 7;
        }

        let platforms = if p < d.len() {
            let pcount = read_u8(&d, &mut p) as usize;
            let mut v = Vec::with_capacity(pcount);
            for _ in 0..pcount {
                if p + 14 > d.len() { break; }
                let mut raw = [0u8; 14];
                raw.copy_from_slice(&d[p..p+14]);
                v.push(PlatformSpawn { raw });
                p += 14;
            }
            v
        } else { vec![] };

        out.push(finalize(w, h, palette_variant, bonus_target, destruction_pct, tiles, attrs, scroll_x, scroll_y, monsters, bonuses, platforms));
        if out.len() >= 7 { break; }
    }
    out
}

fn finalize(
    w: usize, h: usize, palette_variant: u8, bonus_target: u16, destruction_pct: u8,
    tiles: Vec<u8>, attrs: Vec<u16>, scroll_x: u16, scroll_y: u16,
    monsters: Vec<MonsterSpawn>, bonuses: Vec<BonusSpawn>, platforms: Vec<PlatformSpawn>,
) -> Level {
    let orig_tiles = tiles.clone();
    let initial_variant_count = tiles.iter().filter(|&&t| t == palette_variant).count() as u32;
    Level {
        width: w, height: h, palette_variant, bonus_target, destruction_pct,
        tiles, attrs, orig_tiles, initial_variant_count,
        scroll_x, scroll_y, monsters, bonuses, platforms,
    }
}

/// GRAN.MST: [count:u8] then behavior templates. Format per template is not fully documented;
/// we keep raw bytes and expose them to AI for a sprite base + animation hints.
#[derive(Clone, Debug)]
pub struct MonsterTemplate {
    pub sprite_base: u8,
    pub speed: u8,
    pub raw: Vec<u8>,
}

pub fn load_monster_defs(path: &str) -> Vec<MonsterTemplate> {
    let d = match std::fs::read(path) { Ok(d) => d, Err(_) => return default_templates() };
    if d.is_empty() { return default_templates(); }
    let count = d[0].max(1) as usize;
    let body = &d[1..];
    let per = body.len() / count.max(1);
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let s = i * per;
        let e = (s + per).min(body.len());
        let raw = body[s..e].to_vec();
        let sprite_base = raw.get(3).copied().unwrap_or(0x10 + i as u8 * 3);
        let speed = raw.get(1).copied().unwrap_or(2);
        out.push(MonsterTemplate { sprite_base, speed, raw });
    }
    if out.is_empty() { default_templates() } else { out }
}

fn default_templates() -> Vec<MonsterTemplate> {
    (0..7).map(|i| MonsterTemplate {
        sprite_base: 0x10 + i * 3,
        speed: 2,
        raw: vec![],
    }).collect()
}
