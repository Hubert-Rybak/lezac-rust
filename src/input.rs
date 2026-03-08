use macroquad::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct PlayerInput {
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub jump: bool,
    pub fire: bool,
}

impl PlayerInput {
    pub fn read_player1() -> Self {
        PlayerInput {
            left: is_key_down(KeyCode::Z),
            right: is_key_down(KeyCode::X),
            down: is_key_down(KeyCode::C),
            jump: is_key_down(KeyCode::M),
            fire: is_key_pressed(KeyCode::N),
        }
    }
    pub fn read_player2() -> Self {
        PlayerInput {
            left: is_key_down(KeyCode::Left),
            right: is_key_down(KeyCode::Right),
            down: is_key_down(KeyCode::Down),
            jump: is_key_down(KeyCode::Up),
            fire: is_key_pressed(KeyCode::Key0) || is_key_pressed(KeyCode::Kp0),
        }
    }
    pub fn change_weapon(&self) -> bool { self.left && self.right }
}
