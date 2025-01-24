use crate::{
    constants::{ROOT_WARNING, WARNING_TITLE},
    utils::{centered_rect, poll_event},
};
use anyhow::Result;
use nix::unistd::Uid;
use ratatui::{
    crossterm::event::Event,
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};

pub fn check_root() -> bool {
    Uid::effective().is_root()
}

pub fn show_root_warning() -> Result<()> {
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
            title: WARNING_TITLE,
            style: Style::default().fg(Color::Green),
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
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

fn render_warning(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    terminal.draw(|f| {
        let warning = WarningWidget::new(ROOT_WARNING);
        let popup_area = centered_rect(60, 20, f.area());

        f.render_widget(Clear, popup_area);
        f.render_widget(warning, popup_area);
    })?;
    Ok(())
}

fn should_exit() -> Result<bool> {
    Ok(matches!(poll_event()?, Some(Event::Key(_))))
}
