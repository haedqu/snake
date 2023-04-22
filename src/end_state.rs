use crate::game_state::*;
use KeyboardKey::*;

pub struct EndState;

impl EndState {
    pub fn new() -> EndState {
        EndState { }
    }
}

impl GameState for EndState {
    fn update(&mut self, rl: &RaylibHandle, current_state: &mut usize, _should_quit: &mut bool) {
        if rl.is_key_pressed(KEY_ENTER) || rl.is_key_pressed(KEY_SPACE) {
            *current_state = 0;
        }
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_text("GAMEOVER", 64 * 12 / 2 - measure_text("GAMEOVER", 80) / 2, 64 * 12 / 2 - 40, 80, Color::WHITE);
    }
}
