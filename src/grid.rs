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
    geometry: GridGeometry,
    cells: Vec<GridCell>,
}

impl Grid {
    pub fn new(geometry: GridGeometry) -> Grid {
        let cells = Grid::generate_grid(&geometry);
        Grid { cells, geometry }
    }
    fn generate_grid(geometry: &GridGeometry) -> Vec<GridCell> {
        let height = geometry.height();
        let width = geometry.width();
        let capacity = (height * width) as usize;
        let mut cells = Vec::with_capacity(capacity);
        for y in 0..height {
            for x in 0..width {
                if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
                    cells.push(Wall)
                } else {
                    cells.push(Empty)
                }
            }
        }
        cells
    }
    pub fn height(&self) -> i32 {
        self.geometry.height()
    }
    pub fn width(&self) -> i32 {
        self.geometry.width()
    }
    pub fn cell(&self, at: &Point) -> &GridCell {
        let idx = self
            .geometry
            .index(at)
            .expect("point should be withing grid bounds");
        &self.cells[idx]
    }
    pub fn change_cell(&mut self, at: &Point, cell: GridCell) {
        if let Some(idx) = self.geometry.index(at) {
            self.cells[idx] = cell;
        }
    }
    pub fn on_food_consumed(&mut self, at: &Point) {
        if let Some(idx) = self.geometry.index(at) {
            if self.cells[idx] == Food {
                self.cells[idx] = Empty;
            }
        }
    }
    pub fn in_bounds(&self, point: &Point) -> bool {
        self.geometry.in_bounds(point)
    }
}
