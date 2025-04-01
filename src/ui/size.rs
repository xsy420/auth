use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::ui::layout::pad_vertical;

const SIZE_WARNING: &[&str] = &[
    "Terminal size too small:",
    "Width = {} Height = {}",
    "",
    "Needed to display properly:",
    "Width = {} Height = {}",
];

pub fn check_terminal_size(frame: &mut Frame, area: Rect) -> bool {
    if !is_terminal_too_small(area) {
        return false;
    }

    render_size_warning(frame, area);
    true
}

fn is_terminal_too_small(area: Rect) -> bool {
    area.width < 110 || area.height < 31
}

fn create_warning_text(area: Rect) -> Vec<Line<'static>> {
    SIZE_WARNING
        .iter()
        .map(|&s| format_warning_line(s, area.width))
        .collect()
}

fn format_warning_line(text: &str, width: u16) -> Line<'static> {
    let text = match text.contains("{}") {
        true => text.replace("{}", &width.to_string()),
        false => text.to_string(),
    };
    Line::from(text)
}

fn render_size_warning(frame: &mut Frame, area: Rect) {
    let text = create_warning_text(area);
    let padded_text = pad_vertical(text, area.height);

    let warning = Paragraph::new(padded_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::LightCyan));

    frame.render_widget(warning, area);
}
