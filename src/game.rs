use crate::game::Command::{
    Escape, Pause, SnakeMoveDown, SnakeMoveLeft, SnakeMoveRight, SnakeMoveUp,
};
use crate::grid::Grid;
use crate::grid::GridCell::{Empty, Food, Wall};
use crate::raw_mode_guard::RawModeGuard;
use crate::renderer::Renderer;
use crate::snake::{Direction, Snake};
use crossterm::event;
use crossterm::event::KeyEventKind::Press;
use crossterm::event::{Event, KeyCode};
use rand::{RngExt, rngs};
use std::time::Duration;

enum Command {
    Escape,
    Pause,
    SnakeMoveUp,
    SnakeMoveDown,
    SnakeMoveLeft,
    SnakeMoveRight,
}

pub struct Game {
    grid: Grid,
    snake: Snake,
    renderer: Renderer,
    rng: rngs::ThreadRng,
    food_spawn_probability: i32,
}

impl Game {
    pub fn new(grid: Grid, snake: Snake, renderer: Renderer, food_spawn_probability: i32) -> Game {
        let rng = rand::rng();
        Game {
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
            match self.read_key_async() {
                None => {}
                Some(key_code) => match self.key_to_command(key_code) {
                    None => {}
                    Some(command) => match command {
                        Escape => break,
                        Pause => match self.pause_loop() {
                            Some(Escape) => break,
                            _ => (),
                        },
                        SnakeMoveUp => self.snake.set_direction(Direction::Up),
                        SnakeMoveDown => self.snake.set_direction(Direction::Down),
                        SnakeMoveLeft => self.snake.set_direction(Direction::Left),
                        SnakeMoveRight => self.snake.set_direction(Direction::Right),
                    },
                },
            }
            self.snake.move_snake();
            if self.snake.has_self_collision() {
                break;
            }
            let (x, y) = self.snake.head();
            if x < 0 || y < 0 || x >= self.grid.width() || y >= self.grid.height() {
                break;
            }
            if *self.grid.cell(x, y) == Wall {
                break;
            }
            if *self.grid.cell(x, y) == Food {
                self.snake.grow();
                self.grid.on_food_consumed(x, y)
            }
            if self.should_spawn_food() {
                self.spawn_food();
            }
            self.renderer.render(&self.grid, &self.snake);
            std::thread::sleep(Duration::from_millis(150));
        }
    }
    fn pause_loop(&self) -> Option<Command> {
        loop {
            match self.read_key_sync() {
                Some(key_code) => match self.key_to_command(key_code) {
                    Some(Pause) => {
                        return Some(Pause);
                    }
                    Some(Escape) => return Some(Escape),
                    _ => (),
                },
                _ => (),
            }
        }
    }
    fn read_key_async(&self) -> Option<KeyCode> {
        if event::poll(Duration::from_millis(0)).expect("could not poll event")
            && let Event::Key(key) = event::read().expect("could not read key event")
            && key.kind == Press
        {
            return Some(key.code);
        }
        None
    }
    fn read_key_sync(&self) -> Option<KeyCode> {
        match event::read().expect("could not read key event") {
            Event::Key(key) if key.kind == Press => Some(key.code),
            _ => None,
        }
    }
    fn key_to_command(&self, key_code: KeyCode) -> Option<Command> {
        match key_code {
            KeyCode::Up => Some(SnakeMoveUp),
            KeyCode::Down => Some(SnakeMoveDown),
            KeyCode::Left => Some(SnakeMoveLeft),
            KeyCode::Right => Some(SnakeMoveRight),
            KeyCode::Esc => Some(Escape),
            KeyCode::Char(' ') => Some(Pause),
            _ => None,
        }
    }
    fn should_spawn_food(&mut self) -> bool {
        self.rng.random_range(0..100) < self.food_spawn_probability
    }
    fn spawn_food(&mut self) {
        let max_x = self.grid.width();
        let max_y = self.grid.height();
        let x = self.rng.random_range(0..max_x);
        let y = self.rng.random_range(0..max_y);
        if *self.grid.cell(x, y) == Empty && !self.snake.occupies(x, y) {
            self.grid.change_cell(x, y, Food);
        }
    }
    pub fn score(&self) -> usize {
        self.snake.len().saturating_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, Game};
    use crate::grid::Grid;
    use crate::renderer::Renderer;
    use crate::snake::Snake;
    use crossterm::event::KeyCode;

    fn game_with_probability(food_spawn_probability: i32) -> Game {
        Game::new(
            Grid::new(8, 8),
            Snake::new((3, 3)),
            Renderer::new(),
            food_spawn_probability,
        )
    }

    #[test]
    fn key_to_command_maps_arrow_keys_escape_and_pause() {
        let game = game_with_probability(0);

        assert!(matches!(
            game.key_to_command(KeyCode::Up),
            Some(Command::SnakeMoveUp)
        ));
        assert!(matches!(
            game.key_to_command(KeyCode::Down),
            Some(Command::SnakeMoveDown)
        ));
        assert!(matches!(
            game.key_to_command(KeyCode::Left),
            Some(Command::SnakeMoveLeft)
        ));
        assert!(matches!(
            game.key_to_command(KeyCode::Right),
            Some(Command::SnakeMoveRight)
        ));
        assert!(matches!(
            game.key_to_command(KeyCode::Esc),
            Some(Command::Escape)
        ));
        assert!(matches!(
            game.key_to_command(KeyCode::Char(' ')),
            Some(Command::Pause)
        ));
    }

    #[test]
    fn key_to_command_ignores_unhandled_keys() {
        let game = game_with_probability(0);

        assert!(game.key_to_command(KeyCode::Enter).is_none());
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
}
