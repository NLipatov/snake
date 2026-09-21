use crate::domain::game::GameState::{self, GameOver, Paused};
use crate::domain::game::{Game, GameCommand};
use crate::domain::snake::Direction::{Down, Left, Right, Up};
use crate::infrastructure::raw_mode_guard::RawModeGuard;
use crate::infrastructure::terminal::{Terminal, TerminalCommand};
use crate::presentation::renderer::Renderer;
use std::time::Duration;

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
        self.renderer.render(&self.game, self.game.score());
        loop {
            if let Some(command) = match self.game.state() {
                GameState::Paused => self.terminal.wait_for_command_sync(),
                _ => self.terminal.wait_for_command_async(),
            } {
                match command {
                    TerminalCommand::Escape => {
                        return RunResult::Quit {
                            score: self.game.score(),
                        };
                    }
                    TerminalCommand::Down => self.game.apply_command(GameCommand::Move(Down)),
                    TerminalCommand::Up => self.game.apply_command(GameCommand::Move(Up)),
                    TerminalCommand::Left => self.game.apply_command(GameCommand::Move(Left)),
                    TerminalCommand::Right => self.game.apply_command(GameCommand::Move(Right)),
                    TerminalCommand::Space => self.game.apply_command(GameCommand::TogglePause),
                }
            }
            if let GameOver = self.game.tick() {
                break;
            }
            self.renderer.render(&self.game, self.game.score());
            if self.game.state() != &Paused {
                std::thread::sleep(Duration::from_millis(115));
            }
        }
        self.renderer.render(&self.game, self.game.score());
        RunResult::GameOver {
            score: self.game.score(),
        }
    }
}
