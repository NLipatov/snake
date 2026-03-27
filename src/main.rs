use crate::engine::Game;
use crate::grid::Grid;
use crate::renderer::Renderer;

mod engine;
mod grid;
mod renderer;
mod snake;

fn main() {
    let grid = Grid::new(32, 32);
    let snake = snake::Snake::new((5, 5));
    let mut game = Game::new(grid, snake, Renderer::new(), 2);
    game.start();
    println!("Game over!");
}
