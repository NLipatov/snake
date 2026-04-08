use crate::game::Command::{
    Escape, Pause, SnakeMoveDown, SnakeMoveLeft, SnakeMoveRight, SnakeMoveUp,
};
use crate::grid::GridCell::{Empty, Food, Wall};
use crate::grid::{Grid, Point};
use crate::raw_mode_guard::RawModeGuard;
use crate::renderer::{RenderState, Renderer};
use crate::snake::{Direction, Snake};
use crate::terminal::Terminal;
use rand::{RngExt, rngs};
use std::time::Duration;

pub enum Command {
    Escape,
    Pause,
    SnakeMoveUp,
    SnakeMoveDown,
    SnakeMoveLeft,
    SnakeMoveRight,
}

pub struct Game {
    terminal: Terminal,
    grid: Grid,
    snake: Snake,
    renderer: Renderer,
    rng: rngs::ThreadRng,
    food_spawn_probability: i32,
}

impl Game {
    pub fn new(
        terminal: Terminal,
        grid: Grid,
        snake: Snake,
        renderer: Renderer,
        food_spawn_probability: i32,
    ) -> Game {
        let rng = rand::rng();
        Game {
            terminal,
            grid,
            snake,
            renderer,
            rng,
            food_spawn_probability,
        }
    }
    pub fn start(&mut self) {
        let _rmg = RawModeGuard::new();
        self.run_loop();
    }
    fn run_loop(&mut self) {
        loop {
            match self.terminal.wait_for_command_async() {
                None => {}
                Some(command) => match command {
                    Escape => break,
                    Pause => loop {
                        match self.terminal.wait_for_command_sync() {
                            Some(Pause) => break,
                            Some(Escape) => return,
                            _ => (),
                        }
                    },
                    SnakeMoveUp => self.snake.set_direction(Direction::Up),
                    SnakeMoveDown => self.snake.set_direction(Direction::Down),
                    SnakeMoveLeft => self.snake.set_direction(Direction::Left),
                    SnakeMoveRight => self.snake.set_direction(Direction::Right),
                },
            }
            self.snake.move_snake();
            if self.snake.has_self_collision() {
                break;
            }
            let head = self.snake.head();
            if !self.grid.within_bounds(&head) {
                break;
            }
            if *self.grid.cell(&head) == Wall {
                break;
            }
            if *self.grid.cell(&head) == Food {
                self.snake.grow();
                self.grid.on_food_consumed(&head)
            }
            if self.should_spawn_food() {
                self.spawn_food();
            }
            let score = self.snake.len().saturating_sub(1);
            let render_state = RenderState::new(&self.grid, &self.snake, score);
            self.renderer.render(render_state);
            std::thread::sleep(Duration::from_millis(115));
        }
    }
    fn should_spawn_food(&mut self) -> bool {
        self.rng.random_range(0..100) < self.food_spawn_probability
    }
    fn spawn_food(&mut self) {
        let max_x = self.grid.width();
        let max_y = self.grid.height();
        let point = Point::new(
            self.rng.random_range(0..max_x),
            self.rng.random_range(0..max_y),
        );
        self.spawn_food_at(&point);
    }
    fn spawn_food_at(&mut self, point: &Point) {
        if *self.grid.cell(point) == Empty && !self.snake.occupies(point) {
            self.grid.change_cell(point, Food);
        }
    }
    pub fn score(&self) -> usize {
        self.snake.len().saturating_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use crate::grid::{Grid, GridCell, Point};
    use crate::renderer::{RenderState, Renderer};
    use crate::snake::Snake;
    use crate::terminal::Terminal;

    fn point(x: i32, y: i32) -> Point {
        Point::new(x, y)
    }

    fn game_with_probability(food_spawn_probability: i32) -> Game {
        Game::new(
            Terminal::default(),
            Grid::new(8, 8),
            Snake::new(Point::new(3, 3)),
            Renderer::new(),
            food_spawn_probability,
        )
    }

    #[test]
    fn should_spawn_food_is_never_true_at_zero_percent() {
        let mut game = game_with_probability(0);

        for _ in 0..100 {
            assert!(!game.should_spawn_food());
        }
    }

    #[test]
    fn should_spawn_food_is_always_true_at_hundred_percent() {
        let mut game = game_with_probability(100);

        for _ in 0..100 {
            assert!(game.should_spawn_food());
        }
    }

    #[test]
    fn spawn_food_at_places_food_on_empty_unoccupied_cell() {
        let mut game = game_with_probability(0);

        game.spawn_food_at(&point(4, 4));

        assert_eq!(game.grid.cell(&point(4, 4)), &GridCell::Food);
    }

    #[test]
    fn spawn_food_at_does_not_place_food_on_snake() {
        let mut game = game_with_probability(0);

        game.spawn_food_at(&point(3, 3));

        assert_eq!(game.grid.cell(&point(3, 3)), &GridCell::Empty);
    }

    #[test]
    fn spawn_food_at_does_not_overwrite_existing_food_or_wall() {
        let mut game = game_with_probability(0);
        game.grid.change_cell(&point(4, 4), GridCell::Food);

        game.spawn_food_at(&point(4, 4));
        game.spawn_food_at(&point(0, 0));

        assert_eq!(game.grid.cell(&point(4, 4)), &GridCell::Food);
        assert_eq!(game.grid.cell(&point(0, 0)), &GridCell::Wall);
    }

    #[test]
    fn score_is_zero_for_new_snake() {
        let game = game_with_probability(0);

        assert_eq!(game.score(), 0);
    }

    #[test]
    fn score_counts_growth_segments() {
        let mut game = game_with_probability(0);
        game.snake.grow();
        game.snake.grow();

        assert_eq!(game.score(), 2);
    }

    #[test]
    fn render_state_uses_current_grid_snake_and_score() {
        let mut game = game_with_probability(0);
        game.snake.grow();
        let render_state = RenderState::new(&game.grid, &game.snake, game.score());

        assert_eq!(render_state.grid().width(), 8);
        assert!(render_state.snake().occupies(&point(3, 3)));
        assert_eq!(render_state.score(), 1);
    }
}
