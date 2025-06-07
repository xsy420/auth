use ratatui::crossterm::event::{MouseEvent, MouseEventKind};

use crate::auth_core::app::{App, InputMode};

pub fn handle_mouse_event(app: &mut App, event: MouseEvent) {
    match app.input_mode {
        InputMode::Adding
        | InputMode::Importing
        | InputMode::Exporting
        | InputMode::Editing
        | InputMode::FileBrowser => {
            return;
        }
        InputMode::Normal => {}
    }

    match event.kind {
        MouseEventKind::Down(_) => handle_mouse_click(app),
        MouseEventKind::Moved => handle_mouse_hover(app, event.row as usize),
        _ => (),
    }
}

fn handle_mouse_click(app: &mut App) {
    app.copy_current_code();
}

fn handle_mouse_hover(app: &mut App, row: usize) {
    if row >= 1 && row < app.entries.len() + 1 {
        app.selected = row - 1;
    }
}
