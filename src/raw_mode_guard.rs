use crossterm::cursor::{Hide, Show};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::stdout;

pub struct RawModeGuard {}

impl RawModeGuard {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RawModeGuard {
        enable_raw_mode().expect("could not enable raw mode");
        if let Err(err) = execute!(stdout(), Hide) {
            let _ = disable_raw_mode();
            panic!("could not hide cursor: {err}");
        }
        RawModeGuard {}
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), Show);
    }
}
