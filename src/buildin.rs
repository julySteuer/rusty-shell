use std::{env, error::Error, fmt};

use crate::{parser::Call, subprocess::BuildInProcess, subprocess::ExitCode};

// TODO: Maybe here BuildInCommands and BuildInOperations (For things like maybe addition)
#[derive(Debug)]
pub enum BuildInErrors {
    CdFailed,
    PwdFailed,
}

impl fmt::Display for BuildInErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Build in Error [{:?}] Occured", self)
    }
}

impl Error for BuildInErrors {}

pub enum BuildInCommand {
    CD(String),
    PWD,
}

impl BuildInCommand {
    pub fn build_from_call(call: &Call) -> Option<Self> {
        match call.prog.as_str() {
            "cd" => {
                let arg = call.arguments.first()?; // Maybe make to reaulst because this is an error
                Some(BuildInCommand::CD(arg.to_string()))
            }
            "pwd" => Some(BuildInCommand::PWD),
            _ => None,
        }
    }

    pub fn execute(&self) -> Result<BuildInProcess, BuildInErrors> {
        // Has to return something where a stdin and stdout can be derived from
        match self {
            BuildInCommand::CD(path) => execute_cd(path),
            BuildInCommand::PWD => execute_pwd(),
        }
    }
}

fn execute_cd(path: &String) -> Result<BuildInProcess, BuildInErrors> {
    env::set_current_dir(path)
        .map(|_| BuildInProcess::new(None, ExitCode(0)))
        .map_err(|_| BuildInErrors::CdFailed)
}

fn execute_pwd() -> Result<BuildInProcess, BuildInErrors> {
    env::current_dir()
        .map(|output| {
            BuildInProcess::new(Some(String::from(output.to_str().unwrap())), ExitCode(0))
        })
        .map_err(|_| BuildInErrors::PwdFailed)
}
