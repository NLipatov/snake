use snake::game::Game;
use snake::grid::{Grid, Point};
use snake::grid_geometry::GridGeometry;
use snake::renderer::Renderer;
use snake::snake::Snake;
use snake::terminal::Terminal;

fn main() {
    let geometry = GridGeometry::new(32, 32);
    let grid = Grid::new(geometry.clone());
    let snake = match Snake::new(Point::new(5, 5), geometry.clone()) {
        Ok(snake) => snake,
        Err(err) => panic!("{}", err),
    };
    let mut game = Game::new(Terminal::default(), grid, snake, Renderer::new(), 4);
    game.start();
    println!("Game over!");
    println!("You've scored: {}", game.score());
}
