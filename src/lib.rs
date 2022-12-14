mod app;
pub mod canvas;
mod component;
mod events;
mod req;
mod tabs;
mod types;

use anyhow::Context;
use crossterm::{
    event::KeyCode,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    sync::{atomic::AtomicBool, mpsc, Arc},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

macro_rules! is_running {
    ($stats:ident) => {
        $stats.load(std::sync::atomic::Ordering::SeqCst)
    };
}

pub fn setup_crossterm_terminal() -> anyhow::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub async fn run() -> anyhow::Result<()> {
    let mut terminal = setup_crossterm_terminal()?;
    let mut app_data = app::App::default();

    let (tx, rx) = mpsc::channel();

    spawn_terminal_event_sender(tx, Arc::clone(&app_data.is_running));

    let mut first_run = true;

    while app_data.is_running() {
        if first_run {
            canvas::draw_welcome_page(&mut terminal)?;
            app_data.update().unwrap();
            first_run = false;
        }

        if let Err(err) = render(&mut terminal, &mut app_data) {
            clean_up_terminal(&mut terminal)?;
            eprintln!("{err}");
        }

        let event = rx
            .recv()
            .with_context(|| "Event channel close unexpectedly")?;
        match event {
            events::Events::KeyEvent(keycode) => app_data.handle_input(keycode),
        }
    }

    clean_up_terminal(&mut terminal)?;
    Ok(())
}

/// Restore the terminal screen to blank screen
pub fn clean_up_terminal<B: Backend + std::io::Write>(
    terminal: &mut Terminal<B>,
) -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn render<B: Backend>(terminal: &mut Terminal<B>, app: &mut app::App) -> anyhow::Result<()> {
    terminal.draw(|frame| match app.current_display() {
        app::DisplayMode::ViewingPackageStatusTable => {
            canvas::draw_pkg_table_frame(frame, &mut app.pkg_info_table);
        }
        app::DisplayMode::PopUpPstMenu(items) => {
            canvas::draw_popup_menu_frame(frame, items);
        }
    })?;
    Ok(())
}

fn spawn_terminal_event_sender(tx: mpsc::Sender<events::Events>, app_stats: Arc<AtomicBool>) {
    std::thread::spawn(move || {
        while is_running!(app_stats) {
            let event = crossterm::event::read().unwrap();
            #[allow(clippy::single_match)]
            match event {
                crossterm::event::Event::Key(key) => {
                    tx.send(events::Events::KeyEvent(key.code)).unwrap()
                }
                _ => (),
            };
        }
    });
}
