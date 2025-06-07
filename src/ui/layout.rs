use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders};

#[must_use]
pub fn create_block(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Green))
}

#[must_use]
pub fn pad_vertical(text: Vec<Line>, height: u16) -> Vec<Line> {
    let padding = (height.saturating_sub(text.len() as u16) / 2) as usize;
    let mut padded = vec![Line::from(""); padding];
    padded.extend(text);
    padded
}

#[must_use]
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical_layout = create_vertical_layout(percent_y, r);
    create_horizontal_layout(percent_x, vertical_layout[1])
}

fn create_vertical_layout(percent: u16, area: Rect) -> Vec<Rect> {
    let margin = (100 - percent) / 2;
    let constraints = [
        Constraint::Percentage(margin),
        Constraint::Percentage(percent),
        Constraint::Percentage(margin),
    ];

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area)
        .to_vec()
}

fn create_horizontal_layout(percent: u16, area: Rect) -> Rect {
    let margin = (100 - percent) / 2;
    let constraints = [
        Constraint::Percentage(margin),
        Constraint::Percentage(percent),
        Constraint::Percentage(margin),
    ];

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area)[1]
}
