use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::stdout,
    path::PathBuf,
    process::Command,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};
use totp_rs::{Algorithm, TOTP};

#[derive(Serialize, Deserialize)]
struct Entries {
    entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Entry {
    name: String,
    secret: String,
}

impl Entry {
    fn generate_totp_with_time(&self) -> Result<(String, u64)> {
        let mut secret = self.secret.replace(' ', "").to_uppercase();

        while secret.len() % 8 != 0 {
            secret.push('=');
        }

        let decoded = base32::decode(base32::Alphabet::Rfc4648 { padding: true }, &secret)
            .ok_or_else(|| anyhow::anyhow!("Invalid base32 secret"))?;

        if decoded.len() < 16 {
            let mut padded = vec![0u8; 16];
            padded[..decoded.len()].copy_from_slice(&decoded);
            padded[decoded.len()..].fill(0);

            let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, padded)?;

            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            let remaining = 30 - (now % 30);

            Ok((totp.generate_current()?, remaining))
        } else {
            let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, decoded)?;

            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            let remaining = 30 - (now % 30);

            Ok((totp.generate_current()?, remaining))
        }
    }
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Adding,
    Editing,
    Importing,
    Exporting,
}

struct App {
    should_quit: bool,
    entries: Vec<Entry>,
    selected: usize,
    input_mode: InputMode,
    new_entry_name: String,
    new_entry_secret: String,
    edit_entry_name: String,
    edit_entry_secret: String,
    input_field: usize,
    entries_path: PathBuf,
    copy_notification_time: Option<SystemTime>,
    path_input: String,
    error_message: Option<(String, SystemTime)>,
}

impl App {
    fn new() -> Result<Self> {
        let home = dirs::home_dir().expect("Could not find home directory");
        let auth_dir = home.join(".local/share/auth");
        fs::create_dir_all(&auth_dir)?;

        let entries_path = auth_dir.join("entries.toml");

        let mut app = Self {
            should_quit: false,
            entries: Vec::new(),
            selected: 0,
            input_mode: InputMode::Normal,
            new_entry_name: String::new(),
            new_entry_secret: String::new(),
            edit_entry_name: String::new(),
            edit_entry_secret: String::new(),
            input_field: 0,
            entries_path,
            copy_notification_time: None,
            path_input: String::new(),
            error_message: None,
        };

        app.load_entries()?;
        Ok(app)
    }

    fn load_entries(&mut self) -> Result<()> {
        if self.entries_path.exists() {
            let contents = fs::read_to_string(&self.entries_path)?;
            let entries: Entries = toml::from_str(&contents)?;
            self.entries = entries.entries;
        }
        Ok(())
    }

    fn save_entries(&self) -> Result<()> {
        let entries = Entries {
            entries: self.entries.clone(),
        };
        let contents = toml::to_string_pretty(&entries)?;
        fs::write(&self.entries_path, contents)?;
        Ok(())
    }

    fn add_entry(&mut self) -> Result<()> {
        if !self.new_entry_name.is_empty() && !self.new_entry_secret.is_empty() {
            self.entries.push(Entry {
                name: self.new_entry_name.clone(),
                secret: self.new_entry_secret.clone(),
            });
            self.save_entries()?;
        }
        Ok(())
    }

    fn delete_entry(&mut self) {
        if !self.entries.is_empty() {
            self.entries.remove(self.selected);
            if self.selected >= self.entries.len() && !self.entries.is_empty() {
                self.selected = self.entries.len() - 1;
            }
            self.save_entries().expect("Failed to save entries");
        }
    }

    fn copy_current_code(&mut self) -> Result<()> {
        if !self.entries.is_empty() {
            let entry = &self.entries[self.selected];
            let (code, _) = entry.generate_totp_with_time()?;

            thread::spawn(move || Command::new("wl-copy").arg(&code).output());

            self.copy_notification_time = Some(SystemTime::now());
        }
        Ok(())
    }

    fn show_error(&mut self, message: &str) {
        self.error_message = Some((message.to_string(), SystemTime::now()));
    }

    fn expand_path(&self, path: &str) -> PathBuf {
        if path.starts_with('~') {
            let home = dirs::home_dir().expect("Could not find home directory");
            home.join(&path[2..])
        } else if let Some(stripped) = path.strip_prefix('$') {
            let var_end = stripped.find('/').map(|i| i + 1).unwrap_or(path.len());
            let (var, rest) = stripped.split_at(var_end - 1);
            match env::var(var) {
                Ok(val) => PathBuf::from(val).join(rest.trim_start_matches('/')),
                Err(_) => PathBuf::from(path),
            }
        } else {
            PathBuf::from(path)
        }
    }

    fn import_entries(&mut self) -> Result<()> {
        if !self.path_input.is_empty() {
            let path = self.expand_path(&self.path_input);

            if !path.exists() {
                self.show_error("Invalid path");
                return Ok(());
            }

            if !path.extension().map_or(false, |ext| ext == "toml") {
                self.show_error("File must have .toml extension");
                return Ok(());
            }

            match fs::read_to_string(&path) {
                Ok(contents) => match toml::from_str::<Entries>(&contents) {
                    Ok(entries) => {
                        self.entries.extend(entries.entries);
                        if self.save_entries().is_err() {
                            self.show_error("Failed to save entries");
                        }
                    }
                    Err(_) => self.show_error("Failed to parse entries"),
                },
                Err(_) => self.show_error("Failed to read file"),
            }
        }
        Ok(())
    }

    fn export_entries(&mut self) -> Result<()> {
        if !self.path_input.is_empty() {
            let mut path = self.expand_path(&self.path_input);

            if !path.to_string_lossy().ends_with(".toml") {
                path.set_extension("toml");
            }

            let entries = Entries {
                entries: self.entries.clone(),
            };
            match toml::to_string_pretty(&entries) {
                Ok(contents) => {
                    if fs::write(&path, contents).is_err() {
                        self.show_error("Failed to write file");
                    }
                }
                Err(_) => self.show_error("Failed to serialize entries"),
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match &self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => self.should_quit = true,
                        KeyCode::Char('j') | KeyCode::Down => self.next_entry(),
                        KeyCode::Char('k') | KeyCode::Up => self.previous_entry(),
                        KeyCode::Char('a') => self.input_mode = InputMode::Adding,
                        KeyCode::Char('E') => self.start_editing(),
                        KeyCode::Char('d') => self.delete_entry(),
                        KeyCode::Char('i') => self.input_mode = InputMode::Importing,
                        KeyCode::Char('e') => self.input_mode = InputMode::Exporting,
                        KeyCode::Enter => self.copy_current_code()?,
                        _ => {}
                    },
                    InputMode::Importing | InputMode::Exporting => match key.code {
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                            self.path_input.clear();
                        }
                        KeyCode::Enter => {
                            match self.input_mode {
                                InputMode::Importing => self.import_entries()?,
                                InputMode::Exporting => self.export_entries()?,
                                _ => unreachable!(),
                            }
                            self.input_mode = InputMode::Normal;
                            self.path_input.clear();
                        }
                        KeyCode::Char(c) => self.path_input.push(c),
                        KeyCode::Backspace => {
                            self.path_input.pop();
                        }
                        _ => {}
                    },
                    InputMode::Adding => match key.code {
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                            self.new_entry_name.clear();
                            self.new_entry_secret.clear();
                            self.input_field = 0;
                        }
                        KeyCode::Enter => {
                            if self.input_field == 0 {
                                self.input_field = 1;
                            } else {
                                if let Err(e) = self.add_entry() {
                                    eprintln!("Failed to save entries: {}", e);
                                }
                                self.input_mode = InputMode::Normal;
                                self.new_entry_name.clear();
                                self.new_entry_secret.clear();
                                self.input_field = 0;
                            }
                        }
                        KeyCode::Char(c) => {
                            if self.input_field == 0 {
                                self.new_entry_name.push(c);
                            } else {
                                self.new_entry_secret.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            if self.input_field == 0 {
                                self.new_entry_name.pop();
                            } else {
                                self.new_entry_secret.pop();
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                            self.edit_entry_name.clear();
                            self.edit_entry_secret.clear();
                            self.input_field = 0;
                        }
                        KeyCode::Enter => {
                            if self.input_field == 0 {
                                self.input_field = 1;
                            } else {
                                if let Err(e) = self.edit_entry() {
                                    eprintln!("Failed to save entries: {}", e);
                                }
                                self.input_mode = InputMode::Normal;
                                self.edit_entry_name.clear();
                                self.edit_entry_secret.clear();
                                self.input_field = 0;
                            }
                        }
                        KeyCode::Char(c) => {
                            if self.input_field == 0 {
                                self.edit_entry_name.push(c);
                            } else {
                                self.edit_entry_secret.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            if self.input_field == 0 {
                                self.edit_entry_name.pop();
                            } else {
                                self.edit_entry_secret.pop();
                            }
                        }
                        _ => {}
                    },
                }
            }
        }
        Ok(())
    }

    fn next_entry(&mut self) {
        if !self.entries.is_empty() {
            self.selected = (self.selected + 1).min(self.entries.len() - 1);
        }
    }

    fn previous_entry(&mut self) {
        if !self.entries.is_empty() {
            self.selected = self.selected.saturating_sub(1);
        }
    }

    fn edit_entry(&mut self) -> Result<()> {
        if !self.entries.is_empty() {
            self.entries[self.selected] = Entry {
                name: self.edit_entry_name.clone(),
                secret: self.edit_entry_secret.clone(),
            };
            self.save_entries()?;
        }
        Ok(())
    }

    fn start_editing(&mut self) {
        if !self.entries.is_empty() {
            let entry = &self.entries[self.selected];
            self.edit_entry_name = entry.name.clone();
            self.edit_entry_secret = entry.secret.clone();
            self.input_mode = InputMode::Editing;
            self.input_field = 0;
        }
    }
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new()?;

    while !app.should_quit {
        terminal.draw(|frame| {
            let area = frame.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                .split(area);

            let main_block = Block::default()
                .title(if let Some((msg, time)) = &app.error_message {
                    if time.elapsed().unwrap_or_default().as_secs() < 3 {
                        format!(" {} ", msg)
                    } else {
                        app.error_message = None;
                        " Auth ".to_string()
                    }
                } else if let Some(notify_time) = app.copy_notification_time {
                    if notify_time.elapsed().unwrap_or_default().as_secs() < 3 {
                        " Copied! ".to_string()
                    } else {
                        app.copy_notification_time = None;
                        " Auth ".to_string()
                    }
                } else {
                    " Auth ".to_string()
                })
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(203, 153, 148)));

            let help_block = Block::default()
                .title(" Bindings ")
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

            let help_text = vec![Line::from(
                "a: add  E: edit  d: del  i: import  e: export  ↑/k: up  ↓/j: down  enter: copy  q: quit",
            )];

            let main_widget = Paragraph::new(entries)
                .block(main_block)
                .alignment(Alignment::Left);

            let help_widget = Paragraph::new(help_text)
                .block(help_block)
                .alignment(Alignment::Center);

            frame.render_widget(main_widget, chunks[0]);
            frame.render_widget(help_widget, chunks[1]);

            if app.input_mode == InputMode::Adding {
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
                        if app.input_field == 0 { "▎" } else { "" }
                    )),
                    Line::from(""),
                    Line::from("Secret:"),
                    Line::from(format!(
                        "{}{}",
                        app.new_entry_secret.as_str(),
                        if app.input_field == 1 { "▎" } else { "" }
                    )),
                ])
                .block(popup_block);

                frame.render_widget(Clear, area);
                frame.render_widget(popup, area);
            }

            if matches!(app.input_mode, InputMode::Importing | InputMode::Exporting) {
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
                    Line::from(format!("{}▎", app.path_input.as_str())),
                ])
                .block(popup_block);

                frame.render_widget(Clear, area);
                frame.render_widget(popup, area);
            }

            if app.input_mode == InputMode::Editing {
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
                        if app.input_field == 0 { "▎" } else { "" }
                    )),
                    Line::from(""),
                    Line::from("Secret:"),
                    Line::from(format!(
                        "{}{}",
                        app.edit_entry_secret.as_str(),
                        if app.input_field == 1 { "▎" } else { "" }
                    )),
                ])
                .block(popup_block);

                frame.render_widget(Clear, area);
                frame.render_widget(popup, area);
            }
        })?;

        app.handle_events()?;
    }

    Ok(())
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

fn main() -> Result<()> {
    startup()?;
    let result = run();
    shutdown()?;
    result
}
