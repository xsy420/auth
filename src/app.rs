use crate::{
    crypto::Crypto,
    entry::{Entries, Entry},
    utils::copy_to_clipboard,
};
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::{env, fs, path::PathBuf, time::SystemTime};

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
            copy_to_clipboard(code)?;
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

            if path.extension().is_none_or(|ext| ext != "toml") {
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

    pub fn handle_events(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            self.handle_input_mode(key)?;
        }
        Ok(())
    }

    fn handle_input_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode(key),
            InputMode::Adding | InputMode::Editing => self.handle_entry_mode(key),
            InputMode::Importing | InputMode::Exporting => self.handle_file_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('c')
                if key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                self.should_quit = true
            }
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
        }
        Ok(())
    }

    fn handle_entry_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => self.reset_entry_state(),
            KeyCode::Enter => self.process_entry_input()?,
            KeyCode::Char(c) => self.update_entry_field(c),
            KeyCode::Backspace => self.remove_entry_char(),
            _ => {}
        }
        Ok(())
    }

    fn handle_file_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
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
        }
        Ok(())
    }

    fn reset_entry_state(&mut self) {
        self.input_mode = InputMode::Normal;
        if self.input_mode == InputMode::Adding {
            self.new_entry_name.clear();
            self.new_entry_secret.clear();
        } else {
            self.edit_entry_name.clear();
            self.edit_entry_secret.clear();
        }
        self.input_field = 0;
    }

    fn process_entry_input(&mut self) -> Result<()> {
        if self.input_field == 0 {
            self.input_field = 1;
        } else {
            match self.input_mode {
                InputMode::Adding => self.add_entry()?,
                InputMode::Editing => self.edit_entry()?,
                _ => unreachable!(),
            }
            self.reset_entry_state();
        }
        Ok(())
    }

    fn update_entry_field(&mut self, c: char) {
        let (name, secret) = if self.input_mode == InputMode::Adding {
            (&mut self.new_entry_name, &mut self.new_entry_secret)
        } else {
            (&mut self.edit_entry_name, &mut self.edit_entry_secret)
        };

        if self.input_field == 0 {
            name.push(c);
        } else {
            secret.push(c);
        }
    }

    fn remove_entry_char(&mut self) {
        let (name, secret) = if self.input_mode == InputMode::Adding {
            (&mut self.new_entry_name, &mut self.new_entry_secret)
        } else {
            (&mut self.edit_entry_name, &mut self.edit_entry_secret)
        };

        if self.input_field == 0 {
            name.pop();
        } else {
            secret.pop();
        }
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
