use anyhow::Result;
use auth::{root, ui, App};

fn main() -> Result<()> {
    if root::check_root() {
        root::show_root_warning()?;
        return Ok(());
    }

    let mut terminal = ratatui::init();
    let mut app = App::new()?;

    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &app))?;

        if let Some(event) = auth::utils::poll_event()? {
            app.handle_events(event)?;
        }
    }

    ratatui::restore();
    Ok(())
}
