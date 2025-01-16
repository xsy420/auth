use crate::{
    constants::{ROOT_WARNING, WARNING_TITLE},
    utils::{centered_rect, create_block, create_terminal, poll_event, shutdown, startup},
};
use anyhow::Result;
use nix::unistd::Uid;
use ratatui::crossterm::event::Event;
use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

pub fn check_root() -> bool {
    Uid::effective().is_root()
}

pub fn show_root_warning() -> Result<()> {
    startup()?;
    let mut terminal = create_terminal()?;
    terminal.clear()?;

    loop {
        render_warning(&mut terminal)?;
        if should_exit()? {
            break;
        }
    }

    shutdown()?;
    Ok(())
}

fn render_warning(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    terminal.draw(|f| {
        let warning = create_warning_widget();
        let popup_area = centered_rect(60, 20, f.area());

        f.render_widget(Clear, popup_area);
        f.render_widget(warning, popup_area);
    })?;
    Ok(())
}

fn create_warning_widget() -> Paragraph<'static> {
    let block = create_block(WARNING_TITLE);
    let text = ROOT_WARNING
        .iter()
        .map(|&s| Line::from(s))
        .collect::<Vec<_>>();

    Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red))
}

fn should_exit() -> Result<bool> {
    Ok(matches!(poll_event()?, Some(Event::Key(_))))
}
