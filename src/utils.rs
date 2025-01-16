use crate::constants::{AUTH_TITLE, COPIED_MSG, TOTP_DIGITS, TOTP_PERIOD, TOTP_STEP};
use anyhow::Result;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Terminal,
};
use std::{
    io::stdout,
    process::Command,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use totp_rs::{Algorithm, TOTP};

pub trait CommandExt {
    fn process_input(&mut self, input: &[u8]) -> std::io::Result<std::process::Output>;
}

impl CommandExt for Command {
    fn process_input(&mut self, input: &[u8]) -> std::io::Result<std::process::Output> {
        let mut child = self.stdin(std::process::Stdio::piped()).spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            std::io::Write::write_all(&mut stdin, input)?;
        }
        child.wait_with_output()
    }
}

pub fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Ok(())
}

pub fn shutdown() -> Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn create_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    Ok(Terminal::new(CrosstermBackend::new(stdout()))?)
}

pub fn poll_event() -> Result<Option<Event>> {
    if event::poll(Duration::from_millis(50))? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}

pub fn create_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow))
}

pub fn pad_vertical(text: Vec<Line>, height: u16) -> Vec<Line> {
    let padding = (height.saturating_sub(text.len() as u16) / 2) as usize;
    let mut padded = vec![Line::from(""); padding];
    padded.extend(text);
    padded
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

pub fn generate_totp(secret: &str) -> Result<(String, u64)> {
    let mut secret = secret.replace(' ', "").to_uppercase();

    while secret.len() % 8 != 0 {
        secret.push('=');
    }

    let decoded = base32::decode(base32::Alphabet::Rfc4648 { padding: true }, &secret)
        .ok_or_else(|| anyhow::anyhow!("Invalid base32 secret"))?;

    let key = if decoded.len() < 16 {
        let mut padded = vec![0u8; 16];
        padded[..decoded.len()].copy_from_slice(&decoded);
        padded[decoded.len()..].fill(0);
        padded
    } else {
        decoded
    };

    let totp = TOTP::new(Algorithm::SHA1, TOTP_DIGITS, TOTP_STEP, TOTP_PERIOD, key)?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let remaining = TOTP_PERIOD - (now % TOTP_PERIOD);

    Ok((totp.generate_current()?, remaining))
}

pub fn copy_to_clipboard(text: String) -> Result<()> {
    thread::spawn(move || {
        if Command::new("wl-copy").arg(&text).output().is_err()
            && Command::new("xclip")
                .arg("-selection")
                .arg("clipboard")
                .arg("-in")
                .process_input(text.as_bytes())
                .is_err()
        {
            eprintln!("Failed to copy to clipboard: neither wl-copy nor xclip worked");
        }
    });
    Ok(())
}

pub fn get_notification_title(
    error_message: &Option<(String, SystemTime)>,
    copy_notification_time: Option<SystemTime>,
) -> String {
    if let Some((msg, time)) = error_message {
        if time.elapsed().unwrap_or_default().as_secs() < 3 {
            return format!(" {} ", msg);
        }
    }

    if let Some(notify_time) = copy_notification_time {
        if notify_time.elapsed().unwrap_or_default().as_secs() < 3 {
            return COPIED_MSG.to_string();
        }
    }

    AUTH_TITLE.to_string()
}
