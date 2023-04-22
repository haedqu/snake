use crate::Snake;
use crate::Food;

use crate::game_state::*;

pub struct MainState {
    snake: Snake,
    food: Food,
}

impl MainState {
    pub fn new() -> MainState {
        let snake = Snake::new();

        MainState { snake: Snake::new(), food: Food::new(&snake) }
    }
}

impl GameState for MainState {
    fn update(&mut self, rl: &RaylibHandle, current_state: &mut usize, _should_quit: &mut bool) {
        self.snake.handle_input(rl);

        self.food.update(&mut self.snake);
        self.snake.update(rl, current_state);
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.snake.draw(d);
        self.food.draw(d);
    }
}
