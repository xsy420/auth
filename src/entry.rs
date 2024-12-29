use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Algorithm, TOTP};

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
