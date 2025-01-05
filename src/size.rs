use crate::{constants::*, utils::pad_vertical};
use ratatui::{prelude::*, widgets::Paragraph};

pub fn check_terminal_size(frame: &mut Frame, area: Rect) -> bool {
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        let text = SIZE_WARNING
            .iter()
            .map(|&s| {
                if s.contains("{}") {
                    Line::from(s.replace("{}", &format!("{}", area.width).to_string()))
                } else {
                    Line::from(s)
                }
            })
            .collect();

        let padded_text = pad_vertical(text, area.height);
        let warning = Paragraph::new(padded_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightCyan));

        frame.render_widget(warning, area);
        return true;
    }
    false
}
