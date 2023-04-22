use crate::game_state::*;
use KeyboardKey::*;

pub struct StartState {
    selected: u8,
}

impl StartState {
    pub fn new() -> StartState {
        StartState { selected: 0 }
    }
}

impl GameState for StartState {
    fn update(&mut self, rl: &RaylibHandle, current_state: &mut usize, should_quit: &mut bool) {
        if rl.is_key_pressed(KEY_DOWN) && self.selected < 1 {
            self.selected += 1;
        }

        if rl.is_key_pressed(KEY_UP) && self.selected > 0 {
            self.selected -= 1;
        }

        if rl.is_key_pressed(KEY_ENTER) || rl.is_key_pressed(KEY_SPACE) {
            match self.selected {
                0 => *current_state = 1,
                1 => *should_quit = true,
                _ => {},
            }
        }
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_text("Start", 64 * 12 / 2 - measure_text("Start", 80) / 2, 64 * 12 / 2 - 100, 80, if self.selected == 0 { Color::GREEN } else { Color::WHITE });
        d.draw_text("Exit", 64 * 12 / 2 - measure_text("Exit", 80) / 2, 64 * 12 / 2, 80, if self.selected == 1 { Color::GREEN } else { Color::WHITE });
    }
}

