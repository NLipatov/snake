use crossterm::event;
use crossterm::event::KeyEventKind::Press;
use crossterm::event::{Event, KeyCode};
use std::time::Duration;

pub enum TerminalCommand {
    Escape,
    Space,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct Terminal {}

impl Terminal {
    pub fn wait_for_command_sync(&self) -> Option<TerminalCommand> {
        if let Some(key) = self.read_key_sync() {
            return self.key_to_command(key);
        }
        None
    }
    pub fn wait_for_command_async(&self) -> Option<TerminalCommand> {
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
    fn key_to_command(&self, key_code: KeyCode) -> Option<TerminalCommand> {
        match key_code {
            KeyCode::Up => Some(TerminalCommand::Up),
            KeyCode::Down => Some(TerminalCommand::Down),
            KeyCode::Left => Some(TerminalCommand::Left),
            KeyCode::Right => Some(TerminalCommand::Right),
            KeyCode::Esc => Some(TerminalCommand::Escape),
            KeyCode::Char(' ') => Some(TerminalCommand::Space),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Terminal, TerminalCommand};
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
            Some(TerminalCommand::Up)
        ));
        assert!(matches!(
            terminal.key_to_command(KeyCode::Down),
            Some(TerminalCommand::Down)
        ));
        assert!(matches!(
            terminal.key_to_command(KeyCode::Left),
            Some(TerminalCommand::Left)
        ));
        assert!(matches!(
            terminal.key_to_command(KeyCode::Right),
            Some(TerminalCommand::Right)
        ));
        assert!(matches!(
            terminal.key_to_command(KeyCode::Esc),
            Some(TerminalCommand::Escape)
        ));
        assert!(matches!(
            terminal.key_to_command(KeyCode::Char(' ')),
            Some(TerminalCommand::Space)
        ));
    }

    #[test]
    fn key_to_command_ignores_unhandled_keys() {
        let terminal = Terminal::default();

        assert!(terminal.key_to_command(KeyCode::Enter).is_none());
    }
}
