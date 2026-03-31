use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct RawModeGuard {}

impl RawModeGuard {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RawModeGuard {
        enable_raw_mode().expect("could not enable raw mode");
        RawModeGuard {}
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}
