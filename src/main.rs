use std::error::Error;

use crate::{input::TerminalInput, output::TerminalOutput};

mod shell;
mod input;
mod executor;
mod output;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let input_handler = TerminalInput::new();
    let output_handler = TerminalOutput::new();
    shell::run_shell(&input_handler, &output_handler)
}
