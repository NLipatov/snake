use crate::grid::Cell::Empty;
use crate::grid::Grid;
use crate::snake::Snake;
use std::io::Write;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }
    fn clear(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }
    pub fn render(&self, grid: &Grid, snake: &Snake) {
        self.clear();
        let mut y = 0;
        while y < grid.height() {
            for x in 0..grid.width() {
                let top = is_filled(grid, snake, x, y);
                let bottom = if y + 1 < grid.height() {
                    is_filled(grid, snake, x, y + 1)
                } else {
                    false
                };
                match (top, bottom) {
                    (false, false) => print!(" "),
                    (true, false) => print!("▀"),
                    (false, true) => print!("▄"),
                    (true, true) => print!("█"),
                }
            }
            y += 2;
            std::io::stdout().flush().unwrap();
            print!("\r\n")
        }
    }
}

fn is_filled(grid: &Grid, snake: &Snake, x: i32, y: i32) -> bool {
    snake.body().contains(&(x, y)) || *grid.cell(x, y) != Empty
}
