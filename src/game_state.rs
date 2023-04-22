pub use raylib::prelude::*;

pub trait GameState {
    fn update(&mut self, rl: &RaylibHandle, current_state: &mut usize, should_quit: &mut bool);

    fn draw(&mut self, d: &mut RaylibDrawHandle);
}
