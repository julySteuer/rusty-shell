use crate::{
    executor::{ExecutionResult, execute_shell}, io::IoProvider
};

pub fn run_shell<Provider: IoProvider>(
    mut io_provider: Provider
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        io_provider.write(">".to_owned())?; // Global state so PWD can be read and stuff. Maybe a context pipeline 
        let line = io_provider.get_line()?; // Add a Non buffered input so if it is non buffered the command is written to stdout and ptr is moved forward 
        let result = execute_shell(line)?;
        match result {
            ExecutionResult::ShellStop => break,
            ExecutionResult::ShellRun { exit_code } => io_provider.write_line(format!("Program Finished with code: {}", exit_code))?,
            ExecutionResult::Empty => io_provider.write("".to_string())?,
        }
    }
    Ok(())
}
