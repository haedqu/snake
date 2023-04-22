use raylib::prelude::*;

use crate::Snake;

pub struct Food {
    pos_x: i8,
    pos_y: i8,
}

impl Food {
    pub fn new(snake: &Snake) -> Food {
        let mut food = Food { pos_x: 0, pos_y: 0 };

        food.randomize(snake);

        food
    }

    pub fn update(&mut self, snake: &mut Snake) {
        if self.pos_x == snake.segments[0].pos_x && self.pos_y == snake.segments[0].pos_y {
            snake.grow();
            self.randomize(snake);
        }
    }

    pub fn randomize(&mut self, snake: &Snake) {
        use rand::Rng;

        let mut possible_positions = Vec::<(i8, i8)>::new();

        for y in 0..12 {
            for x in 0..12 {
                possible_positions.push((x, y));
            }
        }

        for segment in &snake.segments {
            for (i, position) in possible_positions.iter().enumerate() {
                if position.0 == segment.pos_x && position.1 == segment.pos_y {
                    possible_positions.remove(i);

                    break;
                }
            }
        }

        let (x, y) = possible_positions[rand::thread_rng().gen_range(0..possible_positions.len())];

        self.pos_x = x;
        self.pos_y = y;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.pos_x as i32 * 64, self.pos_y as i32 * 64, 64, 64, Color::RED);
    }
}
