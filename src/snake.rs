use raylib::prelude::*;
use KeyboardKey::*;

pub struct Snake {
    pub segments: Vec<Segment>,
    direction: Direction,
    steer_direction: Direction,
    timer: f32,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
    Paused,
}

#[derive(Clone, Default)]
pub struct Segment {
    pub pos_x: i8,
    pub pos_y: i8,
}

impl Snake {
    pub fn new() -> Snake {
        Snake { segments: vec![Segment { pos_x: 6, pos_y: 6 }], direction: Direction::Right, steer_direction: Direction::Paused, timer: 0f32 }
    }

    pub fn grow(&mut self) {
        let last = self.segments[self.segments.len() - 1].clone();
        self.segments.push(Segment { pos_x: last.pos_x, pos_y: last.pos_y });
    }

    pub fn handle_input(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KEY_RIGHT) && self.direction != Direction::Left {
            self.steer_direction = Direction::Right;
        }

        if rl.is_key_pressed(KEY_DOWN) && self.direction != Direction::Up {
            self.steer_direction = Direction::Down;
        }

        if rl.is_key_pressed(KEY_LEFT) && self.direction != Direction::Right {
            self.steer_direction = Direction::Left;
        }

        if rl.is_key_pressed(KEY_UP) && self.direction != Direction::Down {
            self.steer_direction = Direction::Up;
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, current_state: &mut usize) {
        self.timer += rl.get_frame_time();


        if self.timer >= 0.15f32 {
            self.timer -= 0.15f32;

            if self.segments.len() > 1 {
                for i in (1..self.segments.len()).rev() {
                    self.segments[i] = self.segments[i - 1].clone();
                }
            }

            self.direction = self.steer_direction;

            match self.direction {
                Direction::Right => self.segments[0].pos_x += 1,
                Direction::Down => self.segments[0].pos_y += 1,
                Direction::Left => self.segments[0].pos_x -= 1,
                Direction::Up => self.segments[0].pos_y -= 1,
                Direction::Paused => { },
            }

            self.segments[0].pos_x = (self.segments[0].pos_x + 12) % 12;
            self.segments[0].pos_y = (self.segments[0].pos_y + 12) % 12;

            for tail in self.segments.iter().skip(1) {
                if self.segments[0].pos_x == tail.pos_x && self.segments[0].pos_y == tail.pos_y {
                    *current_state = 2;
                }
            }

            if *current_state == 2 {
                *self = Snake::new();
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for segment in &self.segments {
            d.draw_rectangle(segment.pos_x as i32 * 64, segment.pos_y as i32 * 64, 64, 64, Color::GREEN);
        }
    }
}
