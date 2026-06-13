use std::{
    clone,
    error::Error,
    fmt,
    fs::{self, File},
    io::{self, PipeReader, PipeWriter, Read, Write, pipe},
    os::fd::{AsFd, AsRawFd, FromRawFd},
    process::{Child, Command, ExitStatus, Stdio},
};

use crate::{
    buildin::BuildInCommand,
    parser::{Call, Pipe, Redirect, ShellExpr},
    pipe_utils::{dup_stdin_to_read, dup_stdout_to_write, file_pipe_writer},
    subprocess::{BuildInProcess, ExitCode, Process},
};

// Have to rebuild this with a piplining architecture that is closer to nushell
// Maybe this https://doc.rust-lang.org/beta/std/io/fn.pipe.html
#[derive(Debug)]
pub enum InterperterError {
    FailedToSpawn,
    CouldNotWriteToFile,
    CommandWasNotRun,
    FailedToWait,
    CouldNotPipe,
    CouldNotCopyPipe,
    CouldNotBuffer,
    CouldNotWrite,
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
    pub fn from_exit_code(exit_code: ExitCode) -> Self {
        let code = exit_code.code();
        if code == 0 {
            return InterpreterResult::ExitSuccess;
        }

        if code < 0 {
            return InterpreterResult::ExitCrash(code);
        }

        InterpreterResult::ExitFailure(code as u32) // Was Checked earlier 
    }
}

pub fn interpret_shell(shell_expr: ShellExpr) -> Result<InterpreterResult, InterperterError> {
    let proc = interpret_shell_expr(shell_expr, dup_stdin_to_read(), dup_stdout_to_write())?;
    let exit_status = proc.wait();
    Ok(InterpreterResult::from_exit_code(exit_status))
}

fn interpret_shell_expr(
    shell_expr: ShellExpr,
    stdin: PipeReader,
    stdout: PipeWriter,
) -> Result<Process, InterperterError> {
    // TODO: Rework this function
    match shell_expr {
        ShellExpr::Pipe(pipe) => interpret_pipe(pipe, stdin, stdout),
        ShellExpr::Redirect(redirect) => interpret_redirect(redirect, stdin, stdout),
        ShellExpr::Call(call) => interpret_call(call, stdin, stdout),
    }
}

fn interpret_buildin_call(
    buildin: BuildInCommand,
    mut stdout: PipeWriter,
) -> Result<BuildInProcess, InterperterError> {
    let child = buildin
        .execute()
        .map_err(|_| InterperterError::FailedToSpawn)?;
    if let Some(ref output) = child.stdout {
        stdout
            .write(output.as_bytes())
            .map_err(|_| InterperterError::CouldNotWrite)?;
    }
    Ok(child)
}

fn interpret_call(
    call: Call,
    stdin: PipeReader,
    stdout: PipeWriter,
) -> Result<Process, InterperterError> {
    if let Some(buildin) = BuildInCommand::build_from_call(&call) {
        let build_in_process = interpret_buildin_call(buildin, stdout)?;
        return Ok(Process::BuildIn(build_in_process));
    }
    let command = Command::new(call.prog)
        .args(call.arguments)
        .stdin(stdin)
        .stdout(stdout)
        .spawn();
    let child = command.map_err(|_| InterperterError::FailedToSpawn)?;
    Ok(Process::External(child))
}

fn interpret_pipe(
    pipe_expr: Pipe,
    stdin: PipeReader,
    stdout: PipeWriter,
) -> Result<Process, InterperterError> {
    let left_recursive_expr = pipe_expr.0;
    let (pipe_reader, pipe_writer) = pipe().map_err(|_| InterperterError::CouldNotPipe)?;
    let left = interpret_shell_expr(*left_recursive_expr.left, stdin, pipe_writer)?; // I think this one first
    let right = interpret_call(left_recursive_expr.right, pipe_reader, stdout)?;
    left.wait();
    Ok(right)
}

fn interpret_redirect(
    redirect: Redirect,
    stdin: PipeReader,
    mut stdout: PipeWriter,
) -> Result<Process, InterperterError> {
    // Make this non buffered
    let left_recursive_expr = redirect.0;
    let file_name = left_recursive_expr.right;
    let pipe_writer = file_pipe_writer(&file_name).map_err(|_| InterperterError::CouldNotPipe)?;
    let left = interpret_shell_expr(*left_recursive_expr.left, stdin, pipe_writer)?;
    left.wait();
    stdout
        .write_all(&file_name.as_bytes())
        .map_err(|_| InterperterError::CouldNotWrite)?; // Make this not buffered maybe
    Ok(Process::BuildIn(BuildInProcess {
        stdout: Some(file_name),
        status: ExitCode(0),
    }))
}
