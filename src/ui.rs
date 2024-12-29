use crate::app::{App, InputMode};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(area);

    draw_main_block(frame, app, chunks[0]);
    draw_help_block(frame, chunks[1]);
    draw_popups(frame, app, area);
}

fn draw_main_block(frame: &mut Frame, app: &App, area: Rect) {
    let title = if let Some((msg, time)) = &app.error_message {
        if time.elapsed().unwrap_or_default().as_secs() < 3 {
            format!(" {} ", msg)
        } else {
            " Auth ".to_string()
        }
    } else if let Some(notify_time) = app.copy_notification_time {
        if notify_time.elapsed().unwrap_or_default().as_secs() < 3 {
            " Copied! ".to_string()
        } else {
            " Auth ".to_string()
        }
    } else {
        " Auth ".to_string()
    };

    let main_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(203, 153, 148)));

    let entries: Vec<Line> = app
        .entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let style = if i == app.selected {
                Style::default().fg(Color::Rgb(203, 153, 148))
            } else {
                Style::default()
            };

            let (code, remaining) = entry
                .generate_totp_with_time()
                .unwrap_or_else(|_| ("Invalid".to_string(), 0));

            Line::styled(
                format!("{:<30} {:>6} ({:>2}s)", entry.name, code, remaining),
                style,
            )
        })
        .collect();

    let main_widget = Paragraph::new(entries)
        .block(main_block)
        .alignment(Alignment::Left);

    frame.render_widget(main_widget, area);
}

fn draw_help_block(frame: &mut Frame, area: Rect) {
    let help_block = Block::default()
        .title(" Bindings ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(203, 153, 148)));

    let help_text = vec![Line::from(
        "a: add  E: edit  d: del  i: import  e: export  ↑/k: up  ↓/j: down  enter: copy  q: quit",
    )];

    let help_widget = Paragraph::new(help_text)
        .block(help_block)
        .alignment(Alignment::Center);

    frame.render_widget(help_widget, area);
}

fn draw_popups(frame: &mut Frame, app: &App, area: Rect) {
    match app.input_mode {
        InputMode::Adding => {
            let popup_block = Block::default()
                .title(" Add Entry ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(203, 153, 148)));

            let area = centered_rect(60, 20, area);
            let popup = Paragraph::new(vec![
                Line::from("Name:"),
                Line::from(format!(
                    "{}{}",
                    app.new_entry_name.as_str(),
                    if app.input_field == 0 { "|" } else { "" }
                )),
                Line::from(""),
                Line::from("Secret:"),
                Line::from(format!(
                    "{}{}",
                    app.new_entry_secret.as_str(),
                    if app.input_field == 1 { "|" } else { "" }
                )),
            ])
            .block(popup_block);

            frame.render_widget(Clear, area);
            frame.render_widget(popup, area);
        }
        InputMode::Importing | InputMode::Exporting => {
            let title = match app.input_mode {
                InputMode::Importing => " Import ",
                InputMode::Exporting => " Export ",
                _ => unreachable!(),
            };

            let popup_block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(203, 153, 148)));

            let area = centered_rect(60, 20, area);
            let popup = Paragraph::new(vec![
                Line::from("Path:"),
                Line::from(format!("{}|", app.path_input.as_str())),
            ])
            .block(popup_block);

            frame.render_widget(Clear, area);
            frame.render_widget(popup, area);
        }
        InputMode::Editing => {
            let popup_block = Block::default()
                .title(" Edit Entry ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(203, 153, 148)));

            let area = centered_rect(60, 20, area);
            let popup = Paragraph::new(vec![
                Line::from("Name:"),
                Line::from(format!(
                    "{}{}",
                    app.edit_entry_name.as_str(),
                    if app.input_field == 0 { "|" } else { "" }
                )),
                Line::from(""),
                Line::from("Secret:"),
                Line::from(format!(
                    "{}{}",
                    app.edit_entry_secret.as_str(),
                    if app.input_field == 1 { "|" } else { "" }
                )),
            ])
            .block(popup_block);

            frame.render_widget(Clear, area);
            frame.render_widget(popup, area);
        }
        _ => {}
    }
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
