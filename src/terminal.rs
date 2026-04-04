use crate::game::Command;
use crossterm::event;
use crossterm::event::KeyEventKind::Press;
use crossterm::event::{Event, KeyCode};
use std::time::Duration;

#[derive(Default)]
pub struct Terminal {}

impl Terminal {
    pub fn wait_for_command_sync(&self) -> Option<Command> {
        if let Some(key) = self.read_key_sync() {
            return self.key_to_command(key);
        }
        None
    }
    pub fn wait_for_command_async(&self) -> Option<Command> {
        if let Some(key) = self.read_key_async() {
            return self.key_to_command(key);
        }
        None
    }
    fn read_key_async(&self) -> Option<KeyCode> {
        if event::poll(Duration::from_millis(0)).expect("could not poll event") {
            return self.key_code_from_event(event::read().expect("could not read key event"));
        }
        None
    }
    fn read_key_sync(&self) -> Option<KeyCode> {
        self.key_code_from_event(event::read().expect("could not read key event"))
    }
    fn key_code_from_event(&self, event: Event) -> Option<KeyCode> {
        match event {
            Event::Key(key) if key.kind == Press => Some(key.code),
            _ => None,
        }
    }
    fn key_to_command(&self, key_code: KeyCode) -> Option<Command> {
        match key_code {
            KeyCode::Up => Some(Command::SnakeMoveUp),
            KeyCode::Down => Some(Command::SnakeMoveDown),
            KeyCode::Left => Some(Command::SnakeMoveLeft),
            KeyCode::Right => Some(Command::SnakeMoveRight),
            KeyCode::Esc => Some(Command::Escape),
            KeyCode::Char(' ') => Some(Command::Pause),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Terminal;
    use crate::game::Command;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    #[test]
    fn key_code_from_event_reads_only_press_events() {
        let terminal = Terminal::default();
        let press_event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Up,
            KeyModifiers::NONE,
            KeyEventKind::Press,
        ));
        let repeat_event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Up,
            KeyModifiers::NONE,
            KeyEventKind::Repeat,
        ));
        let release_event = Event::Key(KeyEvent::new_with_kind(
            KeyCode::Up,
            KeyModifiers::NONE,
            KeyEventKind::Release,
        ));

        assert_eq!(terminal.key_code_from_event(press_event), Some(KeyCode::Up));
        assert_eq!(terminal.key_code_from_event(repeat_event), None);
        assert_eq!(terminal.key_code_from_event(release_event), None);
        assert_eq!(terminal.key_code_from_event(Event::Resize(80, 24)), None);
    }

    #[test]
    fn key_to_command_maps_arrow_keys_escape_and_pause() {
        let terminal = Terminal::default();

        assert!(matches!(
            terminal.key_to_command(KeyCode::Up),
            Some(Command::SnakeMoveUp)
        ));
        assert!(matches!(
            terminal.key_to_command(KeyCode::Down),
            Some(Command::SnakeMoveDown)
        ));
        assert!(matches!(
            terminal.key_to_command(KeyCode::Left),
            Some(Command::SnakeMoveLeft)
        ));
        assert!(matches!(
            terminal.key_to_command(KeyCode::Right),
            Some(Command::SnakeMoveRight)
        ));
        assert!(matches!(
            terminal.key_to_command(KeyCode::Esc),
            Some(Command::Escape)
        ));
        assert!(matches!(
            terminal.key_to_command(KeyCode::Char(' ')),
            Some(Command::Pause)
        ));
    }

    #[test]
    fn key_to_command_ignores_unhandled_keys() {
        let terminal = Terminal::default();

        assert!(terminal.key_to_command(KeyCode::Enter).is_none());
    }
}
