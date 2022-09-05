mod component;
mod tabs;
mod req;
mod app;
mod events;

use anyhow::Context;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, sync::{Arc, atomic::AtomicBool, mpsc}};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

fn main() -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app_data = app::App::default();

    let (mut tx, rx) = mpsc::channel();

    let terminated = Arc::new(AtomicBool::new(false));

    while !terminated.load(std::sync::atomic::Ordering::SeqCst) {
        let event = rx.recv().with_context(|| "Event channel close unexpectedly")?;
        match event {
            events::Events::KeyEvent(keycode) => handle_key(keycode, Arc::clone(&terminated), &mut app_data)?,
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_key(keycode: KeyCode, teminate: Arc<AtomicBool>, app: &mut app::App) -> anyhow::Result<()> {
    match keycode {
        KeyCode::Char('q') => teminate.store(true, std::sync::atomic::Ordering::SeqCst),
        _ => ()
    }

    Ok(())
}
