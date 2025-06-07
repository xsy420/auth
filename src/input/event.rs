use std::time::Duration;

use ratatui::crossterm::event::{self, Event};

use crate::AuthResult;

/// # Errors
pub fn poll_event() -> AuthResult<Option<Event>> {
    if event::poll(Duration::from_millis(50))? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}
