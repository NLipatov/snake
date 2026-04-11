use crate::domain::game::GameResult::GameOver;
use crate::domain::game::{Game, GameCommand};
use crate::domain::snake::Direction::{Down, Left, Right, Up};
use crate::infrastructure::raw_mode_guard::RawModeGuard;
use crate::infrastructure::terminal::{Terminal, TerminalCommand};
use crate::presentation::cli::PauseDecision::{Quit, Resume};
use crate::presentation::renderer::Renderer;
use std::time::Duration;

enum PauseDecision {
    Resume,
    Quit,
}

pub enum RunResult {
    GameOver { score: usize },
    Quit { score: usize },
}

pub struct Cli {
    game: Game,
    terminal: Terminal,
    renderer: Renderer,
}

impl Cli {
    pub fn new(game: Game, terminal: Terminal, renderer: Renderer) -> Cli {
        Cli {
            game,
            terminal,
            renderer,
        }
    }
    pub fn run_loop(&mut self) -> RunResult {
        let _rmg = RawModeGuard::new();
        self.renderer
            .render(self.game.grid(), self.game.snake(), self.game.score());
        loop {
            if let Some(command) = self.terminal.wait_for_command_async() {
                match command {
                    TerminalCommand::Escape => {
                        return RunResult::Quit {
                            score: self.game.score(),
                        };
                    }
                    TerminalCommand::Space => match self.pause_loop() {
                        Resume => continue,
                        Quit => {
                            return RunResult::Quit {
                                score: self.game.score(),
                            };
                        }
                    },
                    TerminalCommand::Down => self.game.apply_command(GameCommand::Move(Down)),
                    TerminalCommand::Up => self.game.apply_command(GameCommand::Move(Up)),
                    TerminalCommand::Left => self.game.apply_command(GameCommand::Move(Left)),
                    TerminalCommand::Right => self.game.apply_command(GameCommand::Move(Right)),
                }
            }
            if let GameOver = self.game.tick() {
                break;
            }
            self.renderer
                .render(self.game.grid(), self.game.snake(), self.game.score());
            std::thread::sleep(Duration::from_millis(115));
        }
        self.renderer
            .render(self.game.grid(), self.game.snake(), self.game.score());
        RunResult::GameOver {
            score: self.game.score(),
        }
    }
    fn pause_loop(&self) -> PauseDecision {
        loop {
            match self.terminal.wait_for_command_sync() {
                Some(TerminalCommand::Space) => return Resume,
                Some(TerminalCommand::Escape) => return Quit,
                _ => continue,
            }
        }
    }
}
