use crate::constants::CLIPBOARD_ERROR;
use std::process::Command;

pub trait CommandExt {
    fn process_input(&mut self, input: &[u8]) -> std::io::Result<std::process::Output>;
    fn spawn_with_stdin(&mut self) -> std::io::Result<std::process::Child>;
    fn write_input_to_child(
        &self,
        child: &mut std::process::Child,
        input: &[u8],
    ) -> std::io::Result<()>;
    fn wait_for_child_output(
        &self,
        child: std::process::Child,
    ) -> std::io::Result<std::process::Output>;
}

impl CommandExt for Command {
    fn process_input(&mut self, input: &[u8]) -> std::io::Result<std::process::Output> {
        let mut child = self.spawn_with_stdin()?;
        self.write_input_to_child(&mut child, input)?;
        self.wait_for_child_output(child)
    }

    fn spawn_with_stdin(&mut self) -> std::io::Result<std::process::Child> {
        self.stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, CLIPBOARD_ERROR))
    }

    fn write_input_to_child(
        &self,
        child: &mut std::process::Child,
        input: &[u8],
    ) -> std::io::Result<()> {
        let stdin = match child.stdin.take() {
            Some(stdin) => stdin,
            None => return Ok(()),
        };

        let mut stdin = stdin;
        std::io::Write::write_all(&mut stdin, input)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, CLIPBOARD_ERROR))
    }

    fn wait_for_child_output(
        &self,
        child: std::process::Child,
    ) -> std::io::Result<std::process::Output> {
        child
            .wait_with_output()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, CLIPBOARD_ERROR))
    }
}
