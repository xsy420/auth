use anyhow::Result;
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
    pub fn generate_totp_with_time(&self) -> Result<(String, u64)> {
        crate::utils::generate_totp(&self.secret)
    }
}
