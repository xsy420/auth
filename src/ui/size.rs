use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::ui::layout::pad_vertical;

const MIN_WIDTH: u16 = 110;
const MIN_HEIGHT: u16 = 30;

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
    area.width < MIN_WIDTH || area.height < MIN_HEIGHT
}

fn create_warning_text(area: Rect) -> Vec<Line<'static>> {
    SIZE_WARNING
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            if i == 4 {
                format_warning_line(s, MIN_WIDTH, MIN_HEIGHT)
            } else {
                format_warning_line(s, area.width, area.height)
            }
        })
        .collect()
}

fn format_warning_line(text: &str, width: u16, height: u16) -> Line<'static> {
    let text = if text.contains("{}") {
        text.replacen("{}", &width.to_string(), 1)
            .replacen("{}", &height.to_string(), 1)
    } else {
        text.to_string()
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
