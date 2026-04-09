use crate::grid::GridCell::{Empty, Food, Wall};
use crate::grid_geometry::GridGeometry;

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
    grid_geometry: GridGeometry,
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
            cells,
            grid_geometry: GridGeometry::new(width, height),
        }
    }
    pub fn height(&self) -> i32 {
        self.grid_geometry.height()
    }
    pub fn width(&self) -> i32 {
        self.grid_geometry.width()
    }
    pub fn cell(&self, at: &Point) -> Option<&GridCell> {
        if let Some(idx) = self.grid_geometry.index(at) {
            return Some(&self.cells[idx]);
        }
        None
    }
    pub fn change_cell(&mut self, at: &Point, cell: GridCell) {
        if let Some(idx) = self.grid_geometry.index(at) {
            self.cells[idx] = cell;
        }
    }
    pub fn on_food_consumed(&mut self, at: &Point) {
        if let Some(idx) = self.grid_geometry.index(at) {
            if self.cells[idx] == Food {
                self.cells[idx] = Empty;
            }
        }
    }
    pub fn in_bounds(&self, point: &Point) -> bool {
        self.grid_geometry.in_bounds(point)
    }
}
