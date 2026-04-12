use crate::domain::grid::Point;
use crate::domain::grid_geometry::GridGeometry;
use crate::domain::snake::Direction::{Down, Left, Right, Up};
use crate::domain::snake::MoveResult::{Moved, SelfCollision};
use std::collections::VecDeque;

pub struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
    pending_growth: usize,
    occupancy: Vec<bool>,
    grid_geometry: GridGeometry,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub fn new(starting_point: Point, grid_geometry: GridGeometry) -> Result<Snake, String> {
        let height = grid_geometry.height();
        let width = grid_geometry.width();
        let capacity = (width * height) as usize;
        if !grid_geometry.in_bounds(&starting_point) {
            return Err(String::from("Starting point outside the grid bounds"));
        }
        let mut occupancy = vec![false; capacity];
        let starting_idx = grid_geometry
            .index(&starting_point)
            .expect("starting point should be within grid bounds");
        occupancy[starting_idx] = true;
        let mut snake = Snake {
            body: VecDeque::new(),
            direction: Right,
            pending_growth: 0,
            occupancy,
            grid_geometry,
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
        let growing = self.pending_growth > 0;
        let old_tail = if growing {
            None
        } else {
            self.body.back().copied()
        };

        if let Some(tail) = old_tail
            && let Some(idx) = self.grid_geometry.index(&tail)
        {
            self.occupancy[idx] = false;
        }

        if self.occupies(&new_head) {
            if let Some(tail) = old_tail
                && let Some(idx) = self.grid_geometry.index(&tail)
            {
                self.occupancy[idx] = true;
            }
            return SelfCollision;
        }

        self.body.push_front(new_head);
        if let Some(idx) = self.grid_geometry.index(&new_head) {
            self.occupancy[idx] = true;
        }
        if growing {
            self.pending_growth -= 1;
        } else {
            self.body.pop_back();
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
        self.grid_geometry
            .index(point)
            .and_then(|idx| self.occupancy.get(idx))
            .copied()
            .unwrap_or(false)
    }
    pub fn logical_len(&self) -> usize {
        self.body.len() + self.pending_growth
    }
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
    pub fn occupied_points(&self) -> impl Iterator<Item = &Point> + '_ {
        self.body.iter()
    }
}
