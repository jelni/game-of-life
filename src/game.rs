use std::collections::VecDeque;
use std::mem;

use macroquad::prelude::{KeyCode, MouseButton, GRAY, WHITE};
use macroquad::{input, shapes, text, time, window};

use crate::board::Board;
use crate::quad_tree::Point;

const GLIDER_GUN: [&[i16]; 9] = [
    &[24],
    &[22, 24],
    &[12, 13, 20, 21, 34, 35],
    &[11, 15, 20, 21, 34, 35],
    &[0, 1, 10, 16, 20, 21],
    &[0, 1, 10, 14, 16, 17, 22, 24],
    &[10, 16, 24],
    &[11, 15],
    &[12, 13],
];

pub struct Game {
    paused: bool,
    offset: (f32, f32),
    scale: f32,
    previous_mouse_position: (f32, f32),
    board: Board,
    last_states: VecDeque<Vec<Point>>,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::new();

        for (row, y) in GLIDER_GUN.into_iter().zip(0..) {
            for &x in row {
                let position = Point {
                    x: x + 16,
                    y: y + 16,
                };
                board.set_cell(position, true);
            }
        }

        Self {
            paused: true,
            offset: (0., 0.),
            scale: 4.,
            previous_mouse_position: (0., 0.),
            board,
            last_states: VecDeque::new(),
        }
    }

    pub async fn game_loop(mut self) {
        loop {
            self.simulate();
            self.handle_input();
            self.draw();
            window::next_frame().await;
        }
    }

    fn simulate(&mut self) {
        if !self.paused {
            self.board = self.board.next_state();
        }
    }

    fn handle_input(&mut self) {
        if input::is_key_pressed(KeyCode::Space) {
            if self.paused {
                self.add_board_history(self.board.to_vec());
            }

            self.paused = !self.paused;
        }

        if self.paused && input::is_key_pressed(KeyCode::N) {
            let mut board = self.board.next_state();
            mem::swap(&mut board, &mut self.board);
            self.add_board_history(board.to_vec());
        }

        if input::is_key_pressed(KeyCode::Z) && input::is_key_down(KeyCode::LeftControl)
            || input::is_key_down(KeyCode::RightControl)
        {
            if let Some(last_state) = self.last_states.pop_front() {
                self.board = last_state.into();
                self.paused = true;
            }
        }

        let scroll = input::mouse_wheel().1;
        if scroll != 0. {
            let mouse_position = input::mouse_position();
            if scroll.is_sign_positive() {
                if self.scale < 16. {
                    self.scale *= 2.;
                    self.offset.0 += mouse_position.0 / self.scale;
                    self.offset.1 += mouse_position.1 / self.scale;
                }
            } else if self.scale > 0.125 {
                self.offset.0 -= mouse_position.0 / self.scale;
                self.offset.1 -= mouse_position.1 / self.scale;
                self.scale /= 2.;
            };
        }

        let mouse_position = input::mouse_position();
        let left_button = input::is_mouse_button_down(MouseButton::Left);
        let right_button = input::is_mouse_button_down(MouseButton::Right);
        let middle_button = input::is_mouse_button_down(MouseButton::Middle);

        if left_button || right_button {
            #[allow(clippy::cast_possible_truncation)]
            let position = Point {
                x: (mouse_position.0 / self.scale + self.offset.0) as i16,
                y: (mouse_position.1 / self.scale + self.offset.1) as i16,
            };
            self.board.set_cell(position, left_button);
        }

        if middle_button {
            self.offset.0 += (self.previous_mouse_position.0 - mouse_position.0) / self.scale;
            self.offset.1 += (self.previous_mouse_position.1 - mouse_position.1) / self.scale;
        }

        self.previous_mouse_position = mouse_position;

        self.offset.0 = (self.offset.0 * self.scale).round() / self.scale;
        self.offset.1 = (self.offset.1 * self.scale).round() / self.scale;
    }

    fn draw(&self) {
        let population = self
            .board
            .cells()
            .map(|cell| {
                shapes::draw_rectangle(
                    (f32::from(cell.x) - self.offset.0) * self.scale,
                    (f32::from(cell.y) - self.offset.1) * self.scale,
                    self.scale,
                    self.scale,
                    WHITE,
                );
            })
            .count();

        text::draw_text(&format!("FPS: {}", time::get_fps()), 0., 12., 16., GRAY);
        text::draw_text(&format!("Time: {}", self.board.time()), 0., 24., 16., GRAY);
        text::draw_text(&format!("Population: {population}"), 0., 36., 16., GRAY);
        text::draw_text(&format!("Scale: {}", self.scale), 0., 48., 16., GRAY);
        text::draw_text(&format!("Position: {:?}", self.offset), 0., 60., 16., GRAY);
        if self.paused {
            text::draw_text("[Paused]", 0., 72., 16., GRAY);
        }
    }

    fn add_board_history(&mut self, board: Vec<Point>) {
        self.last_states.push_front(board);
        self.last_states.truncate(64);
    }
}
