use std::env;
use std::path::PathBuf;

use auth::auth_core::app::App;
use serial_test::serial;

#[test]
#[serial]
fn test_home_path_expansion() {
    let app = App::new().unwrap();
    let home = dirs::home_dir().unwrap();

    let result = app.expand_path("~/test_home.txt");
    assert_eq!(result, home.join("test_home.txt"));
}

#[test]
#[serial]
fn test_env_var_expansion() {
    let app = App::new().unwrap();
    unsafe {
        env::set_var("TEST_ENV", "/tmp/test_env");
    }

    let result = app.expand_path("$TEST_ENV/test_file.txt");
    assert_eq!(result, PathBuf::from("/tmp/test_env/test_file.txt"));
}

#[test]
#[serial]
fn test_auth_entries_dir_env_var() {
    unsafe {
        env::set_var("AUTH_ENTRIES_DIR", "/tmp/test_auth_dir");
    }

    let app = App::new().unwrap();

    assert_eq!(
        app.entries_path.parent().unwrap(),
        PathBuf::from("/tmp/test_auth_dir")
    );

    unsafe {
        env::remove_var("AUTH_ENTRIES_DIR");
    }
    std::fs::remove_dir_all("/tmp/test_auth_dir").ok();
}

#[test]
#[serial]
fn test_absolute_path() {
    let app = App::new().unwrap();

    let result = app.expand_path("/etc/passwd");
    assert_eq!(result, PathBuf::from("/etc/passwd"));
}

#[test]
#[serial]
fn test_file_browser_dir_env_var() {
    let test_dir = PathBuf::from("/tmp/test_file_browser_dir");
    std::fs::create_dir_all(&test_dir).expect("Failed to create test directory");

    unsafe {
        env::set_var("AUTH_FILE_BROWSER_DIR", &test_dir);
    }

    let file_browser = auth::ui::file_browser::FileBrowser::new();

    assert_eq!(file_browser.get_current_dir(), test_dir);

    unsafe {
        env::remove_var("AUTH_FILE_BROWSER_DIR");
    }
    std::fs::remove_dir_all(&test_dir).ok();
}

#[test]
#[serial]
fn test_file_browser_invalid_dir() {
    let home = dirs::home_dir().unwrap();
    let nonexistent_dir = PathBuf::from("/tmp/nonexistent_dir_123456789");

    unsafe {
        env::set_var("AUTH_FILE_BROWSER_DIR", &nonexistent_dir);
    }

    let file_browser = auth::ui::file_browser::FileBrowser::new();

    assert_eq!(file_browser.get_current_dir(), home);

    unsafe {
        env::remove_var("AUTH_FILE_BROWSER_DIR");
    }
}
