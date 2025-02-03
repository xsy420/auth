use crate::constants::{TOTP_DIGITS, TOTP_PERIOD, TOTP_STEP};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Algorithm, TOTP};

pub fn generate_totp(secret: &str) -> Result<(String, u64)> {
    let secret = normalize_secret(secret);
    let key = decode_and_pad_secret(&secret)?;
    let (code, remaining) = generate_totp_code(key)?;
    Ok((code, remaining))
}

fn normalize_secret(secret: &str) -> String {
    let mut secret = secret.replace(' ', "").to_uppercase();
    while secret.len() % 8 != 0 {
        secret.push('=');
    }
    secret
}

fn decode_and_pad_secret(secret: &str) -> Result<Vec<u8>> {
    let decoded = base32::decode(base32::Alphabet::Rfc4648 { padding: true }, secret)
        .ok_or_else(|| anyhow::anyhow!(crate::constants::INVALID_KEY_ERROR))?;

    Ok(pad_secret_if_needed(decoded))
}

fn pad_secret_if_needed(decoded: Vec<u8>) -> Vec<u8> {
    if decoded.len() < 16 {
        let mut padded = vec![0u8; 16];
        padded[..decoded.len()].copy_from_slice(&decoded);
        padded[decoded.len()..].fill(0);
        padded
    } else {
        decoded
    }
}

fn generate_totp_code(key: Vec<u8>) -> Result<(String, u64)> {
    let totp = create_totp(key)?;
    let remaining = calculate_remaining_time()?;
    let code = generate_code(&totp)?;

    Ok((code, remaining))
}

fn create_totp(key: Vec<u8>) -> Result<TOTP> {
    TOTP::new(Algorithm::SHA1, TOTP_DIGITS, TOTP_STEP, TOTP_PERIOD, key)
        .map_err(|_| anyhow::anyhow!(crate::constants::TOTP_ERROR))
}

fn calculate_remaining_time() -> Result<u64> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| anyhow::anyhow!(crate::constants::TOTP_ERROR))?
        .as_secs();
    Ok(TOTP_PERIOD - (now % TOTP_PERIOD))
}

fn generate_code(totp: &TOTP) -> Result<String> {
    totp.generate_current()
        .map_err(|_| anyhow::anyhow!(crate::constants::TOTP_ERROR))
}
