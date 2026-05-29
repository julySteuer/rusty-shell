use std::{fmt, io::{self, Write}};

#[derive(Debug)]
pub enum OutputError {
    CouldNotWrite
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Input Error [{:?}] Occured", self)
    }
}

impl std::error::Error for OutputError {}

pub trait Output {
    fn write(&self, payload: String) -> Result<(), OutputError>;
}

pub struct TerminalOutput;

impl TerminalOutput {
    pub fn new() -> Self {
        Self {}
    }
}

impl Output for TerminalOutput {
    fn write(&self, payload: String) -> Result<(), OutputError> {
        io::stdout().write_all(&payload.into_bytes())
            .and_then(|_| io::stdout().flush())
            .map_err(|_| OutputError::CouldNotWrite)
    }
}