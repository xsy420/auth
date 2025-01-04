use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::stdout;

pub mod app;
pub mod crypto;
pub mod entry;
pub mod root;
pub mod size;
pub mod ui;
pub mod utils;

pub use app::App;
pub use utils::{shutdown, startup};

pub fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new()?;

    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &app))?;
        if let Some(event) = utils::poll_event()? {
            app.handle_events(event)?;
        }
    }

    Ok(())
}
