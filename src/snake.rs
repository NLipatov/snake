use crate::grid::{Grid, Point};
use crate::snake::Direction::{Down, Left, Right, Up};
use crate::snake::MoveResult::{Moved, SelfCollision};
use std::collections::VecDeque;

pub struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
    pending_growth: usize,
    occupancy: Vec<bool>,
    grid_width: usize,
}

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq)]
pub enum MoveResult {
    Moved,
    SelfCollision,
}

impl Snake {
    pub fn new(starting_point: Point, grid: &Grid) -> Result<Snake, String> {
        if !grid.within_bounds(&starting_point) {
            return Err(String::from("Starting point outside the grid bounds"));
        }
        let mut occupancy = vec![false; (grid.width() * grid.height()) as usize];
        occupancy[(starting_point.y * grid.width() + starting_point.x) as usize] = true;
        let mut snake = Snake {
            body: VecDeque::new(),
            direction: Right,
            pending_growth: 0,
            occupancy,
            grid_width: grid.width() as usize,
        };
        snake.body.push_front(starting_point);
        Ok(snake)
    }
    pub fn head(&self) -> Point {
        self.body[0]
    }
    pub fn move_snake(&mut self) -> MoveResult {
        let old_head = self.head();
        let new_head = match self.direction {
            Up => Point::new(old_head.x, old_head.y - 1),
            Down => Point::new(old_head.x, old_head.y + 1),
            Left => Point::new(old_head.x - 1, old_head.y),
            Right => Point::new(old_head.x + 1, old_head.y),
        };
        if self.occupies(&new_head) {
            return SelfCollision;
        }
        self.body.push_front(new_head);
        if let Some(idx) = self.occupancy_index(&new_head) {
            self.occupancy[idx] = true;
        }
        if self.pending_growth > 0 {
            self.pending_growth -= 1;
        } else if let Some(old_tail) = self.body.pop_back() {
            if let Some(idx) = self.occupancy_index(&old_tail) {
                self.occupancy[idx] = false;
            }
        }
        Moved
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
        self.pending_growth += 1;
    }
    pub fn occupies(&self, point: &Point) -> bool {
        self.occupancy_index(point)
            .and_then(|idx| self.occupancy.get(idx))
            .copied()
            .unwrap_or(false)
    }
    fn occupancy_index(&self, point: &Point) -> Option<usize> {
        if point.x < 0 || point.y < 0 {
            return None;
        }
        let idx = (point.y as usize)
            .checked_mul(self.grid_width)?
            .checked_add(point.x as usize)?;
        match idx < self.occupancy.len() {
            true => Some(idx),
            false => None,
        }
    }
    pub fn logical_len(&self) -> usize {
        self.body.len() + self.pending_growth
    }
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
}
