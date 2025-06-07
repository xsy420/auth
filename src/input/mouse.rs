use ratatui::crossterm::event::{MouseEvent, MouseEventKind};

use crate::AuthResult;
use crate::auth_core::app::{App, InputMode};

pub fn handle_mouse_event(app: &mut App, event: MouseEvent) -> AuthResult<()> {
    match app.input_mode {
        InputMode::Adding
        | InputMode::Importing
        | InputMode::Exporting
        | InputMode::Editing
        | InputMode::FileBrowser => {
            return Ok(());
        }
        InputMode::Normal => {}
    }

    match event.kind {
        MouseEventKind::Down(_) => handle_mouse_click(app),
        MouseEventKind::Moved => handle_mouse_hover(app, event.row as usize),
        _ => Ok(()),
    }
}

fn handle_mouse_click(app: &mut App) -> AuthResult<()> {
    app.copy_current_code()
}

fn handle_mouse_hover(app: &mut App, row: usize) -> AuthResult<()> {
    if row >= 1 && row < app.entries.len() + 1 {
        app.selected = row - 1;
    }
    Ok(())
}
