use std::env;
use std::path::PathBuf;

use auth::auth_core::app::App;

#[test]
fn test_home_path_expansion() {
    let home = dirs::home_dir().unwrap();

    let result = App::expand_path("~/test_home.txt");
    assert_eq!(result, home.join("test_home.txt"));
}

#[test]
fn test_env_var_expansion() {
    let test_env = &env::temp_dir().join("test_env");
    unsafe {
        env::set_var("TEST_ENV", test_env);
    }

    let result = App::expand_path("$TEST_ENV/test_file.txt");
    assert_eq!(result, test_env.join("test_file.txt"));
}

#[test]
fn test_auth_entries_dir_env_var() {
    let test_auth_dir = &env::temp_dir().join("test_auth_dir");
    unsafe {
        env::set_var("AUTH_ENTRIES_DIR", test_auth_dir);
    }

    let app = App::new().unwrap();

    assert_eq!(app.entries_path.parent().unwrap(), test_auth_dir);

    unsafe {
        env::remove_var("AUTH_ENTRIES_DIR");
    }
    std::fs::remove_dir_all(test_auth_dir).ok();
}

#[test]
fn test_absolute_path() {
    let test_path = if cfg!(windows) {
        r"C:\Windows\System32\drivers\etc\hosts"
    } else {
        "/etc/passwd"
    };
    let result = App::expand_path(test_path);
    assert_eq!(result, PathBuf::from(test_path));
}

#[test]
fn test_file_browser_dir_env_var() {
    let test_dir = env::temp_dir().join("test_file_browser_dir");
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
fn test_file_browser_invalid_dir() {
    let home = dirs::home_dir().unwrap();
    let nonexistent_dir = env::temp_dir().join("nonexistent_dir_123456789");

    unsafe {
        env::set_var("AUTH_FILE_BROWSER_DIR", &nonexistent_dir);
    }

    let file_browser = auth::ui::file_browser::FileBrowser::new();

    assert_eq!(file_browser.get_current_dir(), home);

    unsafe {
        env::remove_var("AUTH_FILE_BROWSER_DIR");
    }
}
