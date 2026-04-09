use crate::grid::{Grid, GridCell, Point};
use crate::snake::Snake;
use std::io::{Write, stdout};

pub struct Renderer {
    preallocated_work_frame: Option<Frame>,
    displayed_frame: Option<Frame>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            preallocated_work_frame: None,
            displayed_frame: None,
        }
    }
    fn clear<W: Write>(&self, out: &mut W) {
        write!(out, "\x1B[2J").expect("could not clear screen");
    }
    pub fn render(&mut self, grid: &Grid, snake: &Snake, score: usize) {
        let mut out = stdout();
        self.render_to(&mut out, grid, snake, score);
    }
    fn render_to<W: Write>(&mut self, out: &mut W, grid: &Grid, snake: &Snake, score: usize) {
        if self.displayed_frame.is_none() {
            self.clear(out)
        }
        self.render_header(out, score);
        self.render_grid(out, snake, grid);
        let footer_row = 2 + ((grid.height() + 1) / 2) as usize;
        self.move_cursor(out, footer_row, 1);
        out.flush().expect("could not flush stdout");
    }
    fn render_header<W: Write>(&self, out: &mut W, score: usize) {
        self.move_cursor(out, 1, 1);
        write!(out, "{FG_DIM}Score{RESET} {FG_GREEN}{}{RESET}", score)
            .expect("could not write header");
    }
    fn render_grid<W: Write>(&mut self, out: &mut W, snake: &Snake, grid: &Grid) {
        if self.preallocated_work_frame.is_none() {
            self.preallocated_work_frame = Option::from(Frame::new(
                grid.width() as usize,
                ((grid.height() + 1) / 2) as usize,
            ))
        }
        let mut frame = self.preallocated_work_frame.take().unwrap();
        let mut y = 0;
        while y < grid.height() {
            let term_y = (y / 2) as usize;
            for x in 0..grid.width() {
                let top_point = Point::new(x, y);
                let bottom_point = Point::new(x, y + 1);
                let top = RenderCell::new(grid, snake, &top_point);
                let bottom = if y + 1 < grid.height() {
                    RenderCell::new(grid, snake, &bottom_point)
                } else {
                    RenderCell::Empty
                };
                frame.set(x as usize, term_y, TerminalCell::new(top, bottom));
                if match self.displayed_frame.as_ref() {
                    None => true,
                    Some(prev_frame) => {
                        prev_frame.get(x as usize, term_y) != frame.get(x as usize, term_y)
                    }
                } {
                    let row = 2 + term_y;
                    let col = 1 + x as usize;
                    self.move_cursor(out, row, col);
                    self.render_cell(out, frame.get(x as usize, term_y));
                }
            }
            y += 2;
        }
        self.preallocated_work_frame = self.displayed_frame.replace(frame);
    }
    fn render_cell<W: Write>(&self, out: &mut W, terminal_cell: &TerminalCell) {
        match (
            terminal_cell.top.to_color(),
            terminal_cell.bottom.to_color(),
        ) {
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
    fn move_cursor<W: Write>(&self, out: &mut W, row: usize, col: usize) {
        write!(out, "\x1B[{};{}H", row, col).expect("could not move cursor");
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

struct Frame {
    width: usize,
    cells: Vec<TerminalCell>,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Frame {
        Frame {
            width,
            cells: vec![TerminalCell::empty(); width * height],
        }
    }
    fn index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
    pub fn get(&self, x: usize, y: usize) -> &TerminalCell {
        &self.cells[self.index(x, y)]
    }
    pub fn set(&mut self, x: usize, y: usize, cell: TerminalCell) {
        let idx = self.index(x, y);
        self.cells[idx] = cell;
    }
}
#[derive(Clone, Eq, PartialEq)]
struct TerminalCell {
    top: RenderCell,
    bottom: RenderCell,
}

impl TerminalCell {
    pub fn new(top: RenderCell, bottom: RenderCell) -> TerminalCell {
        TerminalCell { top, bottom }
    }
    pub fn empty() -> TerminalCell {
        TerminalCell {
            top: RenderCell::Empty,
            bottom: RenderCell::Empty,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
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
        if let Some(cell) = grid.cell(at) {
            return match cell {
                GridCell::Wall => RenderCell::Wall,
                GridCell::Food => RenderCell::Food,
                _ => RenderCell::Empty,
            };
        }
        RenderCell::Empty
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
        RenderCell, Renderer,
    };
    use crate::grid::{Grid, GridCell, Point};
    use crate::snake::Snake;

    fn point(x: i32, y: i32) -> Point {
        Point::new(x, y)
    }

    #[test]
    fn render_accepts_grid_snake_and_score() {
        let mut renderer = Renderer::new();
        let grid = Grid::new(5, 5);
        let snake = Snake::new(Point::new(2, 2), &grid).expect("snake should fit in grid");
        let mut out = Vec::new();

        renderer.render_to(&mut out, &grid, &snake, 7);

        let output = String::from_utf8(out).expect("render should be utf-8");

        assert!(output.contains(&format!("{FG_DIM}Score{RESET} {FG_GREEN}7{RESET}")));
        assert!(output.contains("\x1B[2;1H"));
    }

    #[test]
    fn render_header_writes_dimmed_score_line() {
        let renderer = Renderer::new();
        let mut out = Vec::new();

        renderer.render_header(&mut out, 3);

        assert_eq!(
            String::from_utf8(out).expect("header should be utf-8"),
            format!("\x1B[1;1H{FG_DIM}Score{RESET} {FG_GREEN}3{RESET}")
        );
    }

    #[test]
    fn render_grid_writes_mixed_cells_with_cursor_moves() {
        let mut renderer = Renderer::new();
        let mut grid = Grid::new(5, 5);
        let snake = Snake::new(Point::new(2, 2), &grid).expect("snake should fit in grid");
        grid.change_cell(&point(2, 3), GridCell::Food);
        grid.change_cell(&point(3, 3), GridCell::Food);
        let mut out = Vec::new();

        renderer.render_grid(&mut out, &snake, &grid);

        let output = String::from_utf8(out).expect("grid should be utf-8");

        assert!(output.contains(" "));
        assert!(output.contains("█"));
        assert!(output.contains("▀"));
        assert!(output.contains("▄"));
        assert!(output.contains("\x1B[2;1H"));
        assert!(output.contains("\x1B[3;3H"));
        assert!(!output.contains("\r\n"));
    }

    #[test]
    fn render_writes_clear_sequence_header_and_grid() {
        let mut renderer = Renderer::new();
        let mut grid = Grid::new(5, 5);
        let snake = Snake::new(Point::new(2, 2), &grid).expect("snake should fit in grid");
        grid.change_cell(&point(2, 3), GridCell::Food);
        let mut out = Vec::new();

        renderer.render_to(&mut out, &grid, &snake, 1);

        let output = String::from_utf8(out).expect("render should be utf-8");

        assert!(output.starts_with("\x1B[2J\x1B[1;1H"));
        assert_eq!(output.matches("\x1B[2J").count(), 1);
        assert!(output.contains(&format!("{FG_DIM}Score{RESET} {FG_GREEN}1{RESET}")));
        assert!(output.contains("\x1B[2;1H"));
        assert!(output.ends_with("\x1B[5;1H"));
        assert!(output.contains("█"));
    }

    #[test]
    fn second_render_with_same_state_updates_only_header_and_footer_cursor() {
        let mut renderer = Renderer::new();
        let grid = Grid::new(5, 5);
        let snake = Snake::new(Point::new(2, 2), &grid).expect("snake should fit in grid");
        let mut first_out = Vec::new();
        let mut second_out = Vec::new();

        renderer.render_to(&mut first_out, &grid, &snake, 1);
        renderer.render_to(&mut second_out, &grid, &snake, 1);

        let output = String::from_utf8(second_out).expect("render should be utf-8");

        assert!(!output.contains("\x1B[2J"));
        assert_eq!(
            output,
            format!("\x1B[1;1H{FG_DIM}Score{RESET} {FG_GREEN}1{RESET}\x1B[5;1H")
        );
    }

    #[test]
    fn second_render_with_changed_state_updates_only_changed_cells() {
        let mut renderer = Renderer::new();
        let grid = Grid::new(5, 5);
        let first_snake = Snake::new(Point::new(2, 2), &grid).expect("snake should fit in grid");
        let second_snake = Snake::new(Point::new(3, 2), &grid).expect("snake should fit in grid");
        let mut first_out = Vec::new();
        let mut second_out = Vec::new();

        renderer.render_to(&mut first_out, &grid, &first_snake, 0);
        renderer.render_to(&mut second_out, &grid, &second_snake, 0);

        let output = String::from_utf8(second_out).expect("render should be utf-8");

        assert!(!output.contains("\x1B[2J"));
        assert_eq!(
            output,
            format!(
                "\x1B[1;1H{FG_DIM}Score{RESET} {FG_GREEN}0{RESET}\x1B[3;3H \x1B[3;4H{FG_GREEN}▀{RESET}\x1B[5;1H"
            )
        );
    }

    #[test]
    fn render_cell_prefers_snake_over_grid_contents() {
        let mut grid = Grid::new(5, 5);
        grid.change_cell(&point(2, 2), GridCell::Food);
        let snake = Snake::new(Point::new(2, 2), &grid).expect("snake should fit in grid");

        assert!(matches!(
            RenderCell::new(&grid, &snake, &point(2, 2)),
            RenderCell::Snake
        ));
    }

    #[test]
    fn render_cell_reads_food_wall_and_empty_from_grid() {
        let mut grid = Grid::new(5, 5);
        let snake = Snake::new(Point::new(1, 1), &grid).expect("snake should fit in grid");
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
