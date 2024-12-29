use crate::utils::{shutdown, startup};
use anyhow::Result;
use crossterm::event::{self, Event};
use nix::unistd::Uid;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};
use std::io::stdout;

pub fn check_root() -> bool {
    Uid::effective().is_root()
}

pub fn show_root_warning() -> Result<()> {
    startup()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let block = Block::default()
                .title(" Warning ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(203, 153, 148)));

            let text = vec![
                Line::from("Running as root is not supported"),
                Line::from(""),
                Line::from("Press any key to exit"),
            ];

            let warning = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Rgb(203, 153, 148)));

            let popup_area = centered_rect(60, 20, area);
            f.render_widget(Clear, popup_area);
            f.render_widget(warning, popup_area);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }

    shutdown()?;
    Ok(())
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
