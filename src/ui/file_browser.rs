use std::path::{Path, PathBuf};
use std::{env, fs};

use ratatui::prelude::*;
use ratatui::widgets::{Clear, List, ListItem, ListState};

use crate::auth_core::app::App;
use crate::ui::layout::{centered_rect, create_block};

const FILE_BROWSER_DIR_ENV: &str = "AUTH_FILE_BROWSER_DIR";

#[derive(Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

pub struct FileBrowser {
    current_dir: PathBuf,
    entries: Vec<FileEntry>,
    selected: usize,
    scroll: usize,
    max_visible: usize,
    show_hidden: bool,
}

impl Default for FileBrowser {
    fn default() -> Self {
        Self::new()
    }
}

impl FileBrowser {
    pub fn new() -> Self {
        let default_dir = Self::get_default_directory();
        let mut browser = Self {
            current_dir: default_dir,
            entries: Vec::new(),
            selected: 0,
            scroll: 0,
            max_visible: 11,
            show_hidden: false,
        };
        browser.refresh_entries();
        browser
    }

    fn get_default_directory() -> PathBuf {
        if let Ok(dir) = env::var(FILE_BROWSER_DIR_ENV) {
            let path = PathBuf::from(dir);
            if path.exists() && path.is_dir() {
                return path;
            }
        }

        dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
    }

    pub fn reset(&mut self) {
        self.current_dir = Self::get_default_directory();
        self.selected = 0;
        self.scroll = 0;
        self.refresh_entries();
    }

    pub fn refresh_entries(&mut self) {
        self.entries.clear();

        if let Some(parent) = self.current_dir.parent() {
            self.entries.push(FileEntry {
                name: "..".to_string(),
                path: parent.to_path_buf(),
                is_dir: true,
            });
        }

        if let Ok(read_dir) = fs::read_dir(&self.current_dir) {
            let mut dirs = Vec::new();
            let mut files = Vec::new();

            for entry in read_dir.filter_map(Result::ok) {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = path.is_dir();

                if !self.show_hidden && name.starts_with('.') && name != ".." {
                    continue;
                }

                let file_entry = FileEntry { name, path, is_dir };

                if is_dir {
                    dirs.push(file_entry);
                } else {
                    files.push(file_entry);
                }
            }

            dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            self.entries.extend(dirs);
            self.entries.extend(files);
        }

        if self.selected >= self.entries.len() && !self.entries.is_empty() {
            self.selected = 0;
            self.scroll = 0;
        }
    }

    pub fn move_up(&mut self) {
        if !self.entries.is_empty() {
            if self.selected > 0 {
                self.selected -= 1;
            } else {
                self.selected = self.entries.len() - 1;
            }

            if self.selected < self.scroll {
                self.scroll = self.selected;
            } else if self.selected >= self.scroll + self.max_visible {
                self.scroll = self.selected.saturating_sub(self.max_visible) + 1;
            }
        }
    }

    pub fn move_down(&mut self) {
        if !self.entries.is_empty() {
            if self.selected < self.entries.len() - 1 {
                self.selected += 1;
            } else {
                self.selected = 0;
            }

            if self.selected >= self.scroll + self.max_visible {
                self.scroll = self.selected - self.max_visible + 1;
            } else if self.selected < self.scroll {
                self.scroll = 0;
            }
        }
    }

    pub fn enter(&mut self) -> Option<PathBuf> {
        if self.entries.is_empty() {
            return None;
        }

        let entry = &self.entries[self.selected];

        if entry.is_dir {
            self.current_dir = entry.path.clone();
            self.selected = 0;
            self.scroll = 0;
            self.refresh_entries();
            None
        } else {
            Some(entry.path.clone())
        }
    }

    pub fn get_selected_path(&self) -> Option<PathBuf> {
        if self.entries.is_empty() {
            None
        } else {
            Some(self.entries[self.selected].path.clone())
        }
    }

    pub fn get_current_dir(&self) -> &Path {
        &self.current_dir
    }

    pub fn toggle_hidden_files(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.refresh_entries();
    }
}

pub fn draw_file_browser(
    frame: &mut Frame,
    _app: &App,
    browser: &FileBrowser,
    title: &str,
    area: Rect,
) {
    let popup_area = centered_rect(50, 25, area);
    frame.render_widget(Clear, popup_area);

    let visible_entries = browser
        .entries
        .iter()
        .skip(browser.scroll)
        .take(browser.max_visible)
        .enumerate()
        .map(|(i, entry)| {
            let index = i + browser.scroll;
            let prefix = if entry.is_dir { "📁 " } else { "📄 " };
            let item = ListItem::new(format!("{}{}", prefix, entry.name));

            if index == browser.selected {
                item.style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                item
            }
        })
        .collect::<Vec<ListItem>>();

    let file_list = List::new(visible_entries).block(create_block(title));

    let mut list_state = ListState::default();
    list_state.select(Some(browser.selected - browser.scroll));

    frame.render_stateful_widget(file_list, popup_area, &mut list_state);
}
