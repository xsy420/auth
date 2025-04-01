use std::time::{SystemTime, UNIX_EPOCH};

use totp_rs::{Algorithm, TOTP};

use crate::{AuthError, AuthResult};

pub fn generate_totp(secret: &str) -> AuthResult<(String, u64)> {
    let secret = normalize_secret(secret);
    let key = decode_and_pad_secret(&secret)?;
    let (code, remaining) = generate_totp_code(key)?;
    Ok((code, remaining))
}

fn normalize_secret(secret: &str) -> String {
    if secret.is_empty() {
        return String::new();
    }

    let mut secret = secret.replace(' ', "").to_uppercase();
    while secret.len() % 8 != 0 {
        secret.push('=');
    }
    secret
}

fn decode_and_pad_secret(secret: &str) -> AuthResult<Vec<u8>> {
    if secret.is_empty() {
        return Err(AuthError::InvalidKey("Empty secret".to_string()));
    }

    let decoded = base32::decode(base32::Alphabet::Rfc4648 { padding: true }, secret)
        .ok_or_else(|| AuthError::InvalidKey("Invalid base32 encoding".to_string()))?;

    Ok(pad_secret_if_needed(decoded))
}

fn pad_secret_if_needed(decoded: Vec<u8>) -> Vec<u8> {
    if decoded.len() < 16 {
        let mut padded = vec![0; 16];
        padded[..decoded.len()].copy_from_slice(&decoded);
        padded[decoded.len()..].fill(0);
        padded
    } else {
        decoded
    }
}

fn generate_totp_code(key: Vec<u8>) -> AuthResult<(String, u64)> {
    let totp = create_totp(key)?;
    let remaining = calculate_remaining_time()?;
    let code = generate_code(&totp)?;

    Ok((code, remaining))
}

fn create_totp(key: Vec<u8>) -> AuthResult<TOTP> {
    TOTP::new(Algorithm::SHA1, 6, 1, 30, key).map_err(|_| AuthError::TotpError)
}

fn calculate_remaining_time() -> AuthResult<u64> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AuthError::TotpError)?
        .as_secs();
    Ok(30 - (now % 30))
}

fn generate_code(totp: &TOTP) -> AuthResult<String> {
    totp.generate_current().map_err(|_| AuthError::TotpError)
}
