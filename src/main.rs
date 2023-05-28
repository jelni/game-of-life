#![warn(clippy::pedantic)]

use std::cmp::Ordering;

use macroquad::prelude::{BLACK, GRAY, WHITE};
use macroquad::{shapes, text, time, window};
use quad_tree::QuadTree;
use rand::Rng;

mod quad_tree;

const SIZE: usize = 256;

const DIRECTIONS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[macroquad::main("game of life")]
async fn main() {
    window::request_new_screen_size(512., 512.);
    window::next_frame().await;

    let mut rng = rand::thread_rng();
    let mut tree = QuadTree::new(SIZE, (0, 0), (0, 0), false);
    for _ in 0..1024 * 32 {
        tree.insert((rng.gen_range(0..SIZE), rng.gen_range(0..SIZE)), true);
    }

    loop {
        tree = next_state(&tree);

        window::clear_background(BLACK);

        let screen_width = window::screen_width();
        let screen_height = window::screen_height();

        #[allow(clippy::float_cmp)]
        if screen_width != screen_height {
            let size = screen_width.min(1024.);
            window::request_new_screen_size(size, size);
            window::next_frame().await;
        }

        for ((x, y), value) in tree.all_nodes() {
            if !value {
                continue;
            }

            shapes::draw_rectangle(
                screen_width / SIZE as f32 * x as f32,
                screen_height / SIZE as f32 * y as f32,
                screen_width / SIZE as f32,
                screen_height / SIZE as f32,
                WHITE,
            );
        }

        // for (y, row) in grid.into_iter().enumerate() {
        //     for (x, cell) in row.into_iter().enumerate() {
        //         if !cell {
        //             continue;
        //         }

        //         shapes::draw_rectangle(
        //             screen_width / row.len() as f32 * x as f32,
        //             screen_height / grid.len() as f32 * y as f32,
        //             screen_width / row.len() as f32,
        //             screen_height / grid.len() as f32,
        //             WHITE,
        //         );
        //     }
        // }

        text::draw_text(&time::get_fps().to_string(), 0., 8., 16., GRAY);

        window::next_frame().await;
    }
}

fn next_state(current: &QuadTree<bool>) -> QuadTree<bool> {
    let mut counts = QuadTree::new(SIZE, (0, 0), (0, 0), 0);

    for ((x, y), value) in current.all_nodes() {
        if !value {
            continue;
        }

        for dir in DIRECTIONS {
            if dir.0 == -1 && x == 0
                || dir.1 == -1 && y == 0
                || dir.0 == 1 && x == SIZE - 1
                || dir.1 == 1 && y == SIZE - 1
            {
                continue;
            }

            let position = (
                usize::try_from(i16::try_from(x).unwrap() + dir.0).unwrap(),
                usize::try_from(i16::try_from(y).unwrap() + dir.1).unwrap(),
            );

            let count = counts.get(position).unwrap_or(0);
            counts.insert(position, count + 1);
        }

        // for (y, row) in current.iter().enumerate() {
        //     for (x, cell) in row.iter().enumerate() {
        //         if !cell {
        //             continue;
        //         }

        //         for dir in DIRECTIONS {
        //             if dir.0 == -1 && x == 0
        //                 || dir.1 == -1 && y == 0
        //                 || dir.0 == 1 && x == row.len() - 1
        //                 || dir.1 == 1 && y == current.len() - 1
        //             {
        //                 continue;
        //             }

        //             counts[usize::try_from(i16::try_from(y).unwrap() +
        // dir.1).unwrap()]
        // [usize::try_from(i16::try_from(x).unwrap() + dir.0).unwrap()] += 1;
        //         }
        //     }
    }

    // dbg!(counts);

    let mut new_tree = QuadTree::new(SIZE, (0, 0), (0, 0), false);

    for ((x, y), count) in counts.all_nodes() {
        if count == 0 {
            continue;
        }

        let value = match count.cmp(&2) {
            Ordering::Less => false,
            Ordering::Equal => current.get((x, y)).unwrap_or(false),
            Ordering::Greater => count == 3,
        };

        new_tree.insert((x, y), value);
    }

    new_tree

    // counts
    //     .into_iter()
    //     .enumerate()
    //     .map(|(y, row)| {
    //         row.into_iter()
    //             .enumerate()
    //             .map(|(x, count)| match count.cmp(&2) {
    //                 Ordering::Less => false,
    //                 Ordering::Equal => current[y][x],
    //                 Ordering::Greater => count == 3,
    //             })
    //             .collect::<Vec<_>>()
    //             .try_into()
    //             .unwrap()
    //     })
    //     .collect::<Vec<_>>()
    //     .try_into()
    //     .unwrap()
}
