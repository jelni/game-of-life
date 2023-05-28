use std::collections::VecDeque;

use macroquad::prelude::{KeyCode, MouseButton, GRAY, WHITE};
use macroquad::{input, shapes, text, time, window};

use crate::board::Board;

const GLIDER_GUN: [&[usize]; 9] = [
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
    scale: usize,
    board: Board,
    last_states: VecDeque<Board>,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::new((64, 64), 0);

        for (y, row) in GLIDER_GUN.into_iter().enumerate() {
            for &x in row {
                board.set_cell((x + 16, y + 16), true);
            }
        }

        Self {
            paused: true,
            scale: 4,
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
            self.board = self.board.next_state(
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                (
                    (window::screen_width() as usize / self.scale).next_power_of_two(),
                    (window::screen_height() as usize / self.scale).next_power_of_two(),
                ),
            );
        }
    }

    fn handle_input(&mut self) {
        if input::is_key_pressed(KeyCode::Space) {
            if self.paused {
                self.last_states.push_front(self.board.clone());
                self.last_states.truncate(64);
            }

            self.paused = !self.paused;
        }

        if input::is_key_pressed(KeyCode::Z) && input::is_key_down(KeyCode::LeftControl)
            || input::is_key_down(KeyCode::RightControl)
        {
            if let Some(last_state) = self.last_states.pop_front() {
                self.board = last_state;
                self.paused = true;
            }
        }

        let scroll = input::mouse_wheel().1;
        if scroll != 0. {
            self.scale = if scroll.is_sign_positive() {
                (self.scale * 2).min(16)
            } else {
                (self.scale / 2).max(1)
            };
        }

        let left_button = input::is_mouse_button_down(MouseButton::Left);
        let right_button = input::is_mouse_button_down(MouseButton::Right);

        if left_button || right_button {
            let position = input::mouse_position();
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let position = (
                position.0 as usize / self.scale,
                position.1 as usize / self.scale,
            );
            self.board.set_cell(position, left_button);
        }
    }

    fn draw(&self) {
        let population = self
            .board
            .cells()
            .map(|(x, y)| {
                #[allow(clippy::cast_precision_loss)]
                shapes::draw_rectangle(
                    (self.scale * x) as f32,
                    (self.scale * y) as f32,
                    self.scale as f32,
                    self.scale as f32,
                    WHITE,
                );
            })
            .count();

        let board_size = self.board.size();
        text::draw_text(&format!("FPS: {}", time::get_fps()), 0., 12., 16., GRAY);
        text::draw_text(&format!("Time: {}", self.board.time()), 0., 24., 16., GRAY);
        text::draw_text(&format!("Population: {population}"), 0., 36., 16., GRAY);
        text::draw_text(&format!("Board size: {board_size:?}"), 0., 48., 16., GRAY);
    }
}
