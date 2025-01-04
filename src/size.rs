use crate::utils::{pad_vertical, MIN_HEIGHT, MIN_WIDTH};
use ratatui::{prelude::*, widgets::Paragraph};

pub fn check_terminal_size(frame: &mut Frame, area: Rect) -> bool {
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        let text = vec![
            Line::from("Terminal size too small:"),
            Line::from(format!("Width = {} Height = {}", area.width, area.height)),
            Line::from(""),
            Line::from("Needed to display properly:"),
            Line::from(format!("Width = {} Height = {}", MIN_WIDTH, MIN_HEIGHT)),
        ];

        let padded_text = pad_vertical(text, area.height);
        let warning = Paragraph::new(padded_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::LightCyan));

        frame.render_widget(warning, area);
        return true;
    }
    false
}
