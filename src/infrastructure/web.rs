use crate::domain::game::{Game, GameCommand, GameResult};
use crate::domain::grid::{Grid, GridCell, Point};
use crate::domain::grid_geometry::GridGeometry;
use crate::domain::snake::{Direction, Snake};
use wasm_bindgen::prelude::*;

const DEFAULT_WIDTH: i32 = 32;
const DEFAULT_HEIGHT: i32 = 32;
const DEFAULT_START_X: i32 = 5;
const DEFAULT_START_Y: i32 = 5;
const DEFAULT_FOOD_SPAWN_PROBABILITY: i32 = 4;

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
            DEFAULT_FOOD_SPAWN_PROBABILITY,
        )
    }

    pub fn with_config(
        width: i32,
        height: i32,
        start_x: i32,
        start_y: i32,
        food_spawn_probability: i32,
    ) -> Result<WebGame, JsValue> {
        let geometry = GridGeometry::new(width, height);
        let grid = Grid::new(geometry);
        let snake = Snake::new(Point::new(start_x, start_y), geometry)
            .map_err(|err| JsValue::from_str(&err))?;

        Ok(WebGame {
            game: Game::new(grid, snake, food_spawn_probability),
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
        if !self.game.grid().in_bounds(&point) {
            return u8::MAX;
        }
        match self.game.grid().cell(&point) {
            GridCell::Empty => 0,
            GridCell::Wall => 1,
            GridCell::Food => 2,
        }
    }

    pub fn head_x(&self) -> i32 {
        self.game.snake().head().x
    }

    pub fn head_y(&self) -> i32 {
        self.game.snake().head().y
    }
}
