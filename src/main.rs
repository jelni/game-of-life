#![warn(clippy::pedantic)]

use game_of_life::Board;
use macroquad::prelude::{BLACK, GRAY, WHITE};
use macroquad::{shapes, text, time, window};

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

#[macroquad::main("game of life")]
async fn main() {
    window::request_new_screen_size(512., 512.);
    window::next_frame().await;

    let mut board = Board::new((64, 64));

    for (y, row) in GLIDER_GUN.into_iter().enumerate() {
        for &x in row {
            board.set_cell((x, y), true);
        }
    }

    loop {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        {
            board = board.next_state((
                (window::screen_width() as usize / 4).next_power_of_two(),
                (window::screen_height() as usize / 4).next_power_of_two(),
            ));
        }
        window::clear_background(BLACK);

        for (x, y) in board.cells() {
            #[allow(clippy::cast_precision_loss)]
            shapes::draw_rectangle((4 * x) as f32, (4 * y) as f32, 4., 4., WHITE);
        }

        text::draw_text(&time::get_fps().to_string(), 0., 8., 16., GRAY);

        window::next_frame().await;
    }
}
