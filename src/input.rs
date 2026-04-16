use macroquad::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct PlayerInput {
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub jump: bool,
    pub fire: bool,
    /// Edge-triggered: both left and right just-pressed in this frame.
    pub weapon_change: bool,
}

impl PlayerInput {
    pub fn read_player1() -> Self {
        let left = is_key_down(KeyCode::Z);
        let right = is_key_down(KeyCode::X);
        PlayerInput {
            left, right,
            down: is_key_down(KeyCode::C),
            jump: is_key_down(KeyCode::M),
            fire: is_key_pressed(KeyCode::N),
            weapon_change: is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::X)
                || is_key_pressed(KeyCode::X) && is_key_down(KeyCode::Z),
        }
    }
    pub fn read_player2() -> Self {
        let left = is_key_down(KeyCode::Left);
        let right = is_key_down(KeyCode::Right);
        PlayerInput {
            left, right,
            down: is_key_down(KeyCode::Down),
            jump: is_key_down(KeyCode::Up),
            fire: is_key_pressed(KeyCode::Key0) || is_key_pressed(KeyCode::Kp0),
            weapon_change: is_key_pressed(KeyCode::Left) && is_key_down(KeyCode::Right)
                || is_key_pressed(KeyCode::Right) && is_key_down(KeyCode::Left),
        }
    }
    pub fn change_weapon(&self) -> bool { self.weapon_change }
}
