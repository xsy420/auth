use anyhow::Result;
use auth::{cli, root, ui, App};
use ratatui::crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use std::io::stdout;

fn main() -> Result<()> {
    let args = cli::parse_args();

    if !args.no_root_check && root::check_root() {
        root::show_root_warning()?;
        return Ok(());
    }

    if args.mouse {
        execute!(stdout(), EnableMouseCapture)?;
    }

    let mut terminal = ratatui::init();
    let mut app = App::new()?;

    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &app, args.no_size_check))?;

        if let Some(event) = auth::utils::poll_event()? {
            app.handle_events(event)?;
        }
    }

    if args.mouse {
        execute!(stdout(), DisableMouseCapture)?;
    }

    ratatui::restore();
    Ok(())
}
