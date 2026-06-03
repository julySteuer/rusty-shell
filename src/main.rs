use std::error::Error;

use crate::{input::TerminalInput, output::TerminalOutput};

mod executor;
mod input;
mod interpreter;
mod output;
mod parser;
mod shell;

fn main() -> Result<(), Box<dyn Error>> {
    let input_handler = TerminalInput::new();
    let output_handler = TerminalOutput::new();
    shell::run_shell(&input_handler, &output_handler)
}
