use ratatui::prelude::*;
use ratatui::widgets::{Clear, Paragraph};

use crate::auth_core::app::{App, InputMode};
use crate::auth_core::entry::Entry;
use crate::ui::file_browser::draw_file_browser;
use crate::ui::layout::{centered_rect, create_block};
use crate::ui::notification::get_notification_title;
use crate::ui::size::check_terminal_size;

const HELP_TEXT: &str = "a: add  E: edit  d: del  i: import  e: export  ↑/k: up  ↓/j: down  enter: copy  q: quit  tab: cycle fields";

const FILE_BROWSER_HELP_TEXT: &str =
    "↑/k: up  ↓/j: down  enter: select  .: toggle hidden  q/esc: cancel";

const EXPORT_HELP_TEXT: &str =
    "↑/k: up  ↓/j: down  enter: select  .: toggle hidden  s: save  q/esc: cancel";

pub fn draw(frame: &mut Frame, app: &App, no_size_check: bool) {
    let area = frame.area();

    if !no_size_check && check_terminal_size(frame, area) {
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(area);

    draw_main_block(frame, app, chunks[0]);
    draw_help_block(frame, app, chunks[1]);
    draw_popups(frame, app, area);
}

fn draw_main_block(frame: &mut Frame, app: &App, area: Rect) {
    let title = get_notification_title(&app.error_message, app.copy_notification_time);
    let main_block = create_block(&title);
    let entries = create_entry_lines(app);

    let main_widget = Paragraph::new(entries)
        .block(main_block)
        .alignment(Alignment::Left);

    frame.render_widget(main_widget, area);
}

fn create_entry_lines(app: &App) -> Vec<Line> {
    if app.entries.is_empty() {
        return Vec::new();
    }

    let max_name_width = get_max_name_width(&app.entries);
    create_formatted_lines(&app.entries, app.selected, max_name_width)
}

fn get_max_name_width(entries: &[Entry]) -> usize {
    entries.iter().map(|e| e.name.len()).max().unwrap_or(0)
}

fn create_formatted_lines(entries: &[Entry], selected: usize, max_width: usize) -> Vec<Line> {
    entries
        .iter()
        .enumerate()
        .map(|(i, entry)| create_entry_line(i, entry, selected, max_width))
        .collect()
}

fn create_entry_line(index: usize, entry: &Entry, selected: usize, max_width: usize) -> Line {
    let style = get_line_style(index == selected);
    let formatted_text = format_entry_text(entry, max_width);
    Line::styled(formatted_text, style)
}

fn get_line_style(is_selected: bool) -> Style {
    let color = match is_selected {
        true => Color::Green,
        false => Color::Reset,
    };
    Style::default().fg(color)
}

fn format_entry_text(entry: &Entry, max_width: usize) -> String {
    let (code, remaining) = entry.generate_totp_with_time();
    format!(
        "{:<width$} {:>6} ({:>1}s)",
        entry.name,
        code,
        remaining,
        width = max_width + 2
    )
}

fn draw_help_block(frame: &mut Frame, app: &App, area: Rect) {
    let help_block = create_block(" Bindings ");
    let help_text = get_help_text(app);

    let help_widget = Paragraph::new(vec![help_text])
        .block(help_block)
        .alignment(Alignment::Center);

    frame.render_widget(help_widget, area);
}

fn get_help_text(app: &App) -> Line<'static> {
    match app.input_mode {
        InputMode::FileBrowser => {
            if app.file_operation == Some(InputMode::Exporting) {
                Line::from(EXPORT_HELP_TEXT)
            } else {
                Line::from(FILE_BROWSER_HELP_TEXT)
            }
        }
        _ => Line::from(HELP_TEXT),
    }
}

fn draw_popups(frame: &mut Frame, app: &App, area: Rect) {
    match app.input_mode {
        InputMode::Adding => draw_add_popup(frame, app, area),
        InputMode::Importing | InputMode::Exporting => draw_file_popup(frame, app, area),
        InputMode::Editing => draw_edit_popup(frame, app, area),
        InputMode::FileBrowser => draw_file_browser_popup(frame, app, area),
        _ => {}
    }
}

fn draw_add_popup(frame: &mut Frame, app: &App, area: Rect) {
    let popup = create_entry_popup(
        " Add Entry ",
        &app.new_entry_name,
        &app.new_entry_secret,
        app.input_field,
    );
    render_centered_popup(frame, popup, area);
}

fn draw_edit_popup(frame: &mut Frame, app: &App, area: Rect) {
    let popup = create_entry_popup(
        " Edit Entry ",
        &app.edit_entry_name,
        &app.edit_entry_secret,
        app.input_field,
    );
    render_centered_popup(frame, popup, area);
}

fn create_entry_popup<'a>(
    title: &'a str,
    name: &'a str,
    secret: &'a str,
    input_field: usize,
) -> Paragraph<'a> {
    let cursor = create_cursor_indicators(input_field);
    let lines = create_entry_popup_lines(name, secret, &cursor);

    Paragraph::new(lines).block(create_block(title))
}

fn create_cursor_indicators(input_field: usize) -> (String, String) {
    let name_cursor = if input_field == 0 { "|" } else { "" };
    let secret_cursor = if input_field == 1 { "|" } else { "" };
    (name_cursor.to_string(), secret_cursor.to_string())
}

fn create_entry_popup_lines<'a>(
    name: &'a str,
    secret: &'a str,
    (name_cursor, secret_cursor): &(String, String),
) -> Vec<Line<'a>> {
    vec![
        Line::from("Name:"),
        Line::from(format!("{}{}", name, name_cursor)),
        Line::from(""),
        Line::from("Secret:"),
        Line::from(format!("{}{}", secret, secret_cursor)),
    ]
}

fn draw_file_popup(frame: &mut Frame, app: &App, area: Rect) {
    let title = get_file_popup_title(&app.input_mode);
    let popup = create_file_popup(title, &app.path_input);
    render_centered_popup(frame, popup, area);
}

fn get_file_popup_title(input_mode: &InputMode) -> &'static str {
    match input_mode {
        InputMode::Importing => " Import ",
        InputMode::Exporting => " Export ",
        _ => unreachable!(),
    }
}

fn create_file_popup<'a>(title: &'a str, path: &'a str) -> Paragraph<'a> {
    let lines = vec![Line::from("Path:"), Line::from(format!("{}{}", path, "|"))];

    Paragraph::new(lines).block(create_block(title))
}

fn render_centered_popup(frame: &mut Frame, popup: Paragraph, area: Rect) {
    let popup_area = centered_rect(60, 20, area);
    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup, popup_area);
}

fn draw_file_browser_popup(frame: &mut Frame, app: &App, area: Rect) {
    let title = match app.file_operation {
        Some(InputMode::Importing) => " Select File to Import ",
        Some(InputMode::Exporting) => " Select Location to Export ",
        _ => " File Browser ",
    };

    draw_file_browser(frame, app, &app.file_browser, title, area);
}
