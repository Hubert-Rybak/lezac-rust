use macroquad::prelude::*;

pub struct Palette { pub colors: [(u8,u8,u8); 256] }
impl Palette {
    pub fn load(path: &str) -> Self {
        let d = std::fs::read(path).expect("pal");
        let mut c = [(0u8,0u8,0u8); 256];
        for i in 0..256 { c[i] = ((d[i*3] as u16*4).min(255) as u8, (d[i*3+1] as u16*4).min(255) as u8, (d[i*3+2] as u16*4).min(255) as u8); }
        Palette { colors: c }
    }
    pub fn to_color(&self, i: u8) -> Color { let (r,g,b) = self.colors[i as usize]; Color::new(r as f32/255.0, g as f32/255.0, b as f32/255.0, 1.0) }
    pub fn to_color_alpha(&self, i: u8, a: f32) -> Color { let (r,g,b) = self.colors[i as usize]; Color::new(r as f32/255.0, g as f32/255.0, b as f32/255.0, a) }
}

pub struct SpriteSheet { pub num_sprites: usize, pub width: usize, pub height: usize, pub textures: Vec<Texture2D> }
impl SpriteSheet {
    pub fn load(path: &str, pal: &Palette) -> Self {
        let d = std::fs::read(path).unwrap();
        let ns = d[0] as usize; let w = d[1] as usize; let h = d[2] as usize;
        let mut texs = Vec::new(); let mut pos = 4;
        for _ in 0..ns {
            let mut sd = vec![0u8; w*h];
            for row in 0..h { for col in 0..w { if pos >= d.len() { break; } let b = d[pos]; pos += 1; if b == 0xFF { break; } sd[row*w+col] = b; } }
            let mut rgba = vec![0u8; w*h*4];
            for i in 0..w*h { let idx = sd[i]; if idx != 0 { let (r,g,b) = pal.colors[idx as usize]; rgba[i*4]=r; rgba[i*4+1]=g; rgba[i*4+2]=b; rgba[i*4+3]=255; } }
            let t = Texture2D::from_rgba8(w as u16, h as u16, &rgba); t.set_filter(FilterMode::Nearest); texs.push(t);
        }
        SpriteSheet { num_sprites: ns, width: w, height: h, textures: texs }
    }
    pub fn draw(&self, si: usize, x: f32, y: f32) { if si < self.textures.len() { draw_texture(&self.textures[si], x, y, WHITE); } }
    pub fn draw_flipped(&self, si: usize, x: f32, y: f32) {
        if si < self.textures.len() { draw_texture_ex(&self.textures[si], x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(-(self.width as f32), self.height as f32)), ..Default::default() }); }
    }
}

#[derive(Clone, Debug)]
pub struct HighScore { pub byte0: u8, pub score: u16, pub name: String }
pub fn load_records(path: &str) -> Vec<HighScore> {
    let d = match std::fs::read(path) { Ok(d) => d, Err(_) => return default_records() };
    let mut r = Vec::new(); let mut p = 0;
    while p+13 <= d.len() && r.len() < 7 {
        r.push(HighScore { byte0: d[p], score: u16::from_le_bytes([d[p+1],d[p+2]]), name: String::from_utf8_lossy(&d[p+5..p+13]).trim().into() });
        p += 13;
    }
    if r.is_empty() { default_records() } else { r }
}
fn default_records() -> Vec<HighScore> {
    ["lara","stefano","leo","andrea","daniel","filippu","luciano"].iter().map(|n| HighScore { byte0: 0, score: 10000, name: n.to_string() }).collect()
}

pub struct Background { pub texture: Texture2D, pub width: usize, pub height: usize }
impl Background {
    pub fn load(path: &str, pal: &Palette) -> Self {
        let d = std::fs::read(path).expect("bg");
        let pd = &d[48..]; let w = 320usize; let h = pd.len()/w;
        let mut rgba = vec![0u8; w*h*4];
        for i in 0..(w*h).min(pd.len()) { let idx = pd[i]; let (r,g,b) = pal.colors[idx as usize]; rgba[i*4]=r; rgba[i*4+1]=g; rgba[i*4+2]=b; rgba[i*4+3]=255; }
        let t = Texture2D::from_rgba8(w as u16, h as u16, &rgba); t.set_filter(FilterMode::Nearest);
        Background { texture: t, width: w, height: h }
    }
}

pub fn load_title_screen(path: &str, pal: &Palette) -> Texture2D {
    let d = std::fs::read(path).expect("title");
    let pd = &d[2..]; let tp = 320*200;
    let mut px = Vec::with_capacity(tp); let mut p = 0;
    while px.len() < tp && p+2 < pd.len() {
        let c = pd[p]; let v1 = pd[p+1]; let v2 = pd[p+2]; p += 3;
        for _ in 0..((c>>4)+1) as usize { if px.len() < tp { px.push(v1); } }
        for _ in 0..((c&0xF)+1) as usize { if px.len() < tp { px.push(v2); } }
    }
    while px.len() < tp { px.push(0); }
    let mut rgba = vec![0u8; tp*4];
    for i in 0..tp { let idx = px[i]; let (r,g,b) = pal.colors[idx as usize]; rgba[i*4]=r; rgba[i*4+1]=g; rgba[i*4+2]=b; rgba[i*4+3]=255; }
    let t = Texture2D::from_rgba8(320, 200, &rgba); t.set_filter(FilterMode::Nearest); t
}
