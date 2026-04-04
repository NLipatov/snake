use crate::grid::Point;
use crate::snake::Direction::{Down, Left, Right, Up};

pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
}

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub fn new(starting_point: Point) -> Snake {
        let mut snake = Snake {
            body: Vec::new(),
            direction: Right,
        };
        snake.body.push(starting_point);
        snake
    }
    pub fn head(&self) -> Point {
        self.body[0]
    }
    pub fn move_snake(&mut self) {
        for i in (1..self.body.len()).rev() {
            self.body[i] = self.body[i - 1];
        }
        match self.direction {
            Up => {
                self.body[0] = Point::new(self.body[0].x, self.body[0].y - 1);
            }
            Down => {
                self.body[0] = Point::new(self.body[0].x, self.body[0].y + 1);
            }
            Left => {
                self.body[0] = Point::new(self.body[0].x - 1, self.body[0].y);
            }
            Right => {
                self.body[0] = Point::new(self.body[0].x + 1, self.body[0].y);
            }
        }
    }
    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction == Down && direction == Up
            || self.direction == Up && direction == Down
            || self.direction == Left && direction == Right
            || self.direction == Right && direction == Left
        {
            return;
        }
        self.direction = direction;
    }
    pub fn grow(&mut self) {
        self.body.push(self.head());
    }
    pub fn has_self_collision(&self) -> bool {
        let head = self.head();
        self.body[1..].contains(&head)
    }
    pub fn occupies(&self, point: &Point) -> bool {
        self.body.contains(&point)
    }
    pub fn len(&self) -> usize {
        self.body.len()
    }
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
}
