use snake::game::Game;
use snake::grid::{Grid, Point};
use snake::renderer::Renderer;
use snake::snake::Snake;

fn main() {
    let grid = Grid::new(32, 32);
    let snake = Snake::new(Point::new(5, 5));
    let mut game = Game::new(grid, snake, Renderer::new(), 2);
    game.start();
    println!("Game over!");
    println!("You've scored: {}", game.score());
}
