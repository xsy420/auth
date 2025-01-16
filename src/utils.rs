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
    sync::mpsc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use totp_rs::{Algorithm, TOTP};

pub trait CommandExt {
    fn process_input(&mut self, input: &[u8]) -> std::io::Result<std::process::Output>;
    fn spawn_with_stdin(&mut self) -> std::io::Result<std::process::Child>;
    fn write_input_to_child(
        &self,
        child: &mut std::process::Child,
        input: &[u8],
    ) -> std::io::Result<()>;
    fn wait_for_child_output(
        &self,
        child: std::process::Child,
    ) -> std::io::Result<std::process::Output>;
}

impl CommandExt for Command {
    fn process_input(&mut self, input: &[u8]) -> std::io::Result<std::process::Output> {
        let mut child = self.spawn_with_stdin()?;
        self.write_input_to_child(&mut child, input)?;
        self.wait_for_child_output(child)
    }

    fn spawn_with_stdin(&mut self) -> std::io::Result<std::process::Child> {
        self.stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::Other, crate::constants::CLIPBOARD_ERROR)
            })
    }

    fn write_input_to_child(
        &self,
        child: &mut std::process::Child,
        input: &[u8],
    ) -> std::io::Result<()> {
        let stdin = match child.stdin.take() {
            Some(stdin) => stdin,
            None => return Ok(()),
        };

        let mut stdin = stdin;
        std::io::Write::write_all(&mut stdin, input).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, crate::constants::CLIPBOARD_ERROR)
        })
    }

    fn wait_for_child_output(
        &self,
        child: std::process::Child,
    ) -> std::io::Result<std::process::Output> {
        child.wait_with_output().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, crate::constants::CLIPBOARD_ERROR)
        })
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
    let vertical_layout = create_vertical_layout(percent_y, r);
    create_horizontal_layout(percent_x, vertical_layout[1])
}

fn create_vertical_layout(percent: u16, area: Rect) -> Vec<Rect> {
    let margin = (100 - percent) / 2;
    let constraints = [
        Constraint::Percentage(margin),
        Constraint::Percentage(percent),
        Constraint::Percentage(margin),
    ];

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area)
        .to_vec()
}

fn create_horizontal_layout(percent: u16, area: Rect) -> Rect {
    let margin = (100 - percent) / 2;
    let constraints = [
        Constraint::Percentage(margin),
        Constraint::Percentage(percent),
        Constraint::Percentage(margin),
    ];

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area)[1]
}

pub fn generate_totp(secret: &str) -> Result<(String, u64)> {
    let secret = normalize_secret(secret);
    let key = decode_and_pad_secret(&secret)?;
    let (code, remaining) = generate_totp_code(key)?;
    Ok((code, remaining))
}

fn normalize_secret(secret: &str) -> String {
    let mut secret = secret.replace(' ', "").to_uppercase();
    while secret.len() % 8 != 0 {
        secret.push('=');
    }
    secret
}

fn decode_and_pad_secret(secret: &str) -> Result<Vec<u8>> {
    let decoded = base32::decode(base32::Alphabet::Rfc4648 { padding: true }, secret)
        .ok_or_else(|| anyhow::anyhow!(crate::constants::INVALID_KEY_ERROR))?;

    Ok(pad_secret_if_needed(decoded))
}

fn pad_secret_if_needed(decoded: Vec<u8>) -> Vec<u8> {
    if decoded.len() < 16 {
        let mut padded = vec![0u8; 16];
        padded[..decoded.len()].copy_from_slice(&decoded);
        padded[decoded.len()..].fill(0);
        padded
    } else {
        decoded
    }
}

fn generate_totp_code(key: Vec<u8>) -> Result<(String, u64)> {
    let totp = create_totp(key)?;
    let remaining = calculate_remaining_time()?;
    let code = generate_code(&totp)?;

    Ok((code, remaining))
}

fn create_totp(key: Vec<u8>) -> Result<TOTP> {
    TOTP::new(Algorithm::SHA1, TOTP_DIGITS, TOTP_STEP, TOTP_PERIOD, key)
        .map_err(|_| anyhow::anyhow!(crate::constants::TOTP_ERROR))
}

fn calculate_remaining_time() -> Result<u64> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| anyhow::anyhow!(crate::constants::TOTP_ERROR))?
        .as_secs();
    Ok(TOTP_PERIOD - (now % TOTP_PERIOD))
}

fn generate_code(totp: &TOTP) -> Result<String> {
    totp.generate_current()
        .map_err(|_| anyhow::anyhow!(crate::constants::TOTP_ERROR))
}

pub fn copy_to_clipboard(text: String) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    spawn_clipboard_thread(text, tx);
    check_clipboard_result(rx)
}

fn spawn_clipboard_thread(text: String, tx: mpsc::Sender<()>) {
    thread::spawn(move || {
        if try_copy_to_clipboard(&text) {
            tx.send(()).ok();
        }
    });
}

fn try_copy_to_clipboard(text: &str) -> bool {
    try_wayland_copy(text) || try_xclip_copy(text)
}

fn try_wayland_copy(text: &str) -> bool {
    Command::new("wl-copy").arg(text).output().is_ok()
}

fn try_xclip_copy(text: &str) -> bool {
    let args = ["-selection", "clipboard", "-in"];
    Command::new("xclip")
        .args(args)
        .process_input(text.as_bytes())
        .is_ok()
}

fn check_clipboard_result(rx: mpsc::Receiver<()>) -> Result<()> {
    thread::sleep(Duration::from_millis(100));
    let result = rx.try_recv();
    match result {
        Ok(_) => Err(anyhow::anyhow!(crate::constants::CLIPBOARD_ERROR)),
        Err(_) => Ok(()),
    }
}

pub fn get_notification_title(
    error_message: &Option<(String, SystemTime)>,
    copy_notification_time: Option<SystemTime>,
) -> String {
    let error_msg = get_error_message(error_message);
    if let Some(msg) = error_msg {
        return msg;
    }

    let copy_msg = get_copy_message(copy_notification_time);
    if let Some(msg) = copy_msg {
        return msg;
    }

    AUTH_TITLE.to_string()
}

fn get_error_message(error_message: &Option<(String, SystemTime)>) -> Option<String> {
    let (msg, time) = error_message.as_ref()?;
    if time.elapsed().unwrap_or_default().as_secs() < 3 {
        Some(format!(" {} ", msg))
    } else {
        None
    }
}

fn get_copy_message(copy_notification_time: Option<SystemTime>) -> Option<String> {
    let notify_time = copy_notification_time?;
    if notify_time.elapsed().unwrap_or_default().as_secs() < 3 {
        Some(COPIED_MSG.to_string())
    } else {
        None
    }
}
