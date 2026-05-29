use std::fmt::{self, Display};

use crate::{output::Output, parser::parse_input};

#[derive(Debug)]
pub enum ExecutionError {
    ExecutionFailed
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Input Error [{:?}] Occured", self)
    }
}

impl std::error::Error for ExecutionError {}

pub struct ExitCode(pub u32);

impl Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub enum ExecutionResult {
    ShellStop,
    ShellRun { exit_code: ExitCode }
}

pub fn execute_shell(output_handler: &impl Output, command: String) -> Result<ExecutionResult, ExecutionError> {
    output_handler.write(command.clone()).map_err(|_| ExecutionError::ExecutionFailed)?;
    dbg!(parse_input(&command));
    if command.as_str().trim() == "q" {
        return Ok(ExecutionResult::ShellStop)
    }
    Ok(ExecutionResult::ShellRun { exit_code: ExitCode(0) })
}