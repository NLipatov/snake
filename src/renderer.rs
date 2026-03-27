use crate::grid::{Cell, Grid};
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
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if snake.body().contains(&(x, y)) {
                    print!("■")
                } else {
                    match grid.cell(x, y) {
                        Cell::Empty => print!(" "),
                        Cell::Wall => print!("#"),
                        Cell::Food => print!("*"),
                    }
                }
            }
            std::io::stdout().flush().unwrap();
            print!("\r\n")
        }
    }
}
