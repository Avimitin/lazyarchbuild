use crossterm::event::KeyCode;

pub enum Events {
    KeyEvent(KeyCode),
}
