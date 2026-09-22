use crate::domain::game::Game;
use crate::domain::game::GameState::Paused;
use crate::domain::grid::{Grid, GridCell, Point};
use std::io::{Write, stdout};

// Used to convert 0-based frame columns to 1-based terminal columns.
// The top-left terminal cell is (1, 1); The top-left frame cell is (0, 0)
const X_OFFSET: usize = 1;
const Y_OFFSET: usize = 1;
const HEADER_SIZE: usize = 1;
// SCALE shows how many rows are displayed per terminal row.
// Each terminal row contains to halves - top and bottom.
const SCALE: i32 = 2;

#[derive(Default)]
pub struct Renderer {
    work_frame: Option<Frame>,
    displayed_frame: Option<Frame>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            work_frame: None,
            displayed_frame: None,
        }
    }
    fn clear<W: Write>(&self, out: &mut W) {
        write!(out, "\x1B[2J").expect("could not clear screen");
    }
    pub fn render(&mut self, game: &Game, score: usize) {
        let mut out = stdout();
        self.render_to(&mut out, game, score);
    }
    fn render_to<W: Write>(&mut self, out: &mut W, game: &Game, score: usize) {
        if self.geometry_changed(game.grid()) || self.displayed_frame.is_none() {
            self.clear(out)
        }
        self.render_header(out, score);
        self.render_grid(out, game);
        self.render_state(out, game);
        let footer_row = Y_OFFSET + HEADER_SIZE + Self::effective_frame_height(game.grid());
        self.move_cursor(out, footer_row, X_OFFSET);
        out.flush().expect("could not flush stdout");
    }
    fn render_state<W: Write>(&mut self, out: &mut W, game: &Game) {
        if game.state() == &Paused {
            let label = "Paused";
            // is there a space to put a label?
            if (game.grid().width() as usize) < label.len() {
                return;
            }
            let y = Y_OFFSET + HEADER_SIZE + (Self::effective_frame_height(game.grid()) - 1) / 2;
            let x = X_OFFSET + (game.grid().width() as usize - label.len()) / 2;
            self.move_cursor(out, y, x);
            self.render_text(out, FG_WHITE, BG_BRIGHT_BLACK, label);
            if let Some(frame) = self.displayed_frame.as_mut() {
                for i in 0..label.len() {
                    frame.set(
                        x - X_OFFSET + i,
                        y - Y_OFFSET - HEADER_SIZE,
                        TerminalCell::new(RenderCell::Text, RenderCell::Text),
                    );
                }
            }
        }
    }
    fn render_header<W: Write>(&self, out: &mut W, score: usize) {
        // Terminal coordinates are 1-based; the top-left cell is (1, 1)
        self.move_cursor(out, Y_OFFSET, X_OFFSET);
        write!(out, "{FG_DIM}Score{RESET} {FG_GREEN}{}{RESET}", score)
            .expect("could not write header");
    }
    fn render_grid<W: Write>(&mut self, out: &mut W, game: &Game) {
        let grid = game.grid();
        let mut frame = self.prepare_work_frame(grid);
        // each row contains two halves - top and bottom
        for y in (0..grid.height()).step_by(SCALE as usize) {
            let term_y = (y / SCALE) as usize;
            for x in 0..grid.width() {
                let top = RenderCell::new(grid, game, &Point::new(x, y));
                let bottom = if y + 1 < grid.height() {
                    RenderCell::new(grid, game, &Point::new(x, y + 1))
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
                    let row = Y_OFFSET + HEADER_SIZE + term_y;
                    let col = X_OFFSET + x as usize;
                    self.move_cursor(out, row, col);
                    self.render_cell(out, frame.get(x as usize, term_y));
                }
            }
        }
        self.work_frame = self.displayed_frame.replace(frame);
    }
    fn geometry_changed(&self, grid: &Grid) -> bool {
        self.displayed_frame.as_ref().is_some_and(|f| {
            !f.has_dimensions(grid.width() as usize, Self::effective_frame_height(grid))
        })
    }
    fn prepare_work_frame(&mut self, grid: &Grid) -> Frame {
        let geometry_changed = self.geometry_changed(grid);
        if geometry_changed {
            self.displayed_frame = None;
        }
        if self.work_frame.is_none() || geometry_changed {
            self.work_frame = Option::from(Frame::new(
                grid.width() as usize,
                Self::effective_frame_height(grid),
            ))
        }
        self.work_frame.take().unwrap()
    }
    fn effective_frame_height(grid: &Grid) -> usize {
        // frame row is splitted to 2 halves, which effectively make it 2 rows in a row.
        ((grid.height() + SCALE - 1) / SCALE) as usize
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
    fn render_text<W: Write>(&self, out: &mut W, fg: &str, bg: &str, text: &str) {
        write!(out, "{bg}{fg}{text}{RESET}").expect("could not write text")
    }
    fn move_cursor<W: Write>(&self, out: &mut W, row: usize, col: usize) {
        write!(out, "\x1B[{};{}H", row, col).expect("could not move cursor");
    }
}

const FG_WHITE: &str = "\x1b[37m";
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
    height: usize,
    cells: Vec<TerminalCell>,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Frame {
        Frame {
            width,
            height,
            cells: vec![TerminalCell::empty(); width * height],
        }
    }
    fn index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
    pub fn has_dimensions(&self, width: usize, height: usize) -> bool {
        self.height == height && self.width == width
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
    Text,
}

impl RenderCell {
    fn new(grid: &Grid, game: &Game, at: &Point) -> RenderCell {
        if game.snake_at(at) {
            return RenderCell::Snake;
        }
        if game.food_at(at) {
            return RenderCell::Food;
        }
        match grid.cell(at) {
            GridCell::Wall => RenderCell::Wall,
            GridCell::Empty => RenderCell::Empty,
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
            RenderCell::Text => Some(Color {
                fg: FG_WHITE,
                bg: BG_BRIGHT_BLACK,
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
    use crate::domain::game::Game;
    use crate::domain::grid::{Grid, Point};
    use crate::domain::grid_geometry::GridGeometry;
    use crate::domain::snake::Snake;

    fn point(x: i32, y: i32) -> Point {
        Point::new(x, y)
    }

    fn game_at(start: Point) -> Game {
        game_with_geometry(5, 5, start)
    }

    fn game_with_geometry(width: i32, height: i32, start: Point) -> Game {
        let geometry = GridGeometry::new(width, height);
        let grid = Grid::new(geometry);
        let snake = Snake::new(start, geometry).expect("snake should fit in grid");
        Game::new(grid, snake, 0)
    }

    #[test]
    fn render_accepts_grid_snake_and_score() {
        let mut renderer = Renderer::new();
        let game = game_at(Point::new(2, 2));
        let mut out = Vec::new();

        renderer.render_to(&mut out, &game, 7);

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
        let game = game_with_geometry(5, 6, Point::new(2, 2));
        let mut out = Vec::new();

        renderer.render_grid(&mut out, &game);

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
        let game = game_at(Point::new(2, 2));
        let mut out = Vec::new();

        renderer.render_to(&mut out, &game, 1);

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
        let game = game_at(Point::new(2, 2));
        let mut first_out = Vec::new();
        let mut second_out = Vec::new();

        renderer.render_to(&mut first_out, &game, 1);
        renderer.render_to(&mut second_out, &game, 1);

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
        let first_game = game_at(Point::new(2, 2));
        let second_game = game_at(Point::new(3, 2));
        let mut first_out = Vec::new();
        let mut second_out = Vec::new();

        renderer.render_to(&mut first_out, &first_game, 0);
        renderer.render_to(&mut second_out, &second_game, 0);

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
    fn paused_label_is_centered_and_its_cells_are_restored_after_resuming() {
        use crate::domain::game::GameCommand;

        let mut renderer = Renderer::new();
        let mut game = game_with_geometry(8, 8, point(3, 2));
        let mut out = Vec::new();
        renderer.render_to(&mut out, &game, 0);

        game.apply_command(GameCommand::TogglePause);
        out.clear();
        renderer.render_to(&mut out, &game, 0);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            format!(
                "\x1B[1;1H{FG_DIM}Score{RESET} {FG_GREEN}0{RESET}\x1B[3;2H{BG_BRIGHT_BLACK}{}Paused{RESET}\x1B[6;1H",
                super::FG_WHITE
            )
        );

        // Render twice while paused to exercise both reused frame buffers.
        renderer.render_to(&mut Vec::new(), &game, 0);
        game.apply_command(GameCommand::TogglePause);
        let mut out = Vec::new();
        renderer.render_to(&mut out, &game, 0);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            format!(
                "\x1B[1;1H{FG_DIM}Score{RESET} {FG_GREEN}0{RESET}\x1B[3;2H \x1B[3;3H \x1B[3;4H{FG_GREEN}▀{RESET}\x1B[3;5H \x1B[3;6H \x1B[3;7H \x1B[6;1H"
            )
        );

        let mut out = Vec::new();
        renderer.render_to(&mut out, &game, 0);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            format!("\x1B[1;1H{FG_DIM}Score{RESET} {FG_GREEN}0{RESET}\x1B[6;1H")
        );
    }

    #[test]
    fn paused_label_is_omitted_when_grid_is_too_narrow() {
        use crate::domain::game::GameCommand;

        let mut renderer = Renderer::new();
        let mut game = game_at(point(2, 2));
        let mut running = Vec::new();
        renderer.render_to(&mut running, &game, 0);

        game.apply_command(GameCommand::TogglePause);
        let mut paused = Vec::new();
        Renderer::new().render_to(&mut paused, &game, 0);

        assert_eq!(paused, running);
    }

    #[test]
    fn resizing_clears_and_redraws_the_entire_grid() {
        let mut renderer = Renderer::new();

        for (width, height) in [(5, 5), (8, 5), (8, 8), (5, 5)] {
            let game = game_with_geometry(width, height, point(2, 2));
            let mut resized = Vec::new();
            renderer.render_to(&mut resized, &game, 0);
            let mut fresh = Vec::new();
            Renderer::new().render_to(&mut fresh, &game, 0);
            assert_eq!(resized, fresh, "resizing to {width}x{height}");

            let mut unchanged = Vec::new();
            renderer.render_to(&mut unchanged, &game, 0);
            let output = String::from_utf8(unchanged).unwrap();
            assert!(!output.contains("\x1B[2J"));
            assert!(!output.contains('█'));
        }
    }

    #[test]
    fn mixed_cells_preserve_top_and_bottom_colors() {
        use super::TerminalCell;

        let renderer = Renderer::new();
        for (top, bottom, expected) in [
            (
                RenderCell::Food,
                RenderCell::Snake,
                format!("{FG_RED}{BG_GREEN}▀{RESET}"),
            ),
            (
                RenderCell::Snake,
                RenderCell::Food,
                format!("{FG_GREEN}{BG_RED}▀{RESET}"),
            ),
        ] {
            let mut out = Vec::new();
            renderer.render_cell(&mut out, &TerminalCell::new(top, bottom));
            assert_eq!(String::from_utf8(out).unwrap(), expected);
        }
    }

    #[test]
    fn render_cell_reads_snake_wall_and_empty_from_game_and_grid() {
        let game = game_at(Point::new(1, 1));
        let grid = game.grid();

        assert!(matches!(
            RenderCell::new(grid, &game, &point(1, 1)),
            RenderCell::Snake
        ));
        assert!(matches!(
            RenderCell::new(grid, &game, &point(0, 0)),
            RenderCell::Wall
        ));
        assert!(matches!(
            RenderCell::new(grid, &game, &point(3, 3)),
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
