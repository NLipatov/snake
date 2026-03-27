use crate::grid::Cell::{Empty, Food, Wall};

#[derive(PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    Food,
}

pub struct Grid {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
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
    pub fn cell(&self, x: i32, y: i32) -> &Cell {
        &self.cells[y as usize * self.width as usize + x as usize]
    }
    pub fn change_cell(&mut self, x: i32, y: i32, cell: Cell) {
        self.cells[y as usize * self.width as usize + x as usize] = cell;
    }
    pub fn on_food_consumed(&mut self, x: i32, y: i32) {
        if self.cells[y as usize * self.width as usize + x as usize] == Food {
            self.cells[y as usize * self.width as usize + x as usize] = Empty;
        }
    }
}
