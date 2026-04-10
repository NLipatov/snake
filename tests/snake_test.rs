use snake::grid::Point;
use snake::grid_geometry::GridGeometry;
use snake::snake::{Direction, MoveResult, Snake};

fn point(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

fn snake_at(starting_point: Point) -> Snake {
    let geometry = GridGeometry::new(8, 8);
    Snake::new(starting_point, geometry).expect("snake should fit in test grid")
}

#[test]
fn new_snake_starts_at_given_point() {
    let snake = snake_at(point(5, 5));

    assert_eq!(snake.head(), point(5, 5));
    assert_eq!(snake.logical_len(), 1);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn snake_moves_right_by_default() {
    let mut snake = snake_at(point(5, 5));

    snake.move_snake();

    assert_eq!(snake.head(), point(6, 5));
}

#[test]
fn snake_moves_down_when_direction_is_down() {
    let mut snake = snake_at(point(5, 5));
    snake.set_direction(Direction::Down);

    snake.move_snake();

    assert_eq!(snake.head(), point(5, 6));
}

#[test]
fn snake_moves_left_when_direction_is_left() {
    let mut snake = snake_at(point(5, 5));
    snake.set_direction(Direction::Down);
    snake.set_direction(Direction::Left);

    snake.move_snake();

    assert_eq!(snake.head(), point(4, 5));
}

#[test]
fn snake_moves_up_when_direction_is_up() {
    let mut snake = snake_at(point(5, 5));
    snake.set_direction(Direction::Down);
    snake.set_direction(Direction::Left);
    snake.set_direction(Direction::Up);

    snake.move_snake();

    assert_eq!(snake.head(), point(5, 4));
}

#[test]
fn snake_growth_keeps_previous_head_as_next_segment_after_move() {
    let mut snake = snake_at(point(5, 5));
    snake.grow();

    snake.move_snake();

    assert_eq!(snake.head(), point(6, 5));
    assert_eq!(snake.logical_len(), 2);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn snake_len_increases_with_each_growth() {
    let mut snake = snake_at(point(5, 5));

    snake.grow();
    snake.grow();
    snake.grow();

    assert_eq!(snake.logical_len(), 4);
}

#[test]
fn snake_cannot_reverse_from_right_to_left() {
    let mut snake = snake_at(point(5, 5));
    snake.grow();
    snake.set_direction(Direction::Left);

    snake.move_snake();

    assert_eq!(snake.head(), point(6, 5));
    assert_eq!(snake.logical_len(), 2);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn snake_cannot_reverse_from_left_to_right() {
    let mut snake = snake_at(point(5, 5));
    snake.grow();
    snake.set_direction(Direction::Down);
    snake.set_direction(Direction::Left);
    snake.set_direction(Direction::Right);

    snake.move_snake();

    assert_eq!(snake.head(), point(4, 5));
    assert_eq!(snake.logical_len(), 2);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn snake_cannot_reverse_from_up_to_down() {
    let mut snake = snake_at(point(5, 5));
    snake.grow();
    snake.set_direction(Direction::Down);
    snake.set_direction(Direction::Left);
    snake.set_direction(Direction::Up);
    snake.set_direction(Direction::Down);

    snake.move_snake();

    assert_eq!(snake.head(), point(5, 4));
    assert_eq!(snake.logical_len(), 2);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn snake_cannot_reverse_from_down_to_up() {
    let mut snake = snake_at(point(5, 5));
    snake.grow();
    snake.set_direction(Direction::Down);
    snake.set_direction(Direction::Up);

    snake.move_snake();

    assert_eq!(snake.head(), point(5, 6));
    assert_eq!(snake.logical_len(), 2);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn new_snake_has_no_self_collision() {
    let mut snake = snake_at(point(2, 2));

    assert!(matches!(snake.move_snake(), MoveResult::Moved));
}

#[test]
fn snake_detects_self_collision() {
    let mut snake = snake_at(point(2, 2));

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
    assert!(matches!(snake.move_snake(), MoveResult::SelfCollision));
}

#[test]
fn snake_can_form_a_corner_after_turning() {
    let mut snake = snake_at(point(2, 2));
    snake.grow();
    snake.move_snake();
    snake.set_direction(Direction::Down);
    snake.move_snake();

    assert_eq!(snake.head(), point(3, 3));
    assert!(snake.occupies(&point(3, 2)));
}

#[test]
fn snake_does_not_report_self_collision_for_distinct_body() {
    let mut snake = snake_at(point(2, 2));
    snake.grow();
    assert!(matches!(snake.move_snake(), MoveResult::Moved));
    snake.grow();
    assert!(matches!(snake.move_snake(), MoveResult::Moved));
}

#[test]
fn snake_can_move_into_old_tail_cell_without_growing() {
    let mut snake = snake_at(point(2, 2));

    snake.grow();
    assert!(matches!(snake.move_snake(), MoveResult::Moved));

    snake.grow();
    snake.set_direction(Direction::Down);
    assert!(matches!(snake.move_snake(), MoveResult::Moved));

    snake.grow();
    snake.set_direction(Direction::Left);
    assert!(matches!(snake.move_snake(), MoveResult::Moved));

    snake.set_direction(Direction::Up);

    assert!(matches!(snake.move_snake(), MoveResult::Moved));
    assert_eq!(snake.head(), point(2, 2));
    assert_eq!(snake.logical_len(), 4);
    assert!(snake.occupies(&point(2, 2)));
}

#[test]
fn snake_occupies_head_cell() {
    let snake = snake_at(point(2, 2));

    assert!(snake.occupies(&point(2, 2)));
}

#[test]
fn snake_occupies_head_and_body_cells_only() {
    let mut snake = snake_at(point(2, 2));
    snake.grow();
    snake.move_snake();
    snake.grow();
    snake.move_snake();

    assert!(!snake.occupies(&point(1, 2)));
    assert!(snake.occupies(&point(2, 2)));
    assert!(snake.occupies(&point(3, 2)));
    assert!(snake.occupies(&point(4, 2)));
    assert!(!snake.occupies(&point(5, 2)));
}

#[test]
fn snake_does_not_occupy_unoccupied_cell() {
    let snake = snake_at(point(2, 2));

    assert!(!snake.occupies(&point(2, 3)));
}
