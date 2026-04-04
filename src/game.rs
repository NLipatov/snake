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
                        Pause => {
                            if let Some(Escape) = self.pause_loop() {
                                break;
                            }
                        }
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
            if let Some(key) = self.read_key_sync() {
                match self.key_to_command(key) {
                    Some(Escape) => return Some(Escape),
                    Some(Pause) => return Some(Pause),
                    _ => (),
                }
            }
        }
    }
    fn read_key_async(&self) -> Option<KeyCode> {
        if event::poll(Duration::from_millis(0)).expect("could not poll event") {
            return Self::key_code_from_event(event::read().expect("could not read key event"));
        }
        None
    }
    fn read_key_sync(&self) -> Option<KeyCode> {
        Self::key_code_from_event(event::read().expect("could not read key event"))
    }
    fn key_code_from_event(event: Event) -> Option<KeyCode> {
        match event {
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
        self.spawn_food_at(x, y);
    }
    fn spawn_food_at(&mut self, x: i32, y: i32) {
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
    use crate::grid::{Grid, GridCell};
    use crate::renderer::Renderer;
    use crate::snake::Snake;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

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
    fn key_code_from_event_reads_press_events() {
        let key_event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Up,
            KeyModifiers::NONE,
            KeyEventKind::Press,
        ));

        assert_eq!(Game::key_code_from_event(key_event), Some(KeyCode::Up));
    }

    #[test]
    fn key_code_from_event_ignores_repeat_release_and_non_key_events() {
        let repeat_event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Up,
            KeyModifiers::NONE,
            KeyEventKind::Repeat,
        ));
        let release_event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Up,
            KeyModifiers::NONE,
            KeyEventKind::Release,
        ));

        assert_eq!(Game::key_code_from_event(repeat_event), None);
        assert_eq!(Game::key_code_from_event(release_event), None);
        assert_eq!(Game::key_code_from_event(Event::Resize(80, 24)), None);
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

        game.spawn_food_at(4, 4);

        assert_eq!(game.grid.cell(4, 4), &GridCell::Food);
    }

    #[test]
    fn spawn_food_at_does_not_place_food_on_snake() {
        let mut game = game_with_probability(0);

        game.spawn_food_at(3, 3);

        assert_eq!(game.grid.cell(3, 3), &GridCell::Empty);
    }

    #[test]
    fn spawn_food_at_does_not_overwrite_existing_food_or_wall() {
        let mut game = game_with_probability(0);
        game.grid.change_cell(4, 4, GridCell::Food);

        game.spawn_food_at(4, 4);
        game.spawn_food_at(0, 0);

        assert_eq!(game.grid.cell(4, 4), &GridCell::Food);
        assert_eq!(game.grid.cell(0, 0), &GridCell::Wall);
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
