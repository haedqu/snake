use raylib::prelude::*;

mod snake;
mod food;
mod game_state;
mod start_state;
mod main_state;
mod end_state;

use snake::Snake;
use food::Food;
use game_state::GameState;
use start_state::StartState;
use main_state::MainState;
use end_state::EndState;

fn main() {
    let (mut rl, thread) = raylib::init().size(64 * 12, 64 * 12).title("Snake").vsync().build();

    let mut states: Vec<Box<dyn GameState>> = vec![Box::new(StartState::new()), Box::new(MainState::new()), Box::new(EndState::new())];
    let mut current_state = 0;
    let mut should_quit = false;

    while !should_quit {
        if rl.window_should_close() {
            should_quit = true;
        }

        states[current_state].update(&rl, &mut current_state, &mut should_quit);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        states[current_state].draw(&mut d);
    }
}
