use auth::auth_core::totp::generate_totp;

#[test]
fn test_valid_totp_generation() {
    let result = generate_totp("JBSWY3DPEHPK3PXP");
    assert!(result.is_ok());

    let (code, remaining) = result.unwrap();
    assert_eq!(code.len(), 6);
    assert!(remaining <= 30);
    assert!(remaining >= 1);
}

#[test]
fn test_invalid_totp_secret() {
    let result = generate_totp("INVALID!SECRET");
    assert!(result.is_err());
}

#[test]
fn test_empty_totp_secret() {
    let result = generate_totp("");
    assert!(result.is_err(), "Empty secret should fail base32 decoding");
}

#[test]
fn test_padded_totp_secret() {
    let result = generate_totp("JBSWY3DPEHPK3PX=");
    assert!(result.is_ok(), "Padded secret should be valid");

    let (code, remaining) = result.unwrap();
    assert_eq!(code.len(), 6);
    assert!(remaining <= 30);
}

#[test]
fn test_normalized_secret() {
    let result = generate_totp("jbsw y3dp ehpk 3pxp");
    assert!(result.is_ok(), "Normalized secret should be valid");

    let (code, remaining) = result.unwrap();
    assert_eq!(code.len(), 6);
    assert!(remaining <= 30);
}
