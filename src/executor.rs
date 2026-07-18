use std::fmt::{self};

use crate::{
    interpreter::{InterperterError, InterpreterResult, interpret_shell},
    parser::parse_input,
    subprocess::ExitCode,
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

pub enum ExecutionResult {
    ShellStop,
    ShellRun { exit_code: ExitCode },
    Empty,
}

// Add Shell State Here to store stuff like pwd ad maybe
pub fn execute_shell(command: String) -> Result<ExecutionResult, ExecutionError> {
    if command.as_str().trim() == "q" {
        return Ok(ExecutionResult::ShellStop);
    }
    if command.is_empty() {
        return Ok(ExecutionResult::Empty);
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
