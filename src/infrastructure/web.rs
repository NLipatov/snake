use crate::domain::game::{Game, GameCommand, GameResult};
use crate::domain::grid::{Grid, GridCell, Point};
use crate::domain::grid_geometry::GridGeometry;
use crate::domain::snake::{Direction, Snake};
use wasm_bindgen::prelude::*;

const DEFAULT_WIDTH: i32 = 32;
const DEFAULT_HEIGHT: i32 = 32;
const DEFAULT_START_X: i32 = 5;
const DEFAULT_START_Y: i32 = 5;
const DEFAULT_FOOD_SPAWN_ATTEMPT_PROBABILITY: i32 = 4;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct WebGame {
    game: Game,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl WebGame {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Result<WebGame, JsValue> {
        Self::with_config(
            DEFAULT_WIDTH,
            DEFAULT_HEIGHT,
            DEFAULT_START_X,
            DEFAULT_START_Y,
            DEFAULT_FOOD_SPAWN_ATTEMPT_PROBABILITY,
        )
    }

    pub fn with_config(
        width: i32,
        height: i32,
        start_x: i32,
        start_y: i32,
        food_spawn_attempt_probability: i32,
    ) -> Result<WebGame, JsValue> {
        let geometry = GridGeometry::new(width, height);
        let grid = Grid::new(geometry);
        let snake = Snake::new(Point::new(start_x, start_y), geometry)
            .map_err(|err| JsValue::from_str(&err))?;

        Ok(WebGame {
            game: Game::new(grid, snake, food_spawn_attempt_probability),
        })
    }

    pub fn tick(&mut self) -> bool {
        matches!(self.game.tick(), GameResult::Running)
    }

    pub fn move_up(&mut self) {
        self.game.apply_command(GameCommand::Move(Direction::Up));
    }

    pub fn move_down(&mut self) {
        self.game.apply_command(GameCommand::Move(Direction::Down));
    }

    pub fn move_left(&mut self) {
        self.game.apply_command(GameCommand::Move(Direction::Left));
    }

    pub fn move_right(&mut self) {
        self.game.apply_command(GameCommand::Move(Direction::Right));
    }

    pub fn score(&self) -> u32 {
        self.game.score() as u32
    }

    pub fn width(&self) -> i32 {
        self.game.grid().width()
    }

    pub fn height(&self) -> i32 {
        self.game.grid().height()
    }

    pub fn cell_at(&self, x: i32, y: i32) -> u8 {
        let point = Point::new(x, y);
        if self.game.snake().occupies(&point) {
            return 3;
        }
        if self.game.food_at(&point) {
            return 2;
        }
        if !self.game.grid().in_bounds(&point) {
            return u8::MAX;
        }
        match self.game.grid().cell(&point) {
            GridCell::Empty => 0,
            GridCell::Wall => 1,
        }
    }

    pub fn head_x(&self) -> i32 {
        self.game.snake().head().x
    }

    pub fn head_y(&self) -> i32 {
        self.game.snake().head().y
    }
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_HEIGHT, DEFAULT_START_X, DEFAULT_START_Y, DEFAULT_WIDTH, WebGame};

    #[test]
    fn new_uses_default_configuration() {
        let game = WebGame::new().expect("default web game should initialize");

        assert_eq!(game.width(), DEFAULT_WIDTH);
        assert_eq!(game.height(), DEFAULT_HEIGHT);
        assert_eq!(game.head_x(), DEFAULT_START_X);
        assert_eq!(game.head_y(), DEFAULT_START_Y);
        assert_eq!(game.score(), 0);
        assert_eq!(game.cell_at(DEFAULT_START_X, DEFAULT_START_Y), 3);
    }

    #[test]
    fn cell_at_reports_snake_wall_empty_and_out_of_bounds() {
        let game = WebGame::with_config(8, 8, 3, 3, 0).expect("web game should initialize");

        assert_eq!(game.cell_at(3, 3), 3);
        assert_eq!(game.cell_at(0, 0), 1);
        assert_eq!(game.cell_at(4, 3), 0);
        assert_eq!(game.cell_at(-1, 0), u8::MAX);
    }

    #[test]
    fn movement_methods_update_direction_and_head_position() {
        let mut move_up =
            WebGame::with_config(8, 8, 3, 3, 0).expect("web game should initialize in bounds");
        move_up.move_down();
        move_up.move_left();
        move_up.move_up();
        assert!(move_up.tick());
        assert_eq!((move_up.head_x(), move_up.head_y()), (3, 2));

        let mut move_down =
            WebGame::with_config(8, 8, 3, 3, 0).expect("web game should initialize in bounds");
        move_down.move_down();
        assert!(move_down.tick());
        assert_eq!((move_down.head_x(), move_down.head_y()), (3, 4));

        let mut move_left =
            WebGame::with_config(8, 8, 3, 3, 0).expect("web game should initialize in bounds");
        move_left.move_down();
        move_left.move_left();
        assert!(move_left.tick());
        assert_eq!((move_left.head_x(), move_left.head_y()), (2, 3));

        let mut move_right =
            WebGame::with_config(8, 8, 3, 3, 0).expect("web game should initialize in bounds");
        move_right.move_down();
        move_right.move_right();
        assert!(move_right.tick());
        assert_eq!((move_right.head_x(), move_right.head_y()), (4, 3));
    }

    #[test]
    fn tick_returns_false_when_game_is_over() {
        let mut game =
            WebGame::with_config(4, 4, 3, 1, 0).expect("web game should initialize in bounds");

        assert!(!game.tick());
        assert_eq!((game.head_x(), game.head_y()), (4, 1));
    }
}
