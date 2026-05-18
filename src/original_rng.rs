/// Turbo Pascal-style RNG used by the original runtime (`FUN_1920_13f7`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OriginalRng {
    seed: u32,
}

impl OriginalRng {
    const MULTIPLIER: u32 = 0x0808_8405;

    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn from_dos_time_fields(cx: u16, dx: u16) -> Self {
        Self::new(((dx as u32) << 16) | cx as u32)
    }

    pub fn from_system_time() -> Self {
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let seconds_of_day = duration.as_secs() % 86_400;
        let hour = (seconds_of_day / 3_600) as u16;
        let minute = ((seconds_of_day / 60) % 60) as u16;
        let second = (seconds_of_day % 60) as u16;
        let hundredths = (duration.subsec_millis() / 10) as u16;
        let cx = (hour << 8) | minute;
        let dx = (second << 8) | hundredths;
        Self::from_dos_time_fields(cx, dx)
    }

    pub fn seed(self) -> u32 {
        self.seed
    }

    pub fn next_word(&mut self) -> u16 {
        self.seed = self.seed.wrapping_mul(Self::MULTIPLIER).wrapping_add(1);
        (self.seed >> 16) as u16
    }

    pub fn gen_mod(&mut self, modulus: u16) -> u16 {
        if modulus == 0 {
            0
        } else {
            self.next_word() % modulus
        }
    }

    pub fn gen_centered(&mut self, modulus: u16) -> i16 {
        self.gen_mod(modulus) as i16 - (modulus / 2) as i16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decompiled_step(seed: u32) -> u32 {
        let low = seed as u16 as u32;
        let high = (seed >> 16) as u16 as u32;
        let product = low * 0x8405;
        let low_shift = low << 3;
        let low_times_0x0808 =
            ((low_shift & 0xff) | ((((low_shift >> 8) & 0xff) + (low_shift & 0xff)) << 8)) & 0xffff;
        let high_partial = ((product >> 16) + low_times_0x0808 + high * 5) & 0xffff;
        let high_byte_adjust = ((high & 0xff) * 4 + (((high << 7) & 0xff) as u32)) & 0xff;
        let next_low = ((product as u16).wrapping_add(1)) as u32;
        let carry = u32::from((product & 0xffff) == 0xffff);
        let next_high = ((((high_partial & 0xff00) + (high_byte_adjust << 8)) & 0xff00)
            | (high_partial & 0x00ff))
            .wrapping_add(carry)
            & 0xffff;
        (next_high << 16) | next_low
    }

    #[test]
    fn original_rng_step_matches_decompiled_word_math() {
        for seed in [0, 1, 0x1234_5678, 0xffff_ffff, 0x1afe_1b00] {
            let mut rng = OriginalRng::new(seed);
            rng.next_word();
            assert_eq!(rng.seed(), decompiled_step(seed));
        }
    }

    #[test]
    fn original_rng_mod_matches_fun_1920_13a8_contract() {
        let mut rng = OriginalRng::new(0x1234_5678);
        assert_eq!(rng.gen_mod(0), 0);

        let mut rng = OriginalRng::new(0x1234_5678);
        let word = rng.next_word();
        let mut rng = OriginalRng::new(0x1234_5678);
        assert_eq!(rng.gen_mod(10), word % 10);
    }

    #[test]
    fn original_rng_centered_ranges_match_common_call_pattern() {
        let mut expected = OriginalRng::new(0x2468_ace0);
        let expected_value = expected.gen_mod(600) as i16 - 300;

        let mut rng = OriginalRng::new(0x2468_ace0);
        assert_eq!(rng.gen_centered(600), expected_value);
        assert_eq!(rng.seed(), expected.seed());
        assert_eq!(OriginalRng::new(0).gen_centered(0), 0);
    }

    #[test]
    fn original_rng_seed_matches_dos_time_field_layout() {
        let rng = OriginalRng::from_dos_time_fields(0x1234, 0x5678);
        assert_eq!(rng.seed(), 0x5678_1234);
    }

    #[test]
    fn gameplay_modules_do_not_use_host_rng_directly() {
        let game = include_str!("game.rs");
        let monsters = include_str!("monsters.rs");

        assert!(!game.contains("macroquad::rand"));
        assert!(!game.contains("gen_range"));
        assert!(!monsters.contains("macroquad::rand"));
        assert!(!monsters.contains("gen_range"));
    }
}
