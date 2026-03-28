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

const FG_BLACK: &'static str = "\x1b[30m";
const FG_RED: &'static str = "\x1b[31m";
const FG_GREEN: &'static str = "\x1b[32m";
const FG_WHITE: &'static str = "\x1b[97m";
const BG_BLACK: &'static str = "\x1b[40m";
const BG_RED: &'static str = "\x1b[41m";
const BG_GREEN: &'static str = "\x1b[42m";
const BG_WHITE: &'static str = "\x1b[107m";
const RESET: &'static str = "\x1b[0m";

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
                    (RenderCell::Snake, RenderCell::Snake) => {
                        self.render_fullbox(FG_GREEN);
                    }
                    (RenderCell::Snake, RenderCell::Empty) => {
                        self.render_halfbox(FG_GREEN, BG_WHITE);
                    }
                    (RenderCell::Empty, RenderCell::Snake) => {
                        self.render_halfbox(FG_WHITE, BG_GREEN);
                    }
                    (RenderCell::Wall, RenderCell::Snake) => {
                        self.render_halfbox(FG_BLACK, BG_GREEN);
                    }
                    (RenderCell::Snake, RenderCell::Wall) => {
                        self.render_halfbox(FG_GREEN, BG_BLACK);
                    }
                    (RenderCell::Food, RenderCell::Snake) => {
                        self.render_halfbox(FG_RED, BG_GREEN);
                    }
                    (RenderCell::Snake, RenderCell::Food) => {
                        self.render_halfbox(FG_GREEN, BG_RED);
                    }
                    (RenderCell::Food, RenderCell::Empty) => {
                        self.render_halfbox(FG_RED, BG_WHITE);
                    }
                    (RenderCell::Empty, RenderCell::Food) => {
                        self.render_halfbox(FG_WHITE, BG_RED);
                    }
                    (RenderCell::Food, RenderCell::Wall) => {
                        self.render_halfbox(FG_RED, BG_BLACK);
                    }
                    (RenderCell::Wall, RenderCell::Food) => {
                        self.render_halfbox(FG_BLACK, BG_RED);
                    }
                    (RenderCell::Food, RenderCell::Food) => {
                        self.render_fullbox(FG_RED);
                    }
                    (RenderCell::Wall, RenderCell::Empty) => {
                        self.render_halfbox(FG_BLACK, BG_WHITE);
                    }
                    (RenderCell::Empty, RenderCell::Wall) => {
                        self.render_halfbox(FG_WHITE, BG_BLACK)
                    }
                    (RenderCell::Wall, RenderCell::Wall) => {
                        self.render_fullbox(FG_BLACK);
                    }
                }
            }
            y += 2;
            print!("\r\n")
        }
        std::io::stdout().flush().expect("could not flush stdout");
    }
    fn render_halfbox(&self, up_color: &str, bottom_color: &str) {
        print!("{}{}▀{}", up_color, bottom_color, RESET)
    }
    fn render_fullbox(&self, color: &str) {
        print!("{}█{}", color, RESET)
    }
}
