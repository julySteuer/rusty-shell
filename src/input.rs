use std::fmt;

#[derive(Debug)]
pub enum InputError {
    LineCouldNotBeRead,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Input Error [{:?}] Occured", self)
    }
}

impl std::error::Error for InputError {}

pub trait Input {
    fn get_line(&self) -> Result<String, InputError>;
}

pub struct TerminalInput;

impl TerminalInput {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for TerminalInput {
    fn get_line(&self) -> Result<String, InputError> {
        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            Ok(_) => Ok(buf),
            Err(_) => Err(InputError::LineCouldNotBeRead),
        }
    }
}
