use std::fmt::{self, Display};

use crate::{
    interpreter::{InterperterError, InterpreterResult, interpret_shell},
    output::Output,
    parser::parse_input,
};

#[derive(Debug)]
pub enum ExecutionError {
    ParsingFailed,
    ExecutionFailed,
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Input Error [{:?}] Occured", self)
    }
}

impl std::error::Error for ExecutionError {}

pub struct ExitCode(pub i32);

impl Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub enum ExecutionResult {
    ShellStop,
    ShellRun { exit_code: ExitCode },
}

// Add Shell State Here to store stuff like pwd ad maybe
pub fn execute_shell(
    output_handler: &impl Output,
    command: String,
) -> Result<ExecutionResult, ExecutionError> {
    output_handler
        .write(command.clone())
        .map_err(|_| ExecutionError::ExecutionFailed)?;
    if command.as_str().trim() == "q" {
        return Ok(ExecutionResult::ShellStop);
    }
    let shell_expr = parse_input(&command).map_err(|_| ExecutionError::ParsingFailed)?;
    let result = interpret_shell(shell_expr); // Map the whole result
    Ok(map_interpreter_result_to_execution_result(result))
}

fn map_interpreter_result_to_execution_result(
    interpreter_result: Result<InterpreterResult, InterperterError>,
) -> ExecutionResult {
    interpreter_result
        .map(|result| match result {
            InterpreterResult::ExitSuccess => ExecutionResult::ShellRun {
                exit_code: ExitCode(0),
            },
            InterpreterResult::ExitFailure(code) => ExecutionResult::ShellRun {
                exit_code: ExitCode(code as i32),
            },
            InterpreterResult::ExitCrash(code) => ExecutionResult::ShellRun {
                exit_code: ExitCode(code),
            },
        })
        .unwrap_or(ExecutionResult::ShellRun {
            exit_code: ExitCode(1),
        }) // Add more sophisticated handeling later. Like prog not found n stuff
}
