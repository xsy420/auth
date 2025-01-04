use crate::utils::{centered_rect, create_block, create_terminal, poll_event, shutdown, startup};
use anyhow::Result;
use crossterm::event::Event;
use nix::unistd::Uid;
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
        terminal.draw(|f| {
            let area = f.area();
            let block = create_block(" Warning ");
            let text = vec![
                Line::from("Running as root is not supported"),
                Line::from(""),
                Line::from("Press any key to exit"),
            ];

            let warning = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Red));

            let popup_area = centered_rect(60, 20, area);
            f.render_widget(Clear, popup_area);
            f.render_widget(warning, popup_area);
        })?;

        if let Some(Event::Key(_)) = poll_event()? {
            break;
        }
    }

    shutdown()?;
    Ok(())
}
