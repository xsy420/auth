use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io::stdout, process::Command};

pub trait CommandExt {
    fn process_input(&mut self, input: &[u8]) -> std::io::Result<std::process::Output>;
}

impl CommandExt for Command {
    fn process_input(&mut self, input: &[u8]) -> std::io::Result<std::process::Output> {
        let mut child = self.stdin(std::process::Stdio::piped()).spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            std::io::Write::write_all(&mut stdin, input)?;
        }
        child.wait_with_output()
    }
}

pub fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Ok(())
}

pub fn shutdown() -> Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
