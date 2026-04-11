use snake::presentation::cli::RunResult;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use snake::domain::game::Game;
    use snake::domain::grid::{Grid, Point};
    use snake::domain::grid_geometry::GridGeometry;
    use snake::domain::snake::Snake;
    use snake::infrastructure::terminal::Terminal;
    use snake::presentation::cli::Cli;
    use snake::presentation::renderer::Renderer;

    let geometry = GridGeometry::new(32, 32);
    let grid = Grid::new(geometry);
    let snake = match Snake::new(Point::new(5, 5), geometry) {
        Ok(snake) => snake,
        Err(err) => panic!("{}", err),
    };
    let game = Game::new(grid, snake, 4);
    let mut cli = Cli::new(game, Terminal::default(), Renderer::default());
    match cli.run_loop() {
        RunResult::GameOver { score } => println!("Game Over! Score: {}", score),
        RunResult::Quit { score } => println!("Quit! Score: {}", score),
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {}
