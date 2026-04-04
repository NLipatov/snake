use snake::grid::Point;
use snake::snake::{Direction, Snake};

fn point(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

#[test]
fn new_snake_starts_at_given_point() {
    let snake = Snake::new(point(5, 5));

    assert_eq!(snake.head(), point(5, 5));
    assert_eq!(snake.len(), 1);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn snake_moves_right_by_default() {
    let mut snake = Snake::new(point(5, 5));

    snake.move_snake();

    assert_eq!(snake.head(), point(6, 5));
}

#[test]
fn snake_moves_down_when_direction_is_down() {
    let mut snake = Snake::new(point(5, 5));
    snake.set_direction(Direction::Down);

    snake.move_snake();

    assert_eq!(snake.head(), point(5, 6));
}

#[test]
fn snake_moves_left_when_direction_is_left() {
    let mut snake = Snake::new(point(5, 5));
    snake.set_direction(Direction::Down);
    snake.set_direction(Direction::Left);

    snake.move_snake();

    assert_eq!(snake.head(), point(4, 5));
}

#[test]
fn snake_moves_up_when_direction_is_up() {
    let mut snake = Snake::new(point(5, 5));
    snake.set_direction(Direction::Down);
    snake.set_direction(Direction::Left);
    snake.set_direction(Direction::Up);

    snake.move_snake();

    assert_eq!(snake.head(), point(5, 4));
}

#[test]
fn snake_growth_keeps_previous_head_as_next_segment_after_move() {
    let mut snake = Snake::new(point(5, 5));
    snake.grow();

    snake.move_snake();

    assert_eq!(snake.head(), point(6, 5));
    assert_eq!(snake.len(), 2);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn snake_len_increases_with_each_growth() {
    let mut snake = Snake::new(point(5, 5));

    snake.grow();
    snake.grow();
    snake.grow();

    assert_eq!(snake.len(), 4);
}

#[test]
fn snake_cannot_reverse_from_right_to_left() {
    let mut snake = Snake::new(point(5, 5));
    snake.grow();
    snake.set_direction(Direction::Left);

    snake.move_snake();

    assert_eq!(snake.head(), point(6, 5));
    assert_eq!(snake.len(), 2);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn snake_cannot_reverse_from_left_to_right() {
    let mut snake = Snake::new(point(5, 5));
    snake.grow();
    snake.set_direction(Direction::Down);
    snake.set_direction(Direction::Left);
    snake.set_direction(Direction::Right);

    snake.move_snake();

    assert_eq!(snake.head(), point(4, 5));
    assert_eq!(snake.len(), 2);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn snake_cannot_reverse_from_up_to_down() {
    let mut snake = Snake::new(point(5, 5));
    snake.grow();
    snake.set_direction(Direction::Down);
    snake.set_direction(Direction::Left);
    snake.set_direction(Direction::Up);
    snake.set_direction(Direction::Down);

    snake.move_snake();

    assert_eq!(snake.head(), point(5, 4));
    assert_eq!(snake.len(), 2);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn snake_cannot_reverse_from_down_to_up() {
    let mut snake = Snake::new(point(5, 5));
    snake.grow();
    snake.set_direction(Direction::Down);
    snake.set_direction(Direction::Up);

    snake.move_snake();

    assert_eq!(snake.head(), point(5, 6));
    assert_eq!(snake.len(), 2);
    assert!(snake.occupies(&point(5, 5)));
}

#[test]
fn new_snake_has_no_self_collision() {
    let snake = Snake::new(point(2, 2));

    assert!(!snake.has_self_collision());
}

#[test]
fn snake_detects_self_collision() {
    let mut snake = Snake::new(point(2, 2));

    snake.grow();
    snake.move_snake();

    snake.grow();
    snake.set_direction(Direction::Down);
    snake.move_snake();

    snake.grow();
    snake.set_direction(Direction::Left);
    snake.move_snake();

    snake.grow();
    snake.set_direction(Direction::Up);
    snake.move_snake();

    assert!(snake.has_self_collision());
}

#[test]
fn snake_can_form_a_corner_after_turning() {
    let mut snake = Snake::new(point(2, 2));
    snake.grow();
    snake.move_snake();
    snake.set_direction(Direction::Down);
    snake.move_snake();

    assert_eq!(snake.head(), point(3, 3));
    assert!(snake.occupies(&point(3, 2)));
}

#[test]
fn snake_does_not_report_self_collision_for_distinct_body() {
    let mut snake = Snake::new(point(2, 2));
    snake.grow();
    snake.move_snake();
    snake.grow();
    snake.move_snake();

    assert!(!snake.has_self_collision());
}

#[test]
fn snake_occupies_head_cell() {
    let snake = Snake::new(point(2, 2));

    assert!(snake.occupies(&point(2, 2)));
}

#[test]
fn snake_occupies_head_and_body_cells_only() {
    let mut snake = Snake::new(point(2, 2));
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
    let snake = Snake::new(point(2, 2));

    assert!(!snake.occupies(&point(2, 3)));
}
