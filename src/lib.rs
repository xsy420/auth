pub mod auth_core {
    pub mod app;
    pub mod crypto;
    pub mod entry;
    pub mod totp;
}

pub mod utils {
    pub mod cli;
    pub mod command;
    pub mod error;
}

pub mod input {
    pub mod event;
    pub mod mouse;
    #[cfg(unix)]
    pub mod root;
}

pub mod ui {
    pub mod file_browser;
    pub mod layout;
    pub mod notification;
    pub mod renderer;
    pub mod size;
}

pub use auth_core::app::App;
pub use utils::error::{AuthError, AuthResult};
