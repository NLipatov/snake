use crate::domain::game::GameResult::{GameOver, Running};
use crate::domain::grid::GridCell::{Empty, Wall};
use crate::domain::grid::{Grid, Point};
use crate::domain::snake::{Direction, MoveResult, Snake};
use rand::{RngExt, rngs};
use std::collections::HashSet;

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
    food_spawn_attempt_probability: i32,
    food_points: HashSet<Point>,
}

impl Game {
    pub fn new(grid: Grid, snake: Snake, food_spawn_attempt_probability: i32) -> Game {
        let rng = rand::rng();
        assert!(
            (0..=100).contains(&food_spawn_attempt_probability),
            "food_spawn_attempt_probability must be in 0..=100"
        );
        Game {
            grid,
            snake,
            rng,
            food_spawn_attempt_probability,
            food_points: HashSet::new(),
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
        if self.grid.cell(&head) == &Wall {
            return GameOver;
        }
        if self.food_points.contains(&head) {
            self.snake.grow();
            self.food_points.remove(&head);
        }
        if self.should_attempt_food_spawn() {
            self.attempt_food_spawn();
        }
        Running
    }
    fn should_attempt_food_spawn(&mut self) -> bool {
        self.rng.random_range(0..100) < self.food_spawn_attempt_probability
    }
    fn attempt_food_spawn(&mut self) {
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
            self.food_points.insert(*point);
        }
    }
    pub fn snake_points(&self) -> impl Iterator<Item = &Point> + '_ {
        self.snake.occupied_points()
    }
    pub fn snake_len(&self) -> usize {
        self.snake.occupied_len()
    }
    pub fn snake_point_at(&self, index: usize) -> Option<Point> {
        self.snake.occupied_point_at(index)
    }
    pub fn food_points(&self) -> impl Iterator<Item = &Point> + '_ {
        self.food_points.iter()
    }
    pub fn food_len(&self) -> usize {
        self.food_points.len()
    }
    pub fn food_point_at(&self, index: usize) -> Option<Point> {
        self.food_points.iter().nth(index).copied()
    }
    pub fn food_at(&self, point: &Point) -> bool {
        self.food_points.contains(point)
    }
    pub fn snake_at(&self, point: &Point) -> bool {
        self.snake.occupies(point)
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
    use crate::domain::snake::{Direction, MoveResult, Snake};

    fn point(x: i32, y: i32) -> Point {
        Point::new(x, y)
    }

    fn game_at(starting_point: Point, food_spawn_attempt_probability: i32) -> Game {
        let geometry = GridGeometry::new(8, 8);
        let grid = Grid::new(geometry);
        let snake = match Snake::new(starting_point, geometry) {
            Ok(snake) => snake,
            Err(e) => panic!("{}", e),
        };
        Game::new(grid, snake, food_spawn_attempt_probability)
    }

    fn game_with_probability(food_spawn_attempt_probability: i32) -> Game {
        game_at(point(3, 3), food_spawn_attempt_probability)
    }

    #[test]
    fn should_attempt_food_spawn_is_never_true_at_zero_percent() {
        let mut game = game_with_probability(0);

        for _ in 0..100 {
            assert!(!game.should_attempt_food_spawn());
        }
    }

    #[test]
    fn should_attempt_food_spawn_is_always_true_at_hundred_percent() {
        let mut game = game_with_probability(100);

        for _ in 0..100 {
            assert!(game.should_attempt_food_spawn());
        }
    }

    #[test]
    #[should_panic(expected = "food_spawn_attempt_probability must be in 0..=100")]
    fn new_panics_when_food_spawn_attempt_probability_is_below_zero() {
        let geometry = GridGeometry::new(8, 8);
        let grid = Grid::new(geometry);
        let snake = Snake::new(point(3, 3), geometry).expect("snake should fit in grid");

        let _ = Game::new(grid, snake, -1);
    }

    #[test]
    #[should_panic(expected = "food_spawn_attempt_probability must be in 0..=100")]
    fn new_panics_when_food_spawn_attempt_probability_is_above_hundred() {
        let geometry = GridGeometry::new(8, 8);
        let grid = Grid::new(geometry);
        let snake = Snake::new(point(3, 3), geometry).expect("snake should fit in grid");

        let _ = Game::new(grid, snake, 101);
    }

    #[test]
    fn spawn_food_at_places_food_on_empty_unoccupied_cell() {
        let mut game = game_with_probability(0);

        game.spawn_food_at(&point(4, 4));

        assert!(game.food_points().any(|p| *p == point(4, 4)));
    }

    #[test]
    fn spawn_food_at_does_not_place_food_on_snake() {
        let mut game = game_with_probability(0);

        game.spawn_food_at(&point(3, 3));

        assert!(!game.food_points().any(|p| *p == point(3, 3)));
    }

    #[test]
    fn spawn_food_at_does_not_overwrite_existing_food_or_wall() {
        let mut game = game_with_probability(0);
        game.spawn_food_at(&point(4, 4));

        game.spawn_food_at(&point(4, 4));
        game.spawn_food_at(&point(0, 0));

        assert_eq!(game.food_points().filter(|p| **p == point(4, 4)).count(), 1);
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
    fn game_exposes_snake_and_food_accessors() {
        let mut game = game_with_probability(0);
        game.snake.grow();
        game.spawn_food_at(&point(4, 4));

        let snake_points: Vec<Point> = game.snake_points().copied().collect();
        let food_points: Vec<Point> = game.food_points().copied().collect();

        assert_eq!(snake_points, vec![point(3, 3)]);
        assert_eq!(game.snake_len(), 1);
        assert_eq!(game.snake_point_at(0), Some(point(3, 3)));
        assert_eq!(game.snake_point_at(1), None);
        assert!(game.snake_at(&point(3, 3)));
        assert!(!game.snake_at(&point(4, 4)));

        assert_eq!(food_points, vec![point(4, 4)]);
        assert_eq!(game.food_len(), 1);
        assert_eq!(game.food_point_at(0), Some(point(4, 4)));
        assert_eq!(game.food_point_at(1), None);
        assert!(game.food_at(&point(4, 4)));
        assert!(!game.food_at(&point(3, 3)));
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
    fn tick_returns_game_over_when_snake_moves_out_of_bounds() {
        let mut game = game_at(point(7, 3), 0);

        assert!(matches!(game.tick(), GameResult::GameOver));
        assert_eq!(game.snake().head(), point(8, 3));
    }

    #[test]
    fn tick_consumes_food_and_increases_score() {
        let mut game = game_with_probability(0);
        game.spawn_food_at(&point(4, 3));

        assert!(matches!(game.tick(), GameResult::Running));
        assert_eq!(game.score(), 1);
        assert!(!game.food_points().any(|p| *p == point(4, 3)));
        assert_eq!(game.snake().head(), point(4, 3));
    }

    #[test]
    fn tick_returns_game_over_when_snake_hits_itself() {
        let geometry = GridGeometry::new(8, 8);
        let grid = Grid::new(geometry);
        let mut snake = Snake::new(point(2, 2), geometry).expect("snake should fit in grid");

        snake.grow();
        assert!(matches!(snake.move_snake(), MoveResult::Moved));

        snake.grow();
        snake.set_direction(Direction::Down);
        assert!(matches!(snake.move_snake(), MoveResult::Moved));

        snake.grow();
        snake.set_direction(Direction::Left);
        assert!(matches!(snake.move_snake(), MoveResult::Moved));

        snake.grow();
        snake.set_direction(Direction::Up);
        let mut game = Game::new(grid, snake, 0);

        assert!(matches!(game.tick(), GameResult::GameOver));
        assert_eq!(game.snake().head(), point(2, 3));
    }
}
