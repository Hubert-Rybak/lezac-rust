//! Sound effects — PROEFS.SON ported to PC-speaker-style square waves.
//!
//! PROEFS.SON is 782 bytes of 6-byte entries (per GAME_SPEC §1.9):
//!   `[freq_div:u16 LE] [duration:u16 LE] [flags:u8] [flags2:u8]`
//!
//! Sound effects are addressed by *byte offset* within the file. We read
//! sequential entries from the named offset, generate a square-wave PCM
//! buffer at `1_193_180 / freq_div` Hz for `duration` ticks, wrap it in a
//! WAV header, and hand the bytes to macroquad's `load_sound_from_bytes`.

use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};

const SAMPLE_RATE: u32 = 22050;
/// 8253 PIT base frequency.
const PIT_HZ: f32 = 1_193_180.0;
/// Original game ran at 70 Hz; durations look like "ticks of that timer".
const SECONDS_PER_TICK: f32 = 1.0 / 70.0;
/// Cap any single sound at ~0.6 s so a stuck duration field can't lock us up.
const MAX_SOUND_SECS: f32 = 0.6;

#[derive(Clone, Copy)]
pub enum SoundEffect {
    Walk,
    Jump,
    PlaceBomb,
    Explosion,
    Pickup,
    Hurt,
    Die,
    LevelComplete,
    MenuSelect,
}

impl SoundEffect {
    /// Byte offsets into PROEFS.SON, per GAME_SPEC §1.9.
    fn offset(self) -> usize {
        match self {
            SoundEffect::Walk => 0x08,
            SoundEffect::Jump => 0x12,
            SoundEffect::MenuSelect => 0x21,
            SoundEffect::PlaceBomb => 0x24,
            SoundEffect::Pickup => 0x27,
            SoundEffect::Explosion => 0x35,
            SoundEffect::Hurt => 0x78,
            // No documented offset; reuse a close cousin so something plays.
            SoundEffect::Die => 0x78,
            SoundEffect::LevelComplete => 0x21,
        }
    }
}

pub struct SoundManager {
    pub enabled: bool,
    sounds: [Option<Sound>; 9],
}

impl SoundManager {
    pub fn new() -> Self {
        SoundManager { enabled: true, sounds: Default::default() }
    }

    /// Parse PROEFS.SON and pre-bake one WAV per SoundEffect. Falls back to a
    /// disabled manager if the file is missing or unreadable.
    pub async fn load(path: &str) -> Self {
        let mut mgr = SoundManager::new();
        let bytes = match std::fs::read(path) { Ok(b) => b, Err(_) => { mgr.enabled = false; return mgr; } };
        let effects = [
            SoundEffect::Walk, SoundEffect::Jump, SoundEffect::PlaceBomb,
            SoundEffect::Explosion, SoundEffect::Pickup, SoundEffect::Hurt,
            SoundEffect::Die, SoundEffect::LevelComplete, SoundEffect::MenuSelect,
        ];
        for fx in effects {
            let entries = read_entries(&bytes, fx.offset());
            if entries.is_empty() { continue; }
            let pcm = synth_pcm(&entries);
            if pcm.is_empty() { continue; }
            let wav = wrap_wav(&pcm);
            if let Ok(s) = load_sound_from_bytes(&wav).await {
                mgr.sounds[fx as usize] = Some(s);
            }
        }
        mgr
    }

    pub fn play(&self, effect: SoundEffect) {
        if !self.enabled { return; }
        if let Some(s) = self.sounds.get(effect as usize).and_then(|s| s.as_ref()) {
            play_sound_once(s);
        }
    }
}

/// Read consecutive 6-byte entries from `data[off..]` until duration is 0
/// or we hit a sentinel. Cap at 16 entries — the longest effects in the
/// original file are well under that.
fn read_entries(data: &[u8], off: usize) -> Vec<(u16, u16)> {
    let mut out = Vec::new();
    let mut p = off;
    while p + 6 <= data.len() && out.len() < 16 {
        let freq_div = u16::from_le_bytes([data[p], data[p+1]]);
        let dur = u16::from_le_bytes([data[p+2], data[p+3]]);
        if dur == 0 { break; }
        out.push((freq_div, dur));
        p += 6;
    }
    out
}

/// Concatenate square-wave segments into a single 16-bit-mono PCM buffer.
fn synth_pcm(entries: &[(u16, u16)]) -> Vec<i16> {
    let mut total = 0.0_f32;
    let mut pcm: Vec<i16> = Vec::new();
    let mut phase = 0.0_f32;
    for &(freq_div, dur) in entries {
        if total >= MAX_SOUND_SECS { break; }
        let secs = (dur as f32 * SECONDS_PER_TICK).min(MAX_SOUND_SECS - total);
        total += secs;
        let n = (secs * SAMPLE_RATE as f32) as usize;
        if freq_div == 0 {
            pcm.extend(std::iter::repeat(0).take(n));
            continue;
        }
        let freq = (PIT_HZ / freq_div as f32).clamp(40.0, 10_000.0);
        let step = freq / SAMPLE_RATE as f32;
        for _ in 0..n {
            let s = if (phase % 1.0) < 0.5 { 12_000 } else { -12_000 };
            pcm.push(s);
            phase += step;
        }
    }
    pcm
}

/// Build a minimal RIFF/WAVE container around 16-bit-mono PCM samples.
fn wrap_wav(pcm: &[i16]) -> Vec<u8> {
    let data_bytes = (pcm.len() * 2) as u32;
    let byte_rate = SAMPLE_RATE * 2;
    let mut out = Vec::with_capacity(44 + pcm.len() * 2);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36u32 + data_bytes).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());      // fmt chunk size
    out.extend_from_slice(&1u16.to_le_bytes());       // PCM
    out.extend_from_slice(&1u16.to_le_bytes());       // mono
    out.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());       // block align
    out.extend_from_slice(&16u16.to_le_bytes());      // bits per sample
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_bytes.to_le_bytes());
    for s in pcm { out.extend_from_slice(&s.to_le_bytes()); }
    out
}
