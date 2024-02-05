use game::Game;
use macroquad::window::Conf;
use macroquad::Window;

mod board;
mod game;
mod quad_tree;

fn main() {
    Window::from_config(
        Conf {
            window_title: "Conway's Game of Life".into(),
            window_width: 512,
            window_height: 512,
            ..Default::default()
        },
        Game::new().game_loop(),
    );
}
