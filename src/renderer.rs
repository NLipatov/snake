use crate::grid::GridCell::Empty;
use crate::grid::{Grid, GridCell};
use crate::snake::Snake;
use std::io::Write;

pub struct Renderer {}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

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
                match (top.to_color(), bottom.to_color()) {
                    (None, None) => {
                        print!(" ");
                    }
                    (None, Some(color)) => self.render_bottom_half(color.fg),
                    (Some(color), None) => self.render_top_half(color.fg),
                    (Some(fg), Some(bg)) => match fg == bg {
                        true => self.render_fullbox(fg.fg),
                        false => self.render_halfbox(fg.fg, bg.bg),
                    },
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
    fn render_top_half(&self, color: &str) {
        print!("{}▀{}", color, RESET)
    }
    fn render_bottom_half(&self, color: &str) {
        print!("{}▄{}", color, RESET)
    }
    fn render_fullbox(&self, color: &str) {
        print!("{}█{}", color, RESET)
    }
}

const FG_RED: &str = "\x1b[31m";
const FG_GREEN: &str = "\x1b[32m";
const FG_BRIGHT_BLACK: &str = "\x1b[90m";
const BG_RED: &str = "\x1b[41m";
const BG_GREEN: &str = "\x1b[42m";
const BG_BRIGHT_BLACK: &str = "\x1b[100m";
const RESET: &str = "\x1b[0m";

#[derive(Debug, PartialEq)]
struct Color {
    fg: &'static str,
    bg: &'static str,
}

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
    fn to_color(&self) -> Option<Color> {
        match self {
            RenderCell::Empty => None,
            RenderCell::Food => Some(Color {
                fg: FG_RED,
                bg: BG_RED,
            }),
            RenderCell::Wall => Some(Color {
                fg: FG_BRIGHT_BLACK,
                bg: BG_BRIGHT_BLACK,
            }),
            RenderCell::Snake => Some(Color {
                fg: FG_GREEN,
                bg: BG_GREEN,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BG_BRIGHT_BLACK, BG_GREEN, BG_RED, Color, FG_BRIGHT_BLACK, FG_GREEN, FG_RED, RenderCell,
    };
    use crate::grid::{Grid, GridCell};
    use crate::snake::Snake;

    #[test]
    fn render_cell_prefers_snake_over_grid_contents() {
        let mut grid = Grid::new(5, 5);
        grid.change_cell(2, 2, GridCell::Food);
        let snake = Snake::new((2, 2));

        assert!(matches!(
            RenderCell::new(&grid, &snake, 2, 2),
            RenderCell::Snake
        ));
    }

    #[test]
    fn render_cell_reads_food_wall_and_empty_from_grid() {
        let mut grid = Grid::new(5, 5);
        let snake = Snake::new((1, 1));
        grid.change_cell(2, 2, GridCell::Food);

        assert!(matches!(
            RenderCell::new(&grid, &snake, 2, 2),
            RenderCell::Food
        ));
        assert!(matches!(
            RenderCell::new(&grid, &snake, 0, 0),
            RenderCell::Wall
        ));
        assert!(matches!(
            RenderCell::new(&grid, &snake, 3, 3),
            RenderCell::Empty
        ));
    }

    #[test]
    fn to_color_returns_expected_palette() {
        assert_eq!(
            RenderCell::Food.to_color(),
            Some(Color {
                fg: FG_RED,
                bg: BG_RED,
            })
        );
        assert_eq!(
            RenderCell::Snake.to_color(),
            Some(Color {
                fg: FG_GREEN,
                bg: BG_GREEN,
            })
        );
        assert_eq!(
            RenderCell::Wall.to_color(),
            Some(Color {
                fg: FG_BRIGHT_BLACK,
                bg: BG_BRIGHT_BLACK,
            })
        );
        assert_eq!(RenderCell::Empty.to_color(), None);
    }
}
