use macroquad::prelude::*;

pub fn load_file(path: &str) -> Vec<u8> {
    #[cfg(not(target_arch = "wasm32"))]
    { std::fs::read(path).unwrap_or_else(|_| panic!("Failed to load: {}", path)) }
    #[cfg(target_arch = "wasm32")]
    { let _ = path; panic!("For WASM, use macroquad::file::load_file() async API"); }
}

fn vga6to8(v: u8) -> u8 {
    let v = v & 0x3F;
    (v << 2) | (v >> 4)
}

#[derive(Clone)]
pub struct Palette { pub colors: [(u8, u8, u8); 256] }

impl Palette {
    pub fn load(path: &str) -> Self {
        let d = load_file(path);
        let mut c = [(0u8, 0u8, 0u8); 256];
        for i in 0..256 {
            c[i] = (vga6to8(d[i*3]), vga6to8(d[i*3+1]), vga6to8(d[i*3+2]));
        }
        Palette { colors: c }
    }
    pub fn to_color(&self, i: u8) -> Color {
        let (r, g, b) = self.colors[i as usize];
        Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
    }
    pub fn to_color_alpha(&self, i: u8, a: f32) -> Color {
        let (r, g, b) = self.colors[i as usize];
        Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a)
    }
}

/// Dual-run RLE used by LIVELS.SCH, SFONLEF.ZBG, CARO.CAR.
/// Each 3-byte triplet [ctrl, v1, v2]: (ctrl>>4)+1 copies of v1, then (ctrl&0xF)+1 copies of v2.
pub fn rle_decompress(src: &[u8], out_size: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(out_size);
    let mut i = 0;
    while out.len() < out_size && i + 2 < src.len() {
        let c = src[i]; let v1 = src[i+1]; let v2 = src[i+2];
        i += 3;
        for _ in 0..((c >> 4) + 1) as usize { if out.len() < out_size { out.push(v1); } }
        for _ in 0..((c & 0x0F) + 1) as usize { if out.len() < out_size { out.push(v2); } }
    }
    out.resize(out_size, 0);
    out
}

pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub texture: Texture2D,
}

/// SPR: `[count:u8] { [w:u8] [h:u8] [pixels:w*h] }*count`.
/// Pixel 0 = transparent; any other value is a palette index (including 0xFF).
pub struct SpriteSheet { pub sprites: Vec<Sprite> }

impl SpriteSheet {
    pub fn load(path: &str, pal: &Palette) -> Self {
        let d = load_file(path);
        let ns = d[0] as usize;
        let mut sprites = Vec::with_capacity(ns);
        let mut pos = 1;
        for _ in 0..ns {
            let w = d[pos] as usize;
            let h = d[pos + 1] as usize;
            pos += 2;
            let mut rgba = vec![0u8; w * h * 4];
            for i in 0..(w * h) {
                let idx = d[pos + i];
                if idx != 0 {
                    let (r, g, b) = pal.colors[idx as usize];
                    rgba[i*4] = r; rgba[i*4+1] = g; rgba[i*4+2] = b; rgba[i*4+3] = 255;
                }
            }
            pos += w * h;
            let t = Texture2D::from_rgba8(w as u16, h as u16, &rgba);
            t.set_filter(FilterMode::Nearest);
            sprites.push(Sprite { width: w, height: h, texture: t });
        }
        SpriteSheet { sprites }
    }

    pub fn num_sprites(&self) -> usize { self.sprites.len() }

    pub fn draw(&self, si: usize, x: f32, y: f32) {
        if si < self.sprites.len() {
            draw_texture(&self.sprites[si].texture, x, y, WHITE);
        }
    }

    pub fn draw_colored(&self, si: usize, x: f32, y: f32, color: Color) {
        if si < self.sprites.len() {
            draw_texture(&self.sprites[si].texture, x, y, color);
        }
    }

    pub fn draw_flipped(&self, si: usize, x: f32, y: f32) {
        if si < self.sprites.len() {
            let sp = &self.sprites[si];
            draw_texture_ex(&sp.texture, x, y, WHITE, DrawTextureParams {
                dest_size: Some(vec2(-(sp.width as f32), sp.height as f32)),
                ..Default::default()
            });
        }
    }

    pub fn sprite_width(&self, si: usize) -> usize {
        if si < self.sprites.len() { self.sprites[si].width } else { 8 }
    }
    pub fn sprite_height(&self, si: usize) -> usize {
        if si < self.sprites.len() { self.sprites[si].height } else { 8 }
    }
}

#[derive(Clone, Debug)]
pub struct HighScore { pub score: u16, pub level: u8, pub name: String }

/// RECS.DAT: [count:u8] { score:u16LE, unknown:u16LE, level:u8, name:[u8;8] }*count
pub fn load_records(path: &str) -> Vec<HighScore> {
    let d = match std::fs::read(path) { Ok(d) => d, Err(_) => return default_records() };
    if d.is_empty() { return default_records(); }
    let n = d[0] as usize;
    let mut r = Vec::new();
    let mut p = 1;
    for _ in 0..n {
        if p + 13 > d.len() { break; }
        let score = u16::from_le_bytes([d[p], d[p+1]]);
        let level = d[p+4];
        let name = String::from_utf8_lossy(&d[p+5..p+13]).trim().to_string();
        r.push(HighScore { score, level, name });
        p += 13;
    }
    if r.is_empty() { default_records() } else { r }
}
fn default_records() -> Vec<HighScore> {
    ["lara","stefano","leo","andrea","daniel","filippu","luciano"]
        .iter().map(|n| HighScore { score: 10000, level: 8, name: n.to_string() }).collect()
}

pub struct Background {
    pub texture: Texture2D,
    pub width: usize,
    pub height: usize,
}

impl Background {
    /// SFONLEF.ZBG: 2-byte header + 13 × 6-byte gradient entries + dual-run RLE → 64000 bytes (320×200).
    pub fn load(path: &str, pal: &Palette) -> Self {
        let d = load_file(path);
        const W: usize = 320;
        const H: usize = 200;
        const TOTAL: usize = W * H;
        // Skip 2-byte header + 13×6 gradient palette entries = 80 bytes.
        let data_start = 2 + 13 * 6;
        let pixels = if d.len() > data_start {
            rle_decompress(&d[data_start..], TOTAL)
        } else {
            vec![0u8; TOTAL]
        };
        let mut rgba = vec![0u8; TOTAL * 4];
        for i in 0..TOTAL {
            let idx = pixels[i];
            let (r, g, b) = pal.colors[idx as usize];
            rgba[i*4] = r; rgba[i*4+1] = g; rgba[i*4+2] = b;
            rgba[i*4+3] = if idx == 0 { 0 } else { 255 };
        }
        let t = Texture2D::from_rgba8(W as u16, H as u16, &rgba);
        t.set_filter(FilterMode::Nearest);
        Background { texture: t, width: W, height: H }
    }
}

pub struct TitleCard {
    pub texture: Texture2D,
    pub width: usize,
    pub height: usize,
}

/// CARO.CAR: [padding:u8] [width:u8=132] [raw pixels: width×height] — uncompressed.
/// 8450 bytes file → 132×64 image (132×64 + 2 header = 8450).
pub fn load_title_screen(path: &str, pal: &Palette) -> TitleCard {
    let d = load_file(path);
    let w = if d.len() >= 2 { d[1] as usize } else { 132 };
    let w = if w == 0 { 132 } else { w };
    let pixel_bytes = d.len().saturating_sub(2);
    let h = if w > 0 { pixel_bytes / w } else { 0 };
    let h = h.max(1);
    let total = w * h;
    let src = &d[2..2 + total.min(d.len() - 2)];
    let mut rgba = vec![0u8; w * h * 4];
    for i in 0..total.min(src.len()) {
        let idx = src[i];
        if idx != 0 {
            let (r, g, b) = pal.colors[idx as usize];
            rgba[i*4] = r; rgba[i*4+1] = g; rgba[i*4+2] = b; rgba[i*4+3] = 255;
        }
    }
    let t = Texture2D::from_rgba8(w as u16, h as u16, &rgba);
    t.set_filter(FilterMode::Nearest);
    TitleCard { texture: t, width: w, height: h }
}
