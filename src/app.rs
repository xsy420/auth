use crate::{
    crypto::Crypto,
    entry::{Entries, Entry},
    utils::CommandExt,
};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::{env, fs, path::PathBuf, process::Command, thread, time::SystemTime};

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Adding,
    Editing,
    Importing,
    Exporting,
}

pub struct App {
    pub should_quit: bool,
    pub entries: Vec<Entry>,
    pub selected: usize,
    pub input_mode: InputMode,
    pub new_entry_name: String,
    pub new_entry_secret: String,
    pub edit_entry_name: String,
    pub edit_entry_secret: String,
    pub input_field: usize,
    pub entries_path: PathBuf,
    pub copy_notification_time: Option<SystemTime>,
    pub path_input: String,
    pub error_message: Option<(String, SystemTime)>,
    crypto: Crypto,
}

impl App {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().expect("Could not find home directory");
        let auth_dir = home.join(".local/share/auth");
        fs::create_dir_all(&auth_dir)?;

        let entries_path = auth_dir.join("entries.toml");
        let crypto = Crypto::new(&auth_dir)?;

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
            crypto,
        };

        app.load_entries()?;
        Ok(app)
    }

    fn load_entries(&mut self) -> Result<()> {
        if self.entries_path.exists() {
            let encrypted = fs::read(&self.entries_path)?;
            let decrypted = self.crypto.decrypt(&encrypted)?;
            let contents = String::from_utf8(decrypted)?;
            let entries: Entries = toml::from_str(&contents)?;
            self.entries = entries.entries;
        }
        Ok(())
    }

    pub fn save_entries(&self) -> Result<()> {
        let entries = Entries {
            entries: self.entries.clone(),
        };
        let contents = toml::to_string_pretty(&entries)?;
        let encrypted = self.crypto.encrypt(contents.as_bytes())?;
        fs::write(&self.entries_path, encrypted)?;
        Ok(())
    }

    pub fn add_entry(&mut self) -> Result<()> {
        if !self.new_entry_name.is_empty() && !self.new_entry_secret.is_empty() {
            self.entries.push(Entry {
                name: self.new_entry_name.clone(),
                secret: self.new_entry_secret.clone(),
            });
            self.save_entries()?;
        }
        Ok(())
    }

    pub fn delete_entry(&mut self) {
        if !self.entries.is_empty() {
            self.entries.remove(self.selected);
            if self.selected >= self.entries.len() && !self.entries.is_empty() {
                self.selected = self.entries.len() - 1;
            }
            self.save_entries().expect("Failed to save entries");
        }
    }

    pub fn copy_current_code(&mut self) -> Result<()> {
        if !self.entries.is_empty() {
            let entry = &self.entries[self.selected];
            let (code, _) = entry.generate_totp_with_time()?;

            thread::spawn(move || {
                if Command::new("wl-copy").arg(&code).output().is_err()
                    && Command::new("xclip")
                        .arg("-selection")
                        .arg("clipboard")
                        .arg("-in")
                        .process_input(code.as_bytes())
                        .is_err()
                {
                    eprintln!("Failed to copy to clipboard: neither wl-copy nor xclip worked");
                }
            });

            self.copy_notification_time = Some(SystemTime::now());
        }
        Ok(())
    }

    pub fn show_error(&mut self, message: &str) {
        self.error_message = Some((message.to_string(), SystemTime::now()));
    }

    pub fn expand_path(&self, path: &str) -> PathBuf {
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

    pub fn import_entries(&mut self) -> Result<()> {
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

    pub fn export_entries(&mut self) -> Result<()> {
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

    pub fn handle_events(&mut self) -> Result<()> {
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
            self.selected = (self.selected + 1) % self.entries.len();
        }
    }

    fn previous_entry(&mut self) {
        if !self.entries.is_empty() {
            self.selected = self
                .selected
                .checked_sub(1)
                .unwrap_or(self.entries.len() - 1);
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
