use crate::{
    app::{App, InputMode},
    constants::{
        ADD_ENTRY_TITLE, BINDINGS_TITLE, EDIT_ENTRY_TITLE, EXPORT_TITLE, HELP_TEXT, IMPORT_TITLE,
        NAME_LABEL, PATH_LABEL, SECRET_LABEL,
    },
    size::check_terminal_size,
    utils::{centered_rect, create_block, get_notification_title},
};
use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    if check_terminal_size(frame, area) {
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(area);

    draw_main_block(frame, app, chunks[0]);
    draw_help_block(frame, chunks[1]);
    draw_popups(frame, app, area);
}

fn draw_main_block(frame: &mut Frame, app: &App, area: Rect) {
    let title = get_notification_title(&app.error_message, app.copy_notification_time);
    let main_block = create_block(&title);

    let max_name_width = app.entries.iter().map(|e| e.name.len()).max().unwrap_or(0);

    let entries: Vec<Line> = app
        .entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let style = if i == app.selected {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };

            let (code, remaining) = entry.generate_totp_with_time();

            Line::styled(
                format!(
                    "{:<width$} {:>6} ({:>1}s)",
                    entry.name,
                    code,
                    remaining,
                    width = max_name_width + 2
                ),
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
    let help_block = create_block(BINDINGS_TITLE);
    let help_text = vec![Line::from(HELP_TEXT)];

    let help_widget = Paragraph::new(help_text)
        .block(help_block)
        .alignment(Alignment::Center);

    frame.render_widget(help_widget, area);
}

fn draw_popups(frame: &mut Frame, app: &App, area: Rect) {
    match app.input_mode {
        InputMode::Adding => draw_add_popup(frame, app, area),
        InputMode::Importing | InputMode::Exporting => draw_file_popup(frame, app, area),
        InputMode::Editing => draw_edit_popup(frame, app, area),
        _ => {}
    }
}

fn draw_add_popup(frame: &mut Frame, app: &App, area: Rect) {
    let popup_block = create_block(ADD_ENTRY_TITLE);
    let popup_area = centered_rect(60, 20, area);
    let cursor = if app.input_field == 0 { "|" } else { "" };
    let secret_cursor = if app.input_field == 1 { "|" } else { "" };

    let popup = Paragraph::new(vec![
        Line::from(NAME_LABEL),
        Line::from(format!("{}{}", app.new_entry_name.as_str(), cursor)),
        Line::from(""),
        Line::from(SECRET_LABEL),
        Line::from(format!(
            "{}{}",
            app.new_entry_secret.as_str(),
            secret_cursor
        )),
    ])
    .block(popup_block);

    render_popup(frame, popup, popup_area);
}

fn draw_file_popup(frame: &mut Frame, app: &App, area: Rect) {
    let title = if matches!(app.input_mode, InputMode::Importing) {
        IMPORT_TITLE
    } else {
        EXPORT_TITLE
    };

    let popup_block = create_block(title);
    let popup_area = centered_rect(60, 20, area);

    let popup = Paragraph::new(vec![
        Line::from(PATH_LABEL),
        Line::from(format!("{}|", app.path_input.as_str())),
    ])
    .block(popup_block);

    render_popup(frame, popup, popup_area);
}

fn draw_edit_popup(frame: &mut Frame, app: &App, area: Rect) {
    let popup_block = create_block(EDIT_ENTRY_TITLE);
    let popup_area = centered_rect(60, 20, area);
    let cursor = if app.input_field == 0 { "|" } else { "" };
    let secret_cursor = if app.input_field == 1 { "|" } else { "" };

    let popup = Paragraph::new(vec![
        Line::from(NAME_LABEL),
        Line::from(format!("{}{}", app.edit_entry_name.as_str(), cursor)),
        Line::from(""),
        Line::from(SECRET_LABEL),
        Line::from(format!(
            "{}{}",
            app.edit_entry_secret.as_str(),
            secret_cursor
        )),
    ])
    .block(popup_block);

    render_popup(frame, popup, popup_area);
}

fn render_popup(frame: &mut Frame, popup: Paragraph, area: Rect) {
    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}
