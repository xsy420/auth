use crate::utils::{
    command::CommandExt,
    constants::{
        CLIPBOARD_SLEEP_DURATION, WAYLAND_COPY_COMMAND, WAYLAND_DISPLAY, XCLIP_CLIPBOARD_ARG,
        XCLIP_COMMAND, XCLIP_IN_ARG, XCLIP_SELECTION_ARG,
    },
    error::{AuthError, AuthResult},
};
use std::{process::Command, sync::mpsc, thread, time::Duration};

pub fn copy_to_clipboard(text: String) -> AuthResult<()> {
    let (tx, rx) = mpsc::channel();
    spawn_clipboard_thread(text, tx);
    check_clipboard_result(rx)
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
    std::env::var(WAYLAND_DISPLAY).is_ok()
}

// I shouldn't have to write these comments but @adamperkowski made me do it.
// See command.rs for more details, if this isn't clear enough.

/// Attempts to copy text to clipboard using wl-copy on Wayland
/// Handles clipboard operations in an isolated process with null stdio
/// This prevents potential command injection since text is passed as a direct argument
/// Returns true if copying succeeded, false otherwise
fn try_wayland_copy(text: &str) -> bool {
    Command::new(WAYLAND_COPY_COMMAND)
        .arg(text)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

/// Attempts to copy text to clipboard using xclip on X11
/// Uses a separate trait implementation for secure process input handling
/// Text is passed through a controlled pipe rather than shell arguments
/// Returns true if copying succeeded, false otherwise
fn try_xclip_copy(text: &str) -> bool {
    let args = [XCLIP_SELECTION_ARG, XCLIP_CLIPBOARD_ARG, XCLIP_IN_ARG];
    Command::new(XCLIP_COMMAND)
        .args(args)
        .process_input(text.as_bytes())
        .is_ok()
}

fn check_clipboard_result(rx: mpsc::Receiver<()>) -> AuthResult<()> {
    thread::sleep(Duration::from_millis(CLIPBOARD_SLEEP_DURATION));
    rx.try_recv().map_err(|_| AuthError::ClipboardError)
}
