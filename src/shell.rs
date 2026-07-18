use crate::{
    executor::{ExecutionResult, execute_shell},
    input::Input,
    output::Output,
};

pub fn run_shell(
    input_handler: &impl Input,
    output_handler: &impl Output,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        output_handler.write(">".to_owned())?; // Global state so PWD can be read and stuff. Maybe a context pipeline 
        let line = input_handler.get_line()?; // Add a Non buffered input so if it is non buffered the command is written to stdout and ptr is moved forward 
        let result = execute_shell(line)?;
        match result {
            ExecutionResult::ShellStop => break,
            ExecutionResult::ShellRun { exit_code } => {
                output_handler.write(format!("Program Finished with code: {} \n", exit_code))?
            }
        }
    }
    Ok(())
}
