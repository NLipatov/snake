use crate::engine::Command::{Escape, SnakeMoveDown, SnakeMoveLeft, SnakeMoveRight, SnakeMoveUp};
use crate::grid::Cell::{Empty, Food, Wall};
use crate::grid::Grid;
use crate::renderer::Renderer;
use crate::snake::{Direction, Snake};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::{RngExt, rngs};
use std::time::Duration;

enum Command {
    Escape,
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
        enable_raw_mode().unwrap();
        let result = self.run_loop();
        disable_raw_mode().unwrap();
        result
    }
    fn run_loop(&mut self) {
        loop {
            match self.read_key() {
                None => {}
                Some(key_code) => match self.key_to_command(key_code) {
                    None => {}
                    Some(command) => match command {
                        Escape => break,
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
            }
            if self.should_spawn_food() {
                self.spawn_food();
            }
            self.renderer.render(&self.grid, &self.snake);
            std::thread::sleep(Duration::from_millis(150));
        }
    }
    fn read_key(&self) -> Option<KeyCode> {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                return Some(key.code);
            }
        }
        None
    }
    fn key_to_command(&self, key_code: KeyCode) -> Option<Command> {
        match key_code {
            KeyCode::Up => Some(SnakeMoveUp),
            KeyCode::Down => Some(SnakeMoveDown),
            KeyCode::Left => Some(SnakeMoveLeft),
            KeyCode::Right => Some(SnakeMoveRight),
            KeyCode::Esc => Some(Escape),
            _ => None,
        }
    }
    fn should_spawn_food(&mut self) -> bool {
        let roll = self.rng.random_range(0..100);
        if roll >= self.food_spawn_probability {
            return false;
        }
        true
    }
    fn spawn_food(&mut self) {
        let max_x = self.grid.width();
        let max_y = self.grid.height();
        let x = self.rng.random_range(0..max_x);
        let y = self.rng.random_range(0..max_y);
        if *self.grid.cell(x, y) == Empty {
            self.grid.change_cell(x, y, Food);
        }
    }
}
