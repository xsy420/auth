use serde::{Deserialize, Serialize};

use crate::auth_core::totp::generate_totp;

#[derive(Serialize, Deserialize)]
pub struct Entries {
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub name: String,
    pub secret: String,
}

impl Entry {
    #[must_use]
    pub fn generate_totp_with_time(&self) -> (String, u64) {
        match generate_totp(&self.secret) {
            Ok(result) => result,
            Err(_) => ("Invalid".to_string(), 0),
        }
    }
}
