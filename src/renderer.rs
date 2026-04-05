use crate::grid::GridCell::Empty;
use crate::grid::{Grid, GridCell, Point};
use crate::snake::Snake;
use std::io::Write;

pub struct RenderState<'a> {
    grid: &'a Grid,
    snake: &'a Snake,
    score: usize,
}

impl<'a> RenderState<'a> {
    pub fn new(grid: &'a Grid, snake: &'a Snake, score: usize) -> Self {
        Self { grid, snake, score }
    }

    pub fn grid(&self) -> &'a Grid {
        self.grid
    }

    pub fn snake(&self) -> &'a Snake {
        self.snake
    }

    pub fn score(&self) -> usize {
        self.score
    }
}

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
    fn clear<W: Write>(&self, out: &mut W) {
        write!(out, "\x1B[2J\x1B[1;1H").expect("could not write clear sequence");
    }
    pub fn render(&self, render_state: RenderState<'_>) {
        let mut stdout = std::io::stdout();
        self.render_to(&mut stdout, render_state);
        stdout.flush().expect("could not flush stdout");
    }
    fn render_to<W: Write>(&self, out: &mut W, render_state: RenderState<'_>) {
        self.clear(out);
        self.render_header(out, &render_state);
        self.render_grid(out, &render_state);
    }
    fn render_header<W: Write>(&self, out: &mut W, render_state: &RenderState<'_>) {
        write!(
            out,
            "{FG_DIM}Score{RESET} {FG_GREEN}{}{RESET}\r\n",
            render_state.score
        )
        .expect("could not write header");
    }
    fn render_grid<W: Write>(&self, out: &mut W, render_state: &RenderState<'_>) {
        let mut y = 0;
        while y < render_state.grid.height() {
            for x in 0..render_state.grid.width() {
                let top_point = Point::new(x, y);
                let bottom_point = Point::new(x, y + 1);
                let top = RenderCell::new(render_state.grid, render_state.snake, &top_point);
                let bottom = if y + 1 < render_state.grid.height() {
                    RenderCell::new(render_state.grid, render_state.snake, &bottom_point)
                } else {
                    RenderCell::Empty
                };
                match (top.to_color(), bottom.to_color()) {
                    (None, None) => {
                        write!(out, " ").expect("could not write empty cell");
                    }
                    (None, Some(color)) => self.render_bottom_half(out, color.fg),
                    (Some(color), None) => self.render_top_half(out, color.fg),
                    (Some(fg), Some(bg)) => match fg == bg {
                        true => self.render_fullbox(out, fg.fg),
                        false => self.render_halfbox(out, fg.fg, bg.bg),
                    },
                }
            }
            y += 2;
            write!(out, "\r\n").expect("could not write row break")
        }
    }
    fn render_halfbox<W: Write>(&self, out: &mut W, up_color: &str, bottom_color: &str) {
        write!(out, "{}{}▀{}", up_color, bottom_color, RESET).expect("could not write half box")
    }
    fn render_top_half<W: Write>(&self, out: &mut W, color: &str) {
        write!(out, "{}▀{}", color, RESET).expect("could not write top half")
    }
    fn render_bottom_half<W: Write>(&self, out: &mut W, color: &str) {
        write!(out, "{}▄{}", color, RESET).expect("could not write bottom half")
    }
    fn render_fullbox<W: Write>(&self, out: &mut W, color: &str) {
        write!(out, "{}█{}", color, RESET).expect("could not write full box")
    }
}

const FG_DIM: &str = "\x1b[2m";
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
    fn new(grid: &Grid, snake: &Snake, at: &Point) -> RenderCell {
        if snake.occupies(at) {
            return RenderCell::Snake;
        }
        match grid.cell(at) {
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
        BG_BRIGHT_BLACK, BG_GREEN, BG_RED, Color, FG_BRIGHT_BLACK, FG_DIM, FG_GREEN, FG_RED, RESET,
        RenderCell, RenderState, Renderer,
    };
    use crate::grid::{Grid, GridCell, Point};
    use crate::snake::Snake;

    fn point(x: i32, y: i32) -> Point {
        Point::new(x, y)
    }

    #[test]
    fn render_state_new_keeps_grid_snake_and_score() {
        let grid = Grid::new(5, 5);
        let snake = Snake::new(Point::new(2, 2));
        let render_state = RenderState::new(&grid, &snake, 7);

        assert_eq!(render_state.grid().width(), 5);
        assert!(render_state.snake().occupies(&point(2, 2)));
        assert_eq!(render_state.score(), 7);
    }

    #[test]
    fn render_header_writes_dimmed_score_line() {
        let renderer = Renderer::new();
        let grid = Grid::new(5, 5);
        let snake = Snake::new(Point::new(2, 2));
        let render_state = RenderState::new(&grid, &snake, 3);
        let mut out = Vec::new();

        renderer.render_header(&mut out, &render_state);

        assert_eq!(
            String::from_utf8(out).expect("header should be utf-8"),
            format!("{FG_DIM}Score{RESET} {FG_GREEN}3{RESET}\r\n")
        );
    }

    #[test]
    fn render_grid_writes_mixed_cells_and_row_breaks() {
        let renderer = Renderer::new();
        let mut grid = Grid::new(5, 5);
        let snake = Snake::new(Point::new(2, 2));
        grid.change_cell(&point(2, 3), GridCell::Food);
        grid.change_cell(&point(3, 3), GridCell::Food);
        let render_state = RenderState::new(&grid, &snake, 0);
        let mut out = Vec::new();

        renderer.render_grid(&mut out, &render_state);

        let output = String::from_utf8(out).expect("grid should be utf-8");

        assert!(output.contains(" "));
        assert!(output.contains("█"));
        assert!(output.contains("▀"));
        assert!(output.contains("▄"));
        assert!(output.contains("\r\n"));
    }

    #[test]
    fn render_writes_clear_sequence_header_and_grid() {
        let renderer = Renderer::new();
        let mut grid = Grid::new(5, 5);
        let snake = Snake::new(Point::new(2, 2));
        grid.change_cell(&point(2, 3), GridCell::Food);
        let render_state = RenderState::new(&grid, &snake, 1);
        let mut out = Vec::new();

        renderer.render_to(&mut out, render_state);

        let output = String::from_utf8(out).expect("render should be utf-8");

        assert!(output.starts_with("\x1B[2J\x1B[1;1H"));
        assert!(output.contains(&format!("{FG_DIM}Score{RESET} {FG_GREEN}1{RESET}\r\n")));
        assert!(output.contains("█"));
    }

    #[test]
    fn render_cell_prefers_snake_over_grid_contents() {
        let mut grid = Grid::new(5, 5);
        grid.change_cell(&point(2, 2), GridCell::Food);
        let snake = Snake::new(Point::new(2, 2));

        assert!(matches!(
            RenderCell::new(&grid, &snake, &point(2, 2)),
            RenderCell::Snake
        ));
    }

    #[test]
    fn render_cell_reads_food_wall_and_empty_from_grid() {
        let mut grid = Grid::new(5, 5);
        let snake = Snake::new(Point::new(1, 1));
        grid.change_cell(&point(2, 2), GridCell::Food);

        assert!(matches!(
            RenderCell::new(&grid, &snake, &point(2, 2)),
            RenderCell::Food
        ));
        assert!(matches!(
            RenderCell::new(&grid, &snake, &point(0, 0)),
            RenderCell::Wall
        ));
        assert!(matches!(
            RenderCell::new(&grid, &snake, &point(3, 3)),
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
