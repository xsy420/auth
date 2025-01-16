use crate::{constants::*, utils::pad_vertical};
use ratatui::{prelude::*, widgets::Paragraph};

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
        .map(|&s| format_warning_line(s, area.width))
        .collect()
}

fn format_warning_line(text: &str, width: u16) -> Line<'static> {
    if text.contains("{}") {
        Line::from(text.replace("{}", &width.to_string()))
    } else {
        Line::from(text.to_string())
    }
}

fn render_size_warning(frame: &mut Frame, area: Rect) {
    let text = create_warning_text(area);
    let padded_text = pad_vertical(text, area.height);

    let warning = Paragraph::new(padded_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::LightCyan));

    frame.render_widget(warning, area);
}
