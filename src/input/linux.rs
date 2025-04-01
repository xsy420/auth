use ratatui::crossterm::event::Event;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget};

use crate::AuthResult;
use crate::input::event::poll_event;
use crate::ui::layout::centered_rect;

const LINUX_WARNING: &[&str] = &["Only Linux is supported", "", "Press any key to exit"];

pub fn check_linux() -> bool {
    cfg!(target_os = "linux")
}

pub fn show_linux_warning() -> AuthResult<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    loop {
        render_warning(&mut terminal)?;
        if should_exit()? {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}

pub struct WarningWidget<'a> {
    text: &'a [&'a str],

    title: &'a str,

    style: Style,
}

impl<'a> WarningWidget<'a> {
    pub fn new(text: &'a [&'a str]) -> Self {
        Self {
            text,
            title: " Warning ",
            style: Style::default().fg(Color::Green),
        }
    }
}

impl Widget for WarningWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(self.style);

        let inner_area = block.inner(area);
        block.render(area, buf);

        let text = self.text.iter().map(|&s| Line::from(s)).collect::<Vec<_>>();

        Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White))
            .render(inner_area, buf);
    }
}

fn render_warning(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> AuthResult<()> {
    terminal.draw(|f| {
        let warning = WarningWidget::new(LINUX_WARNING);
        let popup_area = centered_rect(60, 20, f.area());

        f.render_widget(Clear, popup_area);
        f.render_widget(warning, popup_area);
    })?;
    Ok(())
}

fn should_exit() -> AuthResult<bool> {
    Ok(matches!(poll_event()?, Some(Event::Key(_))))
}
