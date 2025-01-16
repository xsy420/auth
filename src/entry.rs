use crate::constants::INVALID_LABEL;
use serde::{Deserialize, Serialize};

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
    pub fn generate_totp_with_time(&self) -> (String, u64) {
        match crate::utils::generate_totp(&self.secret) {
            Ok(result) => result,
            Err(_) => (INVALID_LABEL.to_string(), 0),
        }
    }
}
