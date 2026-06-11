use macroquad::prelude::*;

#[cfg(target_arch = "wasm32")]
const WASM_RECORD_STORAGE_KEY: &str = "lezac-rust:RECS.DAT";

pub fn load_file(path: &str) -> Vec<u8> {
    try_load_file(path).unwrap_or_else(|| panic!("Failed to load: {}", path))
}

pub fn try_load_file(path: &str) -> Option<Vec<u8>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::fs::read(path).ok()
    }
    #[cfg(target_arch = "wasm32")]
    {
        let name = path.rsplit('/').next().unwrap_or(path);
        Some(match name {
            "BOMPAL.PAL" => include_bytes!("../assets/BOMPAL.PAL").to_vec(),
            "PROVA.SPR" => include_bytes!("../assets/PROVA.SPR").to_vec(),
            "BOMOMIMK.SPR" => include_bytes!("../assets/BOMOMIMK.SPR").to_vec(),
            "FONTS.SPR" => include_bytes!("../assets/FONTS.SPR").to_vec(),
            "SFONLEF.ZBG" => include_bytes!("../assets/SFONLEF.ZBG").to_vec(),
            "CARO.CAR" => include_bytes!("../assets/CARO.CAR").to_vec(),
            "LIVELS.SCH" => include_bytes!("../assets/LIVELS.SCH").to_vec(),
            "GRAN.MST" => include_bytes!("../assets/GRAN.MST").to_vec(),
            "RECS.DAT" => include_bytes!("../assets/RECS.DAT").to_vec(),
            "PROEFS.SON" => include_bytes!("../assets/PROEFS.SON").to_vec(),
            _ => return None,
        })
    }
}

fn vga6to8(v: u8) -> u8 {
    let v = v & 0x3F;
    (v << 2) | (v >> 4)
}

#[derive(Clone)]
pub struct Palette {
    pub colors: [(u8, u8, u8); 256],
}

impl Palette {
    pub fn load(path: &str) -> Self {
        let d = load_file(path);
        let mut c = [(0u8, 0u8, 0u8); 256];
        for i in 0..256 {
            c[i] = (
                vga6to8(d[i * 3]),
                vga6to8(d[i * 3 + 1]),
                vga6to8(d[i * 3 + 2]),
            );
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
    pub fn animated(&self, phase: u8) -> Self {
        Palette {
            colors: animated_palette_colors(self.colors, phase),
        }
    }
}

pub fn next_palette_cycle_phase(phase: u8) -> u8 {
    let next = phase.saturating_add(7);
    if next > 0x3f {
        0x14
    } else {
        next
    }
}

pub fn animated_palette_colors(mut colors: [(u8, u8, u8); 256], phase: u8) -> [(u8, u8, u8); 256] {
    let mut red = phase;
    for slot in colors.iter_mut().skip(0xE6).take(6) {
        *slot = (vga6to8(red), 0, 0);
        red = next_palette_cycle_phase(red);
    }
    colors
}

fn indexed_rgba(pixels: &[u8], colors: &[(u8, u8, u8); 256], transparent_zero: bool) -> Vec<u8> {
    let mut rgba = vec![0u8; pixels.len() * 4];
    for (i, &idx) in pixels.iter().enumerate() {
        if transparent_zero && idx == 0 {
            continue;
        }
        let (r, g, b) = colors[idx as usize];
        rgba[i * 4] = r;
        rgba[i * 4 + 1] = g;
        rgba[i * 4 + 2] = b;
        rgba[i * 4 + 3] = 255;
    }
    rgba
}

/// Dual-run RLE used by LIVELS.SCH, SFONLEF.ZBG, CARO.CAR.
/// Each 3-byte triplet [ctrl, v1, v2]: (ctrl>>4)+1 copies of v1, then (ctrl&0xF)+1 copies of v2.
pub fn rle_decompress(src: &[u8], out_size: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(out_size);
    let mut i = 0;
    while out.len() < out_size && i + 2 < src.len() {
        let c = src[i];
        let v1 = src[i + 1];
        let v2 = src[i + 2];
        i += 3;
        for _ in 0..((c >> 4) + 1) as usize {
            if out.len() < out_size {
                out.push(v1);
            }
        }
        for _ in 0..((c & 0x0F) + 1) as usize {
            if out.len() < out_size {
                out.push(v2);
            }
        }
    }
    out.resize(out_size, 0);
    out
}

pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
    pub texture: Texture2D,
}

/// SPR: `[count:u8] { [w:u8] [h:u8] [pixels:w*h] }*count`.
/// Pixel 0 = transparent; any other value is a palette index (including 0xFF).
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
            let pixels = d[pos..pos + w * h].to_vec();
            let rgba = indexed_rgba(&pixels, &pal.colors, true);
            pos += w * h;
            let t = Texture2D::from_rgba8(w as u16, h as u16, &rgba);
            t.set_filter(FilterMode::Nearest);
            sprites.push(Sprite {
                width: w,
                height: h,
                pixels,
                texture: t,
            });
        }
        SpriteSheet { sprites }
    }

    pub fn update_palette(&self, pal: &Palette) {
        for sprite in &self.sprites {
            let rgba = indexed_rgba(&sprite.pixels, &pal.colors, true);
            sprite
                .texture
                .update_from_bytes(sprite.width as u32, sprite.height as u32, &rgba);
        }
    }

    pub fn num_sprites(&self) -> usize {
        self.sprites.len()
    }

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
            draw_texture_ex(
                &sp.texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(-(sp.width as f32), sp.height as f32)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn sprite_width(&self, si: usize) -> usize {
        if si < self.sprites.len() {
            self.sprites[si].width
        } else {
            8
        }
    }
    pub fn sprite_height(&self, si: usize) -> usize {
        if si < self.sprites.len() {
            self.sprites[si].height
        } else {
            8
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HighScore {
    pub score: u32,
    pub level: u8,
    pub name: String,
}

/// RECS.DAT: [count:u8] { score:u32LE, level:u8, name:[u8;8] }*count
pub fn load_records(path: &str) -> Vec<HighScore> {
    #[cfg(target_arch = "wasm32")]
    if let Some(records) = load_records_from_wasm_storage() {
        return records;
    }

    let d = match try_load_file(path) {
        Some(d) => d,
        None => return default_records(),
    };
    parse_records(&d)
}

fn parse_records(d: &[u8]) -> Vec<HighScore> {
    if d.is_empty() {
        return default_records();
    }
    let n = d[0] as usize;
    let mut r = Vec::new();
    let mut p = 1;
    for _ in 0..n {
        if p + 13 > d.len() {
            break;
        }
        let score = u32::from_le_bytes([d[p], d[p + 1], d[p + 2], d[p + 3]]);
        let level = d[p + 4];
        let name = String::from_utf8_lossy(&d[p + 5..p + 13])
            .trim_end()
            .to_string();
        r.push(HighScore { score, level, name });
        p += 13;
    }
    if r.is_empty() {
        default_records()
    } else {
        r
    }
}

pub fn save_records(path: &str, records: &[HighScore]) -> std::io::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::fs::write(path, encode_records(records))
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = path;
        save_records_to_wasm_storage(records)
    }
}

fn encode_records(records: &[HighScore]) -> Vec<u8> {
    let count = records.len().min(7);
    let mut out = Vec::with_capacity(1 + count * 13);
    out.push(count as u8);
    for rec in records.iter().take(count) {
        out.extend_from_slice(&rec.score.to_le_bytes());
        out.push(rec.level);
        let mut name = [b' '; 8];
        for (dst, src) in name.iter_mut().zip(rec.name.bytes()) {
            *dst = src;
        }
        out.extend_from_slice(&name);
    }
    out
}

#[cfg(target_arch = "wasm32")]
fn load_records_from_wasm_storage() -> Option<Vec<HighScore>> {
    let encoded = quad_storage::STORAGE
        .lock()
        .ok()?
        .get(WASM_RECORD_STORAGE_KEY)?;
    let bytes = decode_hex_bytes(&encoded)?;
    Some(parse_records(&bytes))
}

#[cfg(target_arch = "wasm32")]
fn save_records_to_wasm_storage(records: &[HighScore]) -> std::io::Result<()> {
    let mut storage = quad_storage::STORAGE
        .lock()
        .map_err(|_| std::io::Error::other("browser localStorage unavailable"))?;
    storage.set(
        WASM_RECORD_STORAGE_KEY,
        &encode_hex_bytes(&encode_records(records)),
    );
    Ok(())
}

fn encode_hex_bytes(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn decode_hex_bytes(encoded: &str) -> Option<Vec<u8>> {
    let bytes = encoded.as_bytes();
    if !bytes.len().is_multiple_of(2) {
        return None;
    }
    let mut out = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.chunks_exact(2) {
        out.push((hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?);
    }
    Some(out)
}

fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn default_records() -> Vec<HighScore> {
    [
        "lara", "stefano", "leo", "andrea", "daniel", "filippu", "luciano",
    ]
    .iter()
    .map(|n| HighScore {
        score: 10000,
        level: 8,
        name: n.to_string(),
    })
    .collect()
}

pub struct Background {
    pub texture: Texture2D,
    pub width: usize,
    pub height: usize,
    pixels: Vec<u8>,
    colors: [(u8, u8, u8); 256],
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
        let colors = sfonlef_palette(&d, pal);
        let rgba = indexed_rgba(&pixels, &colors, true);
        let t = Texture2D::from_rgba8(W as u16, H as u16, &rgba);
        t.set_filter(FilterMode::Nearest);
        Background {
            texture: t,
            width: W,
            height: H,
            pixels,
            colors,
        }
    }

    pub fn update_palette(&self, phase: u8) {
        let colors = animated_palette_colors(self.colors, phase);
        let rgba = indexed_rgba(&self.pixels, &colors, true);
        self.texture
            .update_from_bytes(self.width as u32, self.height as u32, &rgba);
    }
}

fn sfonlef_palette(data: &[u8], fallback: &Palette) -> [(u8, u8, u8); 256] {
    let mut colors = fallback.colors;
    if data.len() < 2 {
        return colors;
    }
    let start = data[0] as usize;
    let count = data[1] as usize;
    if count == 0 || start >= 256 || data.len() < 2 + count * 6 {
        return colors;
    }
    let span = 256 - start;
    for entry in 0..count {
        let ramp_start = start + entry * span / count;
        let ramp_end = start + (entry + 1) * span / count;
        if ramp_start >= ramp_end {
            continue;
        }
        let e = 2 + entry * 6;
        let from = (data[e], data[e + 1], data[e + 2]);
        let to = (data[e + 3], data[e + 4], data[e + 5]);
        let denom = (ramp_end - ramp_start).saturating_sub(1).max(1);
        for (step, slot) in (ramp_start..ramp_end).enumerate() {
            let r = lerp_vga6(from.0, to.0, step, denom);
            let g = lerp_vga6(from.1, to.1, step, denom);
            let b = lerp_vga6(from.2, to.2, step, denom);
            colors[slot] = (vga6to8(r), vga6to8(g), vga6to8(b));
        }
    }
    colors
}

fn lerp_vga6(from: u8, to: u8, step: usize, denom: usize) -> u8 {
    let from = from as i32;
    let to = to as i32;
    (from + ((to - from) * step as i32) / denom as i32).clamp(0, 63) as u8
}

pub struct TitleCard {
    pub texture: Texture2D,
    pub width: usize,
    pub height: usize,
    pixels: Vec<u8>,
}

/// CARO.CAR: [padding:u8] [width:u8=132] [raw pixels: width×height] — uncompressed.
/// 8450 bytes file → 132×64 image (132×64 + 2 header = 8450).
pub fn load_title_screen(path: &str, pal: &Palette) -> TitleCard {
    let d = load_file(path);
    let w = if d.len() >= 2 { d[1] as usize } else { 132 };
    let w = if w == 0 { 132 } else { w };
    let pixel_bytes = d.len().saturating_sub(2);
    let h = pixel_bytes.checked_div(w).unwrap_or(0);
    let h = h.max(1);
    let total = w * h;
    let src_len = total.min(d.len() - 2);
    let mut pixels = vec![0u8; total];
    if src_len > 0 {
        pixels[..src_len].copy_from_slice(&d[2..2 + src_len]);
    }
    let rgba = indexed_rgba(&pixels, &pal.colors, true);
    let t = Texture2D::from_rgba8(w as u16, h as u16, &rgba);
    t.set_filter(FilterMode::Nearest);
    TitleCard {
        texture: t,
        width: w,
        height: h,
        pixels,
    }
}

impl TitleCard {
    pub fn update_palette(&self, pal: &Palette) {
        let rgba = indexed_rgba(&self.pixels, &pal.colors, true);
        self.texture
            .update_from_bytes(self.width as u32, self.height as u32, &rgba);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_records_matches_recs_dat_layout() {
        let bytes = encode_records(&[
            HighScore {
                score: 10000,
                level: 8,
                name: "lara".to_string(),
            },
            HighScore {
                score: 70_000,
                level: 3,
                name: "longername".to_string(),
            },
            HighScore {
                score: 1,
                level: 1,
                name: String::new(),
            },
        ]);

        assert_eq!(bytes[0], 3);
        assert_eq!(&bytes[1..5], &10000u32.to_le_bytes());
        assert_eq!(bytes[5], 8);
        assert_eq!(&bytes[6..14], b"lara    ");
        assert_eq!(&bytes[14..18], &70_000u32.to_le_bytes());
        assert_eq!(bytes[18], 3);
        assert_eq!(&bytes[19..27], b"longerna");
        assert_eq!(&bytes[27..31], &1u32.to_le_bytes());
        assert_eq!(bytes[31], 1);
        assert_eq!(&bytes[32..40], b"        ");
    }

    #[test]
    fn hex_record_storage_round_trips_binary_recs_dat_bytes() {
        let bytes = encode_records(&[
            HighScore {
                score: 0x1234_5678,
                level: 7,
                name: "abc".to_string(),
            },
            HighScore {
                score: 1,
                level: 2,
                name: "UPPER".to_string(),
            },
        ]);

        let encoded = encode_hex_bytes(&bytes);

        assert_eq!(decode_hex_bytes(&encoded), Some(bytes.clone()));
        assert_eq!(decode_hex_bytes(&encoded.to_uppercase()), Some(bytes));
        assert_eq!(decode_hex_bytes("0"), None);
        assert_eq!(decode_hex_bytes("zz"), None);
    }

    #[test]
    fn bompal_pal_matches_original_vga_palette_entries() {
        let bytes = std::fs::read("assets/BOMPAL.PAL").unwrap();
        let palette = Palette::load("assets/BOMPAL.PAL");

        assert_eq!(bytes.len(), 256 * 3);
        assert_eq!(palette.colors[0x00], (0x00, 0x00, 0x00));
        assert_eq!(palette.colors[0x01], (0x00, 0x00, 0xaa));
        assert_eq!(palette.colors[0x04], (0xaa, 0x00, 0x00));
        assert_eq!(palette.colors[0x06], (0xaa, 0x55, 0x55));
        assert_eq!(palette.colors[0x0c], (0xff, 0x55, 0x55));
        assert_eq!(palette.colors[0xff], (0x55, 0x00, 0x00));
    }

    #[test]
    fn animated_palette_cycles_original_red_range() {
        let mut colors = [(0u8, 0u8, 0u8); 256];
        colors[0xE5] = (vga6to8(10), vga6to8(1), vga6to8(2));
        colors[0xE6] = (vga6to8(1), vga6to8(3), vga6to8(4));
        colors[0xEB] = (vga6to8(60), vga6to8(5), vga6to8(6));
        colors[0xEC] = (vga6to8(11), vga6to8(7), vga6to8(8));

        let animated = animated_palette_colors(colors, 0x3d);

        assert_eq!(animated[0xE5], colors[0xE5]);
        assert_eq!(animated[0xE6], (vga6to8(0x3d), 0, 0));
        assert_eq!(animated[0xE7], (vga6to8(0x14), 0, 0));
        assert_eq!(animated[0xEB], (vga6to8(0x30), 0, 0));
        assert_eq!(animated[0xEC], colors[0xEC]);
        assert_eq!(next_palette_cycle_phase(0x3d), 0x14);
    }

    #[test]
    fn fonts_spr_matches_original_large_and_small_glyph_layout() {
        let bytes = std::fs::read("assets/FONTS.SPR").unwrap();
        let count = bytes[0] as usize;
        let mut pos = 1;
        let mut dims = Vec::with_capacity(count);

        for _ in 0..count {
            let width = bytes[pos] as usize;
            let height = bytes[pos + 1] as usize;
            pos += 2 + width * height;
            dims.push((width, height));
        }

        assert_eq!(count, 68);
        assert_eq!(pos, bytes.len());
        assert!(dims[..26].iter().all(|&d| d == (10, 10)));
        assert!(dims[26..].iter().all(|&d| d == (8, 8)));
    }

    #[test]
    fn prova_spr_matches_original_object_sprite_dimensions() {
        let dims = sprite_dimensions("assets/PROVA.SPR");

        assert_eq!(dims.len(), 91);
        assert_eq!(dims[39], (48, 20));
        assert_eq!(dims[40], (22, 18));
        assert_eq!(dims[43], (22, 18));
        assert_eq!(dims[53], (16, 16));
        assert_eq!(dims[56], (16, 16));
        assert_eq!(dims[58], (13, 13));
        assert_eq!(dims[59], (16, 16));
        assert_eq!(dims[60], (16, 16));
        assert_eq!(dims[61], (10, 12));
        assert_eq!(dims[89], (21, 7));
        assert_eq!(dims[90], (12, 10));
    }

    #[test]
    fn bomomimk_spr_matches_original_tile_and_monster_sheet_shape() {
        let dims = sprite_dimensions("assets/BOMOMIMK.SPR");

        assert_eq!(dims.len(), 91);
        assert_eq!(dims[0], (16, 16));
        assert_eq!(dims[40], (16, 16));
        assert_eq!(dims[45], (17, 10));
        assert_eq!(dims[58], (13, 13));
        assert_eq!(dims[62], (16, 10));
        assert_eq!(dims[66], (16, 10));
    }

    fn sprite_dimensions(path: &str) -> Vec<(usize, usize)> {
        let bytes = std::fs::read(path).unwrap();
        let count = bytes[0] as usize;
        let mut pos = 1;
        let mut dims = Vec::with_capacity(count);
        for _ in 0..count {
            let width = bytes[pos] as usize;
            let height = bytes[pos + 1] as usize;
            pos += 2 + width * height;
            dims.push((width, height));
        }
        assert_eq!(pos, bytes.len());
        dims
    }

    #[test]
    fn recs_dat_round_trips_original_default_table() {
        let records = load_records("assets/RECS.DAT");
        let original = std::fs::read("assets/RECS.DAT").unwrap();

        assert_eq!(records, default_records());
        assert_eq!(encode_records(&records), original);
    }

    #[test]
    fn load_records_preserves_leading_name_spaces() {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "lezac-records-{}-{}.dat",
            std::process::id(),
            "leading-spaces"
        ));
        let bytes = encode_records(&[HighScore {
            score: 123,
            level: 4,
            name: "  ab".to_string(),
        }]);
        std::fs::write(&path, bytes).unwrap();

        let records = load_records(path.to_str().unwrap());

        std::fs::remove_file(&path).unwrap();
        assert_eq!(records[0].name, "  ab");
    }

    #[test]
    fn caro_car_matches_original_raw_title_card_shape() {
        let bytes = std::fs::read("assets/CARO.CAR").unwrap();
        let width = bytes[1] as usize;
        let pixels = &bytes[2..];

        assert_eq!(bytes[0], 0);
        assert_eq!(width, 132);
        assert_eq!(pixels.len(), 132 * 64);
        assert!(pixels.iter().any(|&p| p != 0));
    }

    #[test]
    fn sfonlef_zbg_matches_original_gradient_rle_shape() {
        let bytes = std::fs::read("assets/SFONLEF.ZBG").unwrap();
        let data_start = 2 + 13 * 6;
        let pixels = rle_decompress(&bytes[data_start..], 320 * 200);

        assert_eq!(bytes.len(), 34_292);
        assert_eq!(bytes[0], 0);
        assert_eq!(bytes[1], 13);
        assert_eq!(data_start, 80);
        assert_eq!(pixels.len(), 320 * 200);
        assert!(pixels.iter().any(|&p| p != 0));
    }

    #[test]
    fn sfonlef_palette_builds_header_defined_gradients() {
        let fallback = Palette {
            colors: [(1, 2, 3); 256],
        };
        let data = [0, 2, 0, 0, 0, 63, 0, 0, 0, 63, 0, 0, 0, 63];

        let colors = sfonlef_palette(&data, &fallback);

        assert_eq!(colors[0], (0, 0, 0));
        assert_eq!(colors[127], (255, 0, 0));
        assert_eq!(colors[128], (0, 255, 0));
        assert_eq!(colors[255], (0, 0, 255));
    }
}
