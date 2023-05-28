#![warn(clippy::pedantic)]

use game_of_life::Board;
use macroquad::prelude::{KeyCode, MouseButton, BLACK, GRAY, WHITE};
use macroquad::window::Conf;
use macroquad::{input, shapes, text, time, window, Window};

mod game_of_life;
mod quad_tree;

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

fn main() {
    Window::from_config(
        Conf {
            window_title: "game of life".into(),
            window_width: 512,
            window_height: 512,
            ..Default::default()
        },
        run(),
    );
}

async fn run() {
    let mut board = Board::new((64, 64));

    for (y, row) in GLIDER_GUN.into_iter().enumerate() {
        for &x in row {
            board.set_cell((x, y), true);
        }
    }

    let mut paused = false;
    let mut scale = 4;

    loop {
        if !paused {
            board = board.next_state(
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                (
                    (window::screen_width() as usize / scale).next_power_of_two(),
                    (window::screen_height() as usize / scale).next_power_of_two(),
                ),
            );
        }

        if input::is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        let scroll = input::mouse_wheel().1;
        if scroll != 0. {
            scale = if scroll.is_sign_positive() {
                (scale * 2).min(16)
            } else {
                (scale / 2).max(1)
            };
        }

        let left_button = input::is_mouse_button_down(MouseButton::Left);
        let right_button = input::is_mouse_button_down(MouseButton::Right);

        if left_button || right_button {
            let position = input::mouse_position();
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let position = (position.0 as usize / scale, position.1 as usize / scale);
            board.set_cell(position, left_button);
        }

        window::clear_background(BLACK);

        for (x, y) in board.cells() {
            #[allow(clippy::cast_precision_loss)]
            shapes::draw_rectangle(
                (scale * x) as f32,
                (scale * y) as f32,
                scale as f32,
                scale as f32,
                WHITE,
            );
        }

        text::draw_text(&time::get_fps().to_string(), 0., 8., 16., GRAY);

        window::next_frame().await;
    }
}
