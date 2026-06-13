use std::{
    fs::File,
    io::{self, PipeReader, PipeWriter},
    os::fd::AsFd,
};

pub fn dup_stdin_to_read() -> PipeReader {
    let cloned = io::stdin().as_fd().try_clone_to_owned().unwrap();
    PipeReader::from(cloned)
}

pub fn dup_stdout_to_write() -> PipeWriter {
    let cloned = io::stdout().as_fd().try_clone_to_owned().unwrap();
    PipeWriter::from(cloned)
}

pub fn file_pipe_writer(name: &str) -> io::Result<PipeWriter> {
    let file = File::create(name)?;
    Ok(PipeWriter::from(file.as_fd().try_clone_to_owned().unwrap()))
}
