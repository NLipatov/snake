use crate::grid::GridCell::Empty;
use crate::grid::{Grid, GridCell};
use crate::snake::Snake;
use std::io::Write;

enum RenderCell {
    Empty,
    Food,
    Wall,
    Snake,
}

impl RenderCell {
    fn new(grid: &Grid, snake: &Snake, x: i32, y: i32) -> RenderCell {
        if snake.occupies(x, y) {
            return RenderCell::Snake;
        }
        match grid.cell(x, y) {
            Empty => RenderCell::Empty,
            GridCell::Food => RenderCell::Food,
            GridCell::Wall => RenderCell::Wall,
        }
    }
}

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
                let top = RenderCell::new(grid, snake, x, y);
                let bottom = if y + 1 < grid.height() {
                    RenderCell::new(grid, snake, x, y + 1)
                } else {
                    RenderCell::Empty
                };
                match (top, bottom) {
                    (RenderCell::Empty, RenderCell::Empty) => print!(" "),
                    (
                        RenderCell::Food | RenderCell::Wall | RenderCell::Snake,
                        RenderCell::Empty,
                    ) => print!("▀"),
                    (
                        RenderCell::Empty,
                        RenderCell::Food | RenderCell::Wall | RenderCell::Snake,
                    ) => print!("▄"),
                    (
                        RenderCell::Food | RenderCell::Wall | RenderCell::Snake,
                        RenderCell::Food | RenderCell::Wall | RenderCell::Snake,
                    ) => print!("█"),
                }
            }
            y += 2;
            print!("\r\n")
        }
        std::io::stdout().flush().expect("could not flush stdout");
    }
}
