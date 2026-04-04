use crate::grid::GridCell::{Empty, Food, Wall};

#[derive(Debug, PartialEq, Eq)]
pub enum GridCell {
    Empty,
    Wall,
    Food,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

pub struct Grid {
    width: i32,
    height: i32,
    cells: Vec<GridCell>,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Grid {
        let mut cells = Vec::with_capacity((height * width) as usize);
        for y in 0..height {
            for x in 0..width {
                if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
                    cells.push(Wall)
                } else {
                    cells.push(Empty)
                }
            }
        }
        Grid {
            width,
            height,
            cells,
        }
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    fn index(&self, point: &Point) -> usize {
        point.y as usize * self.width as usize + point.x as usize
    }
    pub fn cell(&self, at: &Point) -> &GridCell {
        &self.cells[self.index(at)]
    }
    pub fn change_cell(&mut self, at: &Point, cell: GridCell) {
        let idx = self.index(at);
        self.cells[idx] = cell;
    }
    pub fn on_food_consumed(&mut self, at: &Point) {
        let idx = self.index(at);
        if self.cells[idx] == Food {
            self.cells[idx] = Empty;
        }
    }
    pub fn within_bounds(&self, at: &Point) -> bool {
        at.x >= 0 && at.y >= 0 && at.x < self.width() && at.y < self.height()
    }
}
