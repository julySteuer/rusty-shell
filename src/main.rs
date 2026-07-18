use std::error::Error;

use crate::io::RawIoProvider;

mod buildin;
mod executor;
mod interpreter;
mod io;
mod parser;
mod pipe_utils;
mod shell;
mod subprocess;

fn main() -> Result<(), Box<dyn Error>> {
    let mut io_provider = RawIoProvider::new()?;
    io_provider.init_sceen()?;
    shell::run_shell(io_provider)
}
