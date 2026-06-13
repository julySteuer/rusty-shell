use std::{
    fmt::{self, Display},
    io,
    process::{Child, ExitStatus},
};

#[derive(Debug)]
pub struct ExitCode(pub i32);

impl ExitCode {
    pub fn from_exit_status(exit_status: Result<ExitStatus, io::Error>) -> Self {
        exit_status
            .map(|status| {
                if status.success() {
                    return Self(0);
                }
                Self(status.code().unwrap())
            })
            .unwrap_or(Self(-1))
    }

    pub fn code(&self) -> i32 {
        self.0
    }
}

impl Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct BuildInProcess {
    pub stdout: Option<String>,
    pub status: ExitCode,
}

impl BuildInProcess {
    pub fn new(stdout: Option<String>, status: ExitCode) -> Self {
        BuildInProcess { stdout, status }
    }
}

#[derive(Debug)]
pub enum Process {
    BuildIn(BuildInProcess),
    External(Child),
}

impl Process {
    pub fn wait(self) -> ExitCode {
        match self {
            Process::BuildIn(build_in_process) => build_in_process.status,
            Process::External(mut child) => ExitCode::from_exit_status(child.wait()),
        }
    }
}

#[derive(Debug)]
pub struct ProcList(Vec<Process>);

impl ProcList {
    pub fn new(process: Process) -> Self {
        Self(vec![process])
    }

    pub fn append(mut self, process: Process) -> Self {
        self.0.push(process);
        Self(self.0)
    }

    pub fn concat(mut self, mut process: ProcList) -> Self {
        self.0.append(&mut process.0);
        Self(self.0)
    }

    pub fn reap(self) -> Option<ExitCode> {
        dbg!(&self);
        // Returns the last exit code
        self.0.into_iter().fold(None, |_, p| Some(p.wait()))
    }
}
