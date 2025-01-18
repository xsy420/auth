use crate::App;
use anyhow::Result;
use ratatui::crossterm::event::{MouseEvent, MouseEventKind};

pub fn handle_mouse_event(app: &mut App, event: MouseEvent) -> Result<()> {
    match event.kind {
        MouseEventKind::Down(_) => handle_mouse_click(app),
        MouseEventKind::Moved => handle_mouse_hover(app, event.row as usize),
        _ => Ok(()),
    }
}

fn handle_mouse_click(app: &mut App) -> Result<()> {
    app.copy_current_code()
}

fn handle_mouse_hover(app: &mut App, row: usize) -> Result<()> {
    if row >= 1 && row < app.entries.len() + 1 {
        app.selected = row - 1;
    }
    Ok(())
}
