use crate::domain::game::GameResult::{GameOver, Running};
use crate::domain::grid::GridCell::{Empty, Food, Wall};
use crate::domain::grid::{Grid, Point};
use crate::domain::snake::{Direction, MoveResult, Snake};
use rand::{RngExt, rngs};

pub enum GameCommand {
    Move(Direction),
}

pub enum GameResult {
    GameOver,
    Running,
}

pub struct Game {
    grid: Grid,
    snake: Snake,
    rng: rngs::ThreadRng,
    food_spawn_probability: i32,
}

impl Game {
    pub fn new(grid: Grid, snake: Snake, food_spawn_probability: i32) -> Game {
        let rng = rand::rng();
        Game {
            grid,
            snake,
            rng,
            food_spawn_probability,
        }
    }
    pub fn apply_command(&mut self, command: GameCommand) {
        match command {
            GameCommand::Move(direction) => {
                self.snake.set_direction(direction);
            }
        }
    }
    pub fn tick(&mut self) -> GameResult {
        if let MoveResult::SelfCollision = self.snake.move_snake() {
            return GameOver;
        }
        let head = self.snake.head();
        if !self.grid.in_bounds(&head) {
            return GameOver;
        }
        match self.grid.cell(&head) {
            Wall => return GameOver,
            Food => {
                self.snake.grow();
                self.grid.on_food_consumed(&head)
            }
            _ => (),
        }
        if self.should_spawn_food() {
            self.spawn_food();
        }
        Running
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
        if self.grid.in_bounds(point)
            && matches!(self.grid.cell(point), Empty)
            && !self.snake.occupies(point)
        {
            self.grid.change_cell(point, Food);
        }
    }
    pub fn score(&self) -> usize {
        self.snake.logical_len().saturating_sub(1)
    }
    pub fn snake(&self) -> &Snake {
        &self.snake
    }
    pub fn grid(&self) -> &Grid {
        &self.grid
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, GameCommand, GameResult};
    use crate::domain::grid::{Grid, GridCell, Point};
    use crate::domain::grid_geometry::GridGeometry;
    use crate::domain::snake::{Direction, Snake};

    fn point(x: i32, y: i32) -> Point {
        Point::new(x, y)
    }

    fn game_at(starting_point: Point, food_spawn_probability: i32) -> Game {
        let geometry = GridGeometry::new(8, 8);
        let grid = Grid::new(geometry);
        let snake = match Snake::new(starting_point, geometry) {
            Ok(snake) => snake,
            Err(e) => panic!("{}", e),
        };
        Game::new(grid, snake, food_spawn_probability)
    }

    fn game_with_probability(food_spawn_probability: i32) -> Game {
        game_at(point(3, 3), food_spawn_probability)
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
    fn game_exposes_current_grid_snake_and_score() {
        let mut game = game_with_probability(0);
        game.snake.grow();

        assert_eq!(game.grid().width(), 8);
        assert!(game.snake().occupies(&point(3, 3)));
        assert_eq!(game.score(), 1);
    }

    #[test]
    fn apply_command_changes_snake_direction_for_next_tick() {
        let mut game = game_with_probability(0);

        game.apply_command(GameCommand::Move(Direction::Down));

        assert!(matches!(game.tick(), GameResult::Running));
        assert_eq!(game.snake().head(), point(3, 4));
    }

    #[test]
    fn apply_command_updates_direction_immediately() {
        let mut game = game_with_probability(0);

        game.apply_command(GameCommand::Move(Direction::Down));
        game.apply_command(GameCommand::Move(Direction::Left));

        assert!(matches!(game.tick(), GameResult::Running));
        assert_eq!(game.snake().head(), point(2, 3));
    }

    #[test]
    fn tick_returns_game_over_when_snake_hits_wall() {
        let mut game = game_at(point(6, 3), 0);

        assert!(matches!(game.tick(), GameResult::GameOver));
        assert_eq!(game.snake().head(), point(7, 3));
    }

    #[test]
    fn tick_consumes_food_and_increases_score() {
        let mut game = game_with_probability(0);
        game.grid.change_cell(&point(4, 3), GridCell::Food);

        assert!(matches!(game.tick(), GameResult::Running));
        assert_eq!(game.score(), 1);
        assert_eq!(game.grid().cell(&point(4, 3)), &GridCell::Empty);
        assert_eq!(game.snake().head(), point(4, 3));
    }
}
