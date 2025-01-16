use crate::{
    constants::{
        CLIPBOARD_ERROR, CREATE_DIR_ERROR, CRYPTO_INIT_ERROR, DECRYPT_ERROR, DIRECTORY_ERROR,
        EMPTY_ENTRY_ERROR, EMPTY_EXPORT_ERROR, ENCRYPTOR_ERROR, FILE_NOT_EXIST_ERROR,
        HOME_DIR_ERROR, NO_FILENAME_ERROR, PARSE_ERROR, READ_ERROR, SAVE_ERROR, SERIALIZE_ERROR,
        TOML_EXT_ERROR, UTF8_ERROR, WRITE_ERROR,
    },
    crypto::Crypto,
    entry::{Entries, Entry},
    utils::copy_to_clipboard,
};
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::{env, fs, path::Path, path::PathBuf, time::SystemTime};

#[derive(PartialEq, Clone)]
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
        let auth_dir = Self::get_auth_directory()?;
        let entries_path = auth_dir.join("entries.toml");
        let crypto = Self::initialize_crypto(&auth_dir)?;
        let mut app = Self::create_initial_app(entries_path, crypto);

        Self::try_load_entries(&mut app);
        Ok(app)
    }

    fn get_auth_directory() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!(HOME_DIR_ERROR))?;
        let auth_dir = home.join(".local/share/auth");

        fs::create_dir_all(&auth_dir).map_err(|_| anyhow::anyhow!(CREATE_DIR_ERROR))?;

        Ok(auth_dir)
    }

    fn initialize_crypto(auth_dir: &Path) -> Result<Crypto> {
        Crypto::new(auth_dir).map_err(|_| anyhow::anyhow!(CRYPTO_INIT_ERROR))
    }

    fn create_initial_app(entries_path: PathBuf, crypto: Crypto) -> App {
        App {
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
        }
    }

    fn try_load_entries(app: &mut App) {
        if app.load_entries().is_err() {
            app.show_error(READ_ERROR);
        }
    }

    fn load_entries(&mut self) -> Result<()> {
        if !self.entries_path.exists() {
            return Ok(());
        }

        let encrypted = self.read_encrypted_file()?;
        let decrypted = self.decrypt_data(&encrypted)?;
        let contents = self.parse_utf8(&decrypted)?;
        self.parse_entries(&contents)?;

        Ok(())
    }

    fn read_encrypted_file(&mut self) -> Result<Vec<u8>> {
        fs::read(&self.entries_path).map_err(|_| {
            self.show_error(READ_ERROR);
            anyhow::anyhow!(READ_ERROR)
        })
    }

    fn decrypt_data(&mut self, encrypted: &[u8]) -> Result<Vec<u8>> {
        self.crypto.decrypt(encrypted).map_err(|_| {
            self.show_error(DECRYPT_ERROR);
            anyhow::anyhow!(DECRYPT_ERROR)
        })
    }

    fn parse_utf8(&mut self, data: &[u8]) -> Result<String> {
        String::from_utf8(data.to_vec()).map_err(|_| {
            self.show_error(UTF8_ERROR);
            anyhow::anyhow!(UTF8_ERROR)
        })
    }

    fn parse_entries(&mut self, contents: &str) -> Result<()> {
        let entries = toml::from_str::<Entries>(contents).map_err(|_| {
            self.show_error(PARSE_ERROR);
            anyhow::anyhow!(PARSE_ERROR)
        })?;

        self.entries = entries.entries;
        Ok(())
    }

    pub fn save_entries(&mut self) -> Result<()> {
        let contents = self.serialize_entries()?;
        let encrypted = self.encrypt_contents(&contents)?;
        self.write_encrypted_file(&encrypted)?;
        Ok(())
    }

    fn serialize_entries(&mut self) -> Result<String> {
        let entries = Entries {
            entries: self.entries.clone(),
        };

        toml::to_string_pretty(&entries).map_err(|_| {
            self.show_error(SERIALIZE_ERROR);
            anyhow::anyhow!(SERIALIZE_ERROR)
        })
    }

    fn encrypt_contents(&mut self, contents: &str) -> Result<Vec<u8>> {
        self.crypto.encrypt(contents.as_bytes()).map_err(|_| {
            self.show_error(ENCRYPTOR_ERROR);
            anyhow::anyhow!(ENCRYPTOR_ERROR)
        })
    }

    fn write_encrypted_file(&mut self, encrypted: &[u8]) -> Result<()> {
        fs::write(&self.entries_path, encrypted).map_err(|_| {
            self.show_error(WRITE_ERROR);
            anyhow::anyhow!(WRITE_ERROR)
        })
    }

    pub fn add_entry(&mut self) -> Result<()> {
        if self.new_entry_name.is_empty() || self.new_entry_secret.is_empty() {
            self.show_error(EMPTY_ENTRY_ERROR);
            return Ok(());
        }
        self.entries.push(Entry {
            name: self.new_entry_name.clone(),
            secret: self.new_entry_secret.clone(),
        });
        if self.save_entries().is_err() {
            self.show_error(SAVE_ERROR);
        }
        Ok(())
    }

    pub fn delete_entry(&mut self) {
        if !self.entries.is_empty() {
            self.entries.remove(self.selected);
            if self.selected >= self.entries.len() && !self.entries.is_empty() {
                self.selected = self.entries.len() - 1;
            }
            if self.save_entries().is_err() {
                self.show_error(SAVE_ERROR);
            }
        }
    }

    pub fn copy_current_code(&mut self) -> Result<()> {
        if self.entries.is_empty() {
            return Ok(());
        }

        self.try_copy_code()?;
        Ok(())
    }

    fn try_copy_code(&mut self) -> Result<()> {
        let entry = &self.entries[self.selected];
        let (code, _) = entry.generate_totp_with_time();

        if copy_to_clipboard(code).is_err() {
            self.show_error(CLIPBOARD_ERROR);
            return Ok(());
        }

        self.copy_notification_time = Some(SystemTime::now());
        Ok(())
    }

    pub fn show_error(&mut self, message: &str) {
        self.error_message = Some((message.to_string(), SystemTime::now()));
    }

    pub fn expand_path(&self, path: &str) -> PathBuf {
        if path.starts_with('~') {
            return self.expand_home_path(path);
        }

        if let Some(stripped) = path.strip_prefix('$') {
            return self.expand_env_path(stripped, path);
        }

        PathBuf::from(path)
    }

    fn expand_home_path(&self, path: &str) -> PathBuf {
        match dirs::home_dir() {
            Some(home) => {
                if path.len() == 1 {
                    home
                } else {
                    home.join(&path[2..])
                }
            }
            None => PathBuf::from(path),
        }
    }

    fn expand_env_path(&self, stripped: &str, original_path: &str) -> PathBuf {
        let var_end = Self::get_var_end(stripped, original_path);
        let (var, rest) = stripped.split_at(var_end - 1);
        let expanded_path = Self::expand_env_var(var, rest);
        expanded_path.unwrap_or_else(|| PathBuf::from(original_path))
    }

    fn get_var_end(stripped: &str, original_path: &str) -> usize {
        stripped
            .find('/')
            .map(|i| i + 1)
            .unwrap_or(original_path.len())
    }

    fn expand_env_var(var: &str, rest: &str) -> Option<PathBuf> {
        env::var(var)
            .ok()
            .map(|val| PathBuf::from(val).join(rest.trim_start_matches('/')))
    }

    pub fn import_entries(&mut self) -> Result<()> {
        if self.path_input.is_empty() {
            return Ok(());
        }

        let path = self.get_validated_import_path()?;
        let entries = self.read_and_parse_entries(&path)?;
        self.merge_and_save_entries(entries)?;

        Ok(())
    }

    fn get_validated_import_path(&mut self) -> Result<PathBuf> {
        let path = self.expand_path(&self.path_input);
        self.validate_import_path(&path)?;
        Ok(path)
    }

    fn read_and_parse_entries(&mut self, path: &Path) -> Result<Entries> {
        if !self.validate_file_exists(path) {
            return Ok(Entries { entries: vec![] });
        }

        let contents = self.read_file_contents(path);
        self.parse_toml_contents(contents)
    }

    fn validate_file_exists(&mut self, path: &Path) -> bool {
        if !path.exists() {
            self.show_error(FILE_NOT_EXIST_ERROR);
            return false;
        }
        true
    }

    fn read_file_contents(&mut self, path: &Path) -> String {
        match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(_) => {
                self.show_error(READ_ERROR);
                String::new()
            }
        }
    }

    fn parse_toml_contents(&mut self, contents: String) -> Result<Entries> {
        if contents.is_empty() {
            return Ok(Entries { entries: vec![] });
        }

        match toml::from_str(&contents) {
            Ok(entries) => Ok(entries),
            Err(_) => {
                self.show_error(PARSE_ERROR);
                Ok(Entries { entries: vec![] })
            }
        }
    }

    fn merge_and_save_entries(&mut self, entries: Entries) -> Result<()> {
        self.entries.extend(entries.entries);

        if self.save_entries().is_err() {
            self.show_error(SAVE_ERROR);
        }

        Ok(())
    }

    fn validate_import_path(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            self.show_error(FILE_NOT_EXIST_ERROR);
            return Ok(());
        }

        if path.is_dir() {
            self.show_error(DIRECTORY_ERROR);
            return Ok(());
        }

        if path.extension().is_none_or(|ext| ext != "toml") {
            self.show_error(TOML_EXT_ERROR);
            return Ok(());
        }

        Ok(())
    }

    pub fn export_entries(&mut self) -> Result<()> {
        if self.path_input.is_empty() {
            self.show_error(NO_FILENAME_ERROR);
            return Ok(());
        }

        if self.entries.is_empty() {
            self.show_error(EMPTY_EXPORT_ERROR);
            return Ok(());
        }

        let path = self.get_validated_export_path()?;
        let contents = self.serialize_entries()?;
        self.write_export_file(&path, &contents)?;

        Ok(())
    }

    fn get_validated_export_path(&mut self) -> Result<PathBuf> {
        let mut path = self.expand_path(&self.path_input);

        if path.is_dir() || self.path_input.ends_with('/') || self.path_input.ends_with('\\') {
            self.show_error(NO_FILENAME_ERROR);
            return Ok(path);
        }

        if !path.to_string_lossy().ends_with(".toml") {
            path.set_extension("toml");
        }
        Ok(path)
    }

    fn write_export_file(&mut self, path: &Path, contents: &str) -> Result<()> {
        if path.is_dir() {
            self.show_error(DIRECTORY_ERROR);
            return Ok(());
        }

        if fs::write(path, contents).is_err() {
            self.show_error(WRITE_ERROR);
            return Ok(());
        }
        Ok(())
    }

    pub fn handle_events(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key) => self.handle_key_event(key),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        if self.check_control_quit(key) {
            return Ok(());
        }

        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode(key),
            InputMode::Adding | InputMode::Editing => self.handle_entry_mode(key),
            InputMode::Importing | InputMode::Exporting => self.handle_file_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        if self.check_control_quit(key) {
            return Ok(());
        }

        self.handle_normal_key(key)?;
        Ok(())
    }

    fn handle_normal_key(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
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

    fn check_control_quit(&mut self, key: crossterm::event::KeyEvent) -> bool {
        if matches!(key.code, KeyCode::Char('q') | KeyCode::Char('c'))
            && key.modifiers.contains(KeyModifiers::CONTROL)
        {
            self.should_quit = true;
            return true;
        }
        false
    }

    fn handle_entry_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => self.reset_entry_state(),
            KeyCode::Enter => self.process_entry_input()?,
            KeyCode::Char(c) => self.update_entry_field(c),
            KeyCode::Backspace => self.remove_entry_char(),
            KeyCode::Tab => self.handle_tab_key(key.modifiers.contains(KeyModifiers::SHIFT)),
            _ => {}
        }
        Ok(())
    }

    fn handle_tab_key(&mut self, is_shift: bool) {
        self.input_field = match (self.input_field, is_shift) {
            (0, true) | (1, false) => 0,
            (0, false) | (1, true) => 1,
            _ => unreachable!(),
        };
    }

    fn handle_file_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => self.reset_file_mode(),
            KeyCode::Enter => self.process_file_mode_input()?,
            KeyCode::Char(c) => self.path_input.push(c),
            KeyCode::Backspace => {
                self.path_input.pop();
            }
            _ => {}
        }
        Ok(())
    }

    fn reset_file_mode(&mut self) {
        self.input_mode = InputMode::Normal;
        self.path_input.clear();
    }

    fn process_file_mode_input(&mut self) -> Result<()> {
        self.handle_file_operation()?;
        self.reset_file_mode();
        Ok(())
    }

    fn handle_file_operation(&mut self) -> Result<()> {
        match self.input_mode {
            InputMode::Importing => self.import_entries(),
            InputMode::Exporting => self.export_entries(),
            _ => unreachable!(),
        }
    }

    fn reset_entry_state(&mut self) {
        let current_mode = self.input_mode.clone();
        self.input_mode = InputMode::Normal;
        if current_mode == InputMode::Adding {
            self.new_entry_name.clear();
            self.new_entry_secret.clear();
        } else {
            self.edit_entry_name.clear();
            self.edit_entry_secret.clear();
        }
        self.input_field = 0;
    }

    fn process_entry_input(&mut self) -> Result<()> {
        if self.is_name_field() {
            self.switch_to_secret_field();
            return Ok(());
        }
        self.handle_final_entry_input()
    }

    fn is_name_field(&self) -> bool {
        self.input_field == 0
    }

    fn handle_final_entry_input(&mut self) -> Result<()> {
        match self.input_mode {
            InputMode::Adding => self.handle_add_entry()?,
            InputMode::Editing => self.handle_edit_entry()?,
            _ => unreachable!(),
        }
        self.reset_input_field();
        Ok(())
    }

    fn handle_add_entry(&mut self) -> Result<()> {
        self.add_entry()?;
        self.reset_entry_state();
        Ok(())
    }

    fn handle_edit_entry(&mut self) -> Result<()> {
        self.edit_entry()?;
        self.reset_entry_state();
        Ok(())
    }

    fn reset_input_field(&mut self) {
        self.input_field = 0;
    }

    fn switch_to_secret_field(&mut self) {
        self.input_field = 1;
    }

    fn update_entry_field(&mut self, c: char) {
        let field = self.get_current_field();
        field.push(c);
    }

    fn remove_entry_char(&mut self) {
        let field = self.get_current_field();
        field.pop();
    }

    fn get_current_field(&mut self) -> &mut String {
        match (self.input_mode.clone(), self.input_field) {
            (InputMode::Adding, 0) => &mut self.new_entry_name,
            (InputMode::Adding, _) => &mut self.new_entry_secret,
            (_, 0) => &mut self.edit_entry_name,
            (_, _) => &mut self.edit_entry_secret,
        }
    }

    fn edit_entry(&mut self) -> Result<()> {
        if self.entries.is_empty() {
            return Ok(());
        }

        if !self.validate_edit_entry()? {
            return Ok(());
        }

        self.update_entry();
        self.try_save_entries();
        Ok(())
    }

    fn validate_edit_entry(&mut self) -> Result<bool> {
        if self.edit_entry_name.is_empty() || self.edit_entry_secret.is_empty() {
            self.show_error(EMPTY_ENTRY_ERROR);
            return Ok(false);
        }
        Ok(true)
    }

    fn update_entry(&mut self) {
        self.entries[self.selected] = Entry {
            name: self.edit_entry_name.clone(),
            secret: self.edit_entry_secret.clone(),
        };
    }

    fn try_save_entries(&mut self) {
        if self.save_entries().is_err() {
            self.show_error(SAVE_ERROR);
        }
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
}
