use anyhow::Result;
use auth::{root, ui, App};
use ratatui::crossterm::event::{self};
use std::time::Duration;

fn main() -> Result<()> {
    if root::check_root() {
        root::show_root_warning()?;
        return Ok(());
    }

    let mut terminal = ratatui::init();
    let mut app = App::new()?;

    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Ok(event) = event::read() {
                app.handle_events(event)?;
            }
        }
    }

    ratatui::restore();
    Ok(())
}
