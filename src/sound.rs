//! Sound effects — PROEFS.SON ported to PC-speaker-style square waves.
//!
//! PROEFS.SON is 782 bytes of 6-byte entries (per GAME_SPEC §1.9):
//!   `[freq_div:u16 LE] [duration:u16 LE] [flags:u8] [flags2:u8]`
//!
//! Sound effects are addressed by *byte offset* within the file. We read
//! sequential entries from the named offset, generate a square-wave PCM
//! buffer at `1_193_180 / freq_div` Hz for `duration` ticks, wrap it in a
//! WAV header, and hand the bytes to macroquad's `load_sound_from_bytes`.

use std::cell::Cell;

use macroquad::audio::{load_sound_from_bytes, play_sound_once, stop_sound, Sound};
use macroquad::time::get_time;

const SAMPLE_RATE: u32 = 22050;
/// 8253 PIT base frequency.
const PIT_HZ: f32 = 1_193_180.0;
/// Original game ran at 70 Hz; durations look like "ticks of that timer".
const SECONDS_PER_TICK: f32 = 1.0 / 70.0;
/// Cap any single sound at ~0.6 s so a stuck duration field can't lock us up.
const MAX_SOUND_SECS: f32 = 0.6;

#[derive(Clone, Copy)]
pub enum SoundEffect {
    Jump,
    Teleport,
    State6Impulse,
    PlatformAction,
    PlaceBomb,
    Collapse,
    Explosion,
    Pickup,
    HighScoreConfirm,
    Hurt,
    Die,
    State6Death,
    LevelComplete,
    MenuSelect,
}

const ALL_SOUND_EFFECTS: [SoundEffect; 14] = [
    SoundEffect::Jump,
    SoundEffect::Teleport,
    SoundEffect::State6Impulse,
    SoundEffect::PlatformAction,
    SoundEffect::PlaceBomb,
    SoundEffect::Collapse,
    SoundEffect::Explosion,
    SoundEffect::Pickup,
    SoundEffect::HighScoreConfirm,
    SoundEffect::Hurt,
    SoundEffect::Die,
    SoundEffect::State6Death,
    SoundEffect::LevelComplete,
    SoundEffect::MenuSelect,
];

impl SoundEffect {
    /// Byte offsets into PROEFS.SON, per GAME_SPEC §1.9.
    fn offset(self) -> usize {
        match self {
            SoundEffect::Jump => 0x12,
            SoundEffect::Teleport => 0x1a,
            SoundEffect::State6Impulse => 0x69,
            SoundEffect::PlatformAction => 0x27,
            SoundEffect::MenuSelect => 0x21,
            SoundEffect::PlaceBomb => 0x24,
            SoundEffect::Collapse => 0x27,
            SoundEffect::Pickup => 0x08,
            SoundEffect::HighScoreConfirm => 0x08,
            SoundEffect::Hurt => 0x2d,
            SoundEffect::Explosion => 0x35,
            SoundEffect::LevelComplete => 0x3d,
            SoundEffect::State6Death => 0x3d,
            // FUN_1000_2adc requests 0x78 at priority 0x0b for death/game-over.
            SoundEffect::Die => 0x78,
        }
    }

    fn priority(self) -> u8 {
        match self {
            SoundEffect::Jump => 3,
            SoundEffect::Teleport => 4,
            SoundEffect::State6Impulse => 4,
            SoundEffect::PlatformAction => 6,
            SoundEffect::MenuSelect => 2,
            SoundEffect::PlaceBomb => 2,
            SoundEffect::Collapse => 5,
            SoundEffect::Explosion => 5,
            SoundEffect::Pickup => 5,
            SoundEffect::HighScoreConfirm => 11,
            SoundEffect::Hurt => 4,
            SoundEffect::Die => 11,
            SoundEffect::State6Death => 12,
            SoundEffect::LevelComplete => 10,
        }
    }
}

pub struct SoundManager {
    pub enabled: bool,
    sounds: [Option<Sound>; 14],
    durations: [f64; 14],
    current_sound: Cell<Option<usize>>,
    current_priority: Cell<u8>,
    current_until: Cell<f64>,
}

impl SoundManager {
    pub fn new() -> Self {
        SoundManager {
            enabled: true,
            sounds: Default::default(),
            durations: [0.0; 14],
            current_sound: Cell::new(None),
            current_priority: Cell::new(0),
            current_until: Cell::new(0.0),
        }
    }

    /// Parse PROEFS.SON and pre-bake one WAV per SoundEffect. Falls back to a
    /// disabled manager if the file is missing or unreadable.
    pub async fn load(path: &str) -> Self {
        let mut mgr = SoundManager::new();
        let bytes = match crate::assets::try_load_file(path) {
            Some(b) => b,
            None => {
                mgr.enabled = false;
                return mgr;
            }
        };
        for fx in ALL_SOUND_EFFECTS {
            let entries = read_entries(&bytes, fx.offset());
            if entries.is_empty() {
                continue;
            }
            mgr.durations[fx as usize] = entries_duration(&entries) as f64;
            let pcm = synth_pcm(&entries);
            if pcm.is_empty() {
                continue;
            }
            let wav = wrap_wav(&pcm);
            if let Ok(s) = load_sound_from_bytes(&wav).await {
                mgr.sounds[fx as usize] = Some(s);
            }
        }
        mgr
    }

    pub fn play(&self, effect: SoundEffect) {
        if !self.enabled {
            return;
        }
        let now = get_time();
        let active = self.current_sound.get().is_some() && now < self.current_until.get();
        let requested_priority = effect.priority();
        if !should_preempt(active, self.current_priority.get(), requested_priority) {
            return;
        }
        if let Some(current) = self
            .current_sound
            .get()
            .and_then(|idx| self.sounds[idx].as_ref())
        {
            stop_sound(current);
        }
        let idx = effect as usize;
        if let Some(s) = self.sounds.get(idx).and_then(|s| s.as_ref()) {
            play_sound_once(s);
            self.current_sound.set(Some(idx));
            self.current_priority.set(requested_priority);
            self.current_until.set(now + self.durations[idx]);
        }
    }
}

fn should_preempt(active: bool, current_priority: u8, requested_priority: u8) -> bool {
    !active || current_priority.saturating_sub(1) < requested_priority
}

/// Read consecutive 6-byte entries from `data[off..]` until duration is 0
/// or we hit a sentinel. Cap at 16 entries — the longest effects in the
/// original file are well under that.
fn read_entries(data: &[u8], off: usize) -> Vec<(u16, u16)> {
    let mut out = Vec::new();
    let mut p = off;
    while p + 6 <= data.len() && out.len() < 16 {
        let freq_div = u16::from_le_bytes([data[p], data[p + 1]]);
        let dur = u16::from_le_bytes([data[p + 2], data[p + 3]]);
        if dur == 0 {
            break;
        }
        out.push((freq_div, dur));
        p += 6;
    }
    out
}

fn entries_duration(entries: &[(u16, u16)]) -> f32 {
    entries
        .iter()
        .map(|&(_, dur)| dur as f32 * SECONDS_PER_TICK)
        .sum::<f32>()
        .min(MAX_SOUND_SECS)
}

/// Concatenate square-wave segments into a single 16-bit-mono PCM buffer.
fn synth_pcm(entries: &[(u16, u16)]) -> Vec<i16> {
    let mut total = 0.0_f32;
    let mut pcm: Vec<i16> = Vec::new();
    let mut phase = 0.0_f32;
    for &(freq_div, dur) in entries {
        if total >= MAX_SOUND_SECS {
            break;
        }
        let secs = (dur as f32 * SECONDS_PER_TICK).min(MAX_SOUND_SECS - total);
        total += secs;
        let n = (secs * SAMPLE_RATE as f32) as usize;
        if freq_div == 0 {
            pcm.extend(std::iter::repeat_n(0, n));
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
    out.extend_from_slice(&16u32.to_le_bytes()); // fmt chunk size
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&1u16.to_le_bytes()); // mono
    out.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes()); // block align
    out.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_bytes.to_le_bytes());
    for s in pcm {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sound_effect_offsets_match_original_proefs_ids() {
        assert_eq!(SoundEffect::Jump.offset(), 0x12);
        assert_eq!(SoundEffect::Teleport.offset(), 0x1a);
        assert_eq!(SoundEffect::State6Impulse.offset(), 0x69);
        assert_eq!(SoundEffect::PlatformAction.offset(), 0x27);
        assert_eq!(SoundEffect::MenuSelect.offset(), 0x21);
        assert_eq!(SoundEffect::PlaceBomb.offset(), 0x24);
        assert_eq!(SoundEffect::Collapse.offset(), 0x27);
        assert_eq!(SoundEffect::Pickup.offset(), 0x08);
        assert_eq!(SoundEffect::HighScoreConfirm.offset(), 0x08);
        assert_eq!(SoundEffect::Hurt.offset(), 0x2d);
        assert_eq!(SoundEffect::Explosion.offset(), 0x35);
        assert_eq!(SoundEffect::LevelComplete.offset(), 0x3d);
        assert_eq!(SoundEffect::State6Death.offset(), 0x3d);
        assert_eq!(SoundEffect::Die.offset(), 0x78);
    }

    #[test]
    fn sound_effect_priorities_follow_original_preemption_rule() {
        assert_eq!(SoundEffect::Jump.priority(), 3);
        assert_eq!(SoundEffect::Teleport.priority(), 4);
        assert_eq!(SoundEffect::State6Impulse.priority(), 4);
        assert_eq!(SoundEffect::PlatformAction.priority(), 6);
        assert_eq!(SoundEffect::PlaceBomb.priority(), 2);
        assert_eq!(SoundEffect::Collapse.priority(), 5);
        assert_eq!(SoundEffect::Pickup.priority(), 5);
        assert_eq!(SoundEffect::HighScoreConfirm.priority(), 11);
        assert_eq!(SoundEffect::LevelComplete.priority(), 10);
        assert_eq!(SoundEffect::State6Death.priority(), 12);
        assert_eq!(SoundEffect::Die.priority(), 11);
        assert!(should_preempt(false, 11, 1));
        assert!(should_preempt(true, 5, 5));
        assert!(should_preempt(true, 5, 6));
        assert!(!should_preempt(true, 5, 4));
    }

    #[test]
    fn proefs_son_known_offsets_decode_to_pcm() {
        let bytes = std::fs::read("assets/PROEFS.SON").unwrap();
        assert_eq!(bytes.len(), 782);
        for effect in ALL_SOUND_EFFECTS {
            let entries = read_entries(&bytes, effect.offset());
            assert!(!entries.is_empty());
            assert!(!synth_pcm(&entries).is_empty());
        }
    }
}
