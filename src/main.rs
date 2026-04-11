use snake::cli::Cli;
use snake::game::Game;
use snake::grid::{Grid, Point};
use snake::grid_geometry::GridGeometry;
use snake::renderer::Renderer;
use snake::snake::Snake;
use snake::terminal::Terminal;

fn main() {
    let geometry = GridGeometry::new(32, 32);
    let grid = Grid::new(geometry);
    let snake = match Snake::new(Point::new(5, 5), geometry) {
        Ok(snake) => snake,
        Err(err) => panic!("{}", err),
    };
    let game = Game::new(grid, snake, 4);
    let mut cli = Cli::new(game, Terminal::default(), Renderer::default());
    let score = cli.run_loop();
    println!("Game over!");
    println!("You've scored: {}", score);
}
