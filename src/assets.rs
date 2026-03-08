use macroquad::prelude::*;

/// Load a file from disk (native) or from embedded data (WASM)
pub fn load_file(path: &str) -> Vec<u8> {
    #[cfg(not(target_arch = "wasm32"))]
    { std::fs::read(path).unwrap_or_else(|_| panic!("Failed to load: {}", path)) }
    #[cfg(target_arch = "wasm32")]
    { let _ = path; panic!("For WASM, use macroquad::file::load_file() async API"); }
}

pub struct Palette { pub colors: [(u8,u8,u8); 256] }
impl Palette {
    pub fn load(path: &str) -> Self {
        let d = load_file(path);
        let mut c = [(0u8,0u8,0u8); 256];
        for i in 0..256 {
            c[i] = (
                (d[i*3] as u16 * 4).min(255) as u8,
                (d[i*3+1] as u16 * 4).min(255) as u8,
                (d[i*3+2] as u16 * 4).min(255) as u8,
            );
        }
        Palette { colors: c }
    }
    pub fn to_color(&self, i: u8) -> Color {
        let (r,g,b) = self.colors[i as usize];
        Color::new(r as f32/255.0, g as f32/255.0, b as f32/255.0, 1.0)
    }
    pub fn to_color_alpha(&self, i: u8, a: f32) -> Color {
        let (r,g,b) = self.colors[i as usize];
        Color::new(r as f32/255.0, g as f32/255.0, b as f32/255.0, a)
    }
}

/// A single sprite with its own dimensions
pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub texture: Texture2D,
}

/// SPR file format:
/// - 1 byte: number of sprites
/// - Per sprite: 1 byte width, 1 byte height, width*height raw pixel bytes
///   (pixel 0 = transparent, 0xFF = opaque palette entry, NOT a row terminator)
pub struct SpriteSheet {
    pub sprites: Vec<Sprite>,
}

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
                    rgba[i*4] = r;
                    rgba[i*4+1] = g;
                    rgba[i*4+2] = b;
                    rgba[i*4+3] = 255;
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
pub struct HighScore { pub byte0: u8, pub score: u16, pub name: String }
pub fn load_records(path: &str) -> Vec<HighScore> {
    let d = match std::fs::read(path) { Ok(d) => d, Err(_) => return default_records() };
    let mut r = Vec::new(); let mut p = 0;
    while p+13 <= d.len() && r.len() < 7 {
        r.push(HighScore {
            byte0: d[p],
            score: u16::from_le_bytes([d[p+1],d[p+2]]),
            name: String::from_utf8_lossy(&d[p+5..p+13]).trim().into(),
        });
        p += 13;
    }
    if r.is_empty() { default_records() } else { r }
}
fn default_records() -> Vec<HighScore> {
    ["lara","stefano","leo","andrea","daniel","filippu","luciano"]
        .iter().map(|n| HighScore { byte0: 0, score: 10000, name: n.to_string() }).collect()
}

pub struct Background { pub texture: Texture2D, pub width: usize, pub height: usize }
impl Background {
    pub fn load(path: &str, pal: &Palette) -> Self {
        let d = load_file(path);
        // RLE-compressed background, header at offset 0..48, pixel data starts at 48
        let pd = &d[48..];
        let w = 320usize;
        let h = pd.len() / w;
        let mut rgba = vec![0u8; w * h * 4];
        for i in 0..(w * h).min(pd.len()) {
            let idx = pd[i];
            let (r, g, b) = pal.colors[idx as usize];
            rgba[i*4] = r; rgba[i*4+1] = g; rgba[i*4+2] = b; rgba[i*4+3] = 255;
        }
        let t = Texture2D::from_rgba8(w as u16, h as u16, &rgba);
        t.set_filter(FilterMode::Nearest);
        Background { texture: t, width: w, height: h }
    }
}

pub fn load_title_screen(path: &str, _pal: &Palette) -> Texture2D {
    let d = load_file(path);
    let w = 320usize;
    let h = 200usize;
    let total = w * h;

    // CARO.CAR format:
    // - 768 bytes: embedded VGA palette (6-bit per channel, 0-63, multiply by 4)
    // - 2 bytes: compressed data size (LE)
    // - N bytes: 3-byte triplet RLE compressed pixel data
    //   Each triplet [counts, val1, val2]:
    //     count1 = (counts >> 4) + 1 pixels of val1
    //     count2 = (counts & 0x0f) + 1 pixels of val2
    // NOTE: The compressed data only produces ~33k pixels (about half of 64000).
    // The title screen appears to use VGA Mode X (planar) or a two-pass scheme.
    // Until fully decoded, we decompress what we can and pad the rest.

    // Read embedded palette
    let mut car_pal = [(0u8, 0u8, 0u8); 256];
    if d.len() >= 768 {
        for i in 0..256 {
            car_pal[i] = (
                (d[i * 3].min(63) as u16 * 4).min(255) as u8,
                (d[i * 3 + 1].min(63) as u16 * 4).min(255) as u8,
                (d[i * 3 + 2].min(63) as u16 * 4).min(255) as u8,
            );
        }
    }

    // Decompress pixel data (3-byte triplet RLE starting after palette+size)
    let data_start = 770; // 768 palette + 2 size bytes
    let mut pixels = Vec::with_capacity(total);
    let mut pos = data_start;
    while pixels.len() < total && pos + 2 < d.len() {
        let counts = d[pos];
        let v1 = d[pos + 1];
        let v2 = d[pos + 2];
        pos += 3;
        let c1 = ((counts >> 4) + 1) as usize;
        let c2 = ((counts & 0x0F) + 1) as usize;
        for _ in 0..c1 {
            if pixels.len() < total { pixels.push(v1); }
        }
        for _ in 0..c2 {
            if pixels.len() < total { pixels.push(v2); }
        }
    }
    pixels.resize(total, 0);

    // Convert to RGBA using the embedded palette
    let mut rgba = vec![0u8; total * 4];
    for i in 0..total {
        let idx = pixels[i] as usize;
        let (r, g, b) = car_pal[idx];
        rgba[i * 4] = r;
        rgba[i * 4 + 1] = g;
        rgba[i * 4 + 2] = b;
        rgba[i * 4 + 3] = 255;
    }
    let t = Texture2D::from_rgba8(w as u16, h as u16, &rgba);
    t.set_filter(FilterMode::Nearest);
    t
}
