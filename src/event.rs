use anyhow::Result;
use ratatui::crossterm::event::{self, Event};
use std::time::Duration;

pub fn poll_event() -> Result<Option<Event>> {
    if event::poll(Duration::from_millis(50))? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}
