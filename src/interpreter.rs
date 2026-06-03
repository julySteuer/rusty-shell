use std::{error::Error, fmt, io::{Stdout, pipe}, process::{Child, ChildStdin, ChildStdout, Command, ExitCode, ExitStatus, Stdio}};

use crate::parser::{Call, Pipe, Redirect, ShellExpr};

#[derive(Debug)]
pub enum InterperterError {
    FailedToSpawn,
    CommandWasNotRun,
    FailedToWait
}

impl fmt::Display for InterperterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Interpreter Error [{:?}] Occured", self)
    }
}

impl Error for InterperterError {}

pub enum InterpreterResult {
    ExitSuccess,
    ExitFailure(u32),
    ExitCrash(i32),
}

impl InterpreterResult {
    pub fn from_exit_code(exit_code: ExitStatus) -> Self {
        if exit_code.success() {
            return InterpreterResult::ExitSuccess;
        }

        let exit_code = exit_code.code().unwrap();
        if exit_code < 0 {
            return InterpreterResult::ExitCrash(exit_code);
        }

        InterpreterResult::ExitFailure(exit_code as u32) // Was Checked earlier 
    }
}

pub fn interpret_shell(shell_expr: ShellExpr) -> Result<InterpreterResult, InterperterError> {
    let mut proc = interpret_shell_expr(shell_expr, Stdio::inherit(), Stdio::inherit())?;
    let exit_status = proc.wait().map_err(|_| InterperterError::FailedToWait)?;
    Ok(InterpreterResult::from_exit_code(exit_status))
}

fn interpret_shell_expr(shell_expr: ShellExpr, stdin: Stdio, stdout: Stdio) -> Result<Child, InterperterError> { // TODO: Rework this function 
    match shell_expr {
        ShellExpr::Pipe(pipe) => interpret_pipe(pipe, stdin, stdout),
        ShellExpr::Redirect(redirect) => interpret_redirect(redirect, stdin, stdout),
        ShellExpr::Call(call) => interpret_call(call, stdin, stdout),
    }
}

fn interpret_call(call: Call, stdin: Stdio, stdout: Stdio) -> Result<Child, InterperterError> {
    let command = Command::new(call.prog)
        .args(call.arguments)
        .stdin(stdin)
        .stdout(stdout)
        .spawn();
    let child = command.map_err(|_| InterperterError::FailedToSpawn)?; // Map to result withut err 
    Ok(child)
}

fn interpret_pipe(pipe: Pipe, stdin: Stdio, stdout: Stdio) -> Result<Child, InterperterError> {
    let left_recursive_expr = pipe.0;
    let left = interpret_shell_expr(*left_recursive_expr.left , stdin, Stdio::piped())?; // I think this one first
    let right = interpret_call(left_recursive_expr.right, Stdio::from(left.stdout.unwrap()), stdout)?;
    Ok(right)
}

fn interpret_redirect(redirect: Redirect, stdin: Stdio, stdout: Stdio) -> Result<Child, InterperterError> { // TODO: Make the right implementation here
    todo!()
}