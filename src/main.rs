use std::io::stdout;

use auth::auth_core::app::App;
use auth::input::event;
#[cfg(unix)]
use auth::input::root;
use auth::ui::renderer::draw;
use auth::utils::cli;
use auth::utils::error::AuthResult;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() -> AuthResult<()> {
    let args = cli::parse_args();

    #[cfg(unix)]
    if !args.no_root_check && root::check_root() {
        root::show_root_warning()?;
        return Ok(());
    }

    if args.mouse {
        execute!(stdout(), EnableMouseCapture)?;
    }

    enable_raw_mode()?;
    let mut terminal = ratatui::init();
    let mut app = App::new()?;

    while !app.should_quit {
        terminal.draw(|f| draw(f, &app, args.no_size_check))?;

        if let Some(event) = event::poll_event()? {
            app.handle_events(&event)?;
        }
    }

    if args.mouse {
        execute!(stdout(), DisableMouseCapture)?;
    }
    disable_raw_mode()?;

    ratatui::restore();
    Ok(())
}
