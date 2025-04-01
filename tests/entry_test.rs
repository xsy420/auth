use auth::auth_core::entry::Entry;

#[test]
fn test_valid_entry_totp() {
    let entry = Entry {
        name: "Test Entry".to_string(),
        secret: "JBSWY3DPEHPK3PXP".to_string(),
    };

    let (code, remaining) = entry.generate_totp_with_time();
    assert_eq!(code.len(), 6);
    assert!(code.chars().all(|c| c.is_ascii_digit()));
    assert!(remaining <= 30);
}

#[test]
fn test_invalid_entry_totp() {
    let entry = Entry {
        name: "Invalid Entry".to_string(),
        secret: "INVALID!SECRET".to_string(),
    };

    let (code, remaining) = entry.generate_totp_with_time();
    assert_eq!(code, "Invalid");
    assert_eq!(remaining, 0);
}
