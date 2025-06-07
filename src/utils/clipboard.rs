use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::utils::command::CommandExt;
use crate::utils::error::{AuthError, AuthResult};

/// # Errors
pub fn copy_to_clipboard(text: String) -> AuthResult<()> {
    let (tx, rx) = mpsc::channel();
    spawn_clipboard_thread(text, tx);
    check_clipboard_result(&rx)
}

fn spawn_clipboard_thread(text: String, tx: mpsc::Sender<()>) {
    thread::spawn(move || {
        if try_copy_to_clipboard(&text) {
            tx.send(()).ok();
        }
    });
}

fn try_copy_to_clipboard(text: &str) -> bool {
    if is_wayland_session() {
        try_wayland_copy(text)
    } else {
        try_xclip_copy(text)
    }
}

fn is_wayland_session() -> bool {
    std::env::var("WAYLAND_DISPLAY").is_ok()
}

fn try_wayland_copy(text: &str) -> bool {
    Command::new("wl-copy")
        .arg(text)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

fn try_xclip_copy(text: &str) -> bool {
    let args = ["-selection", "clipboard", "-in"];
    Command::new("xclip")
        .args(args)
        .process_input(text.as_bytes())
        .is_ok()
}

fn check_clipboard_result(rx: &mpsc::Receiver<()>) -> AuthResult<()> {
    thread::sleep(Duration::from_millis(100));
    rx.try_recv().map_err(|_| AuthError::ClipboardError)
}
