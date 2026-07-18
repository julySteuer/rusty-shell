use std::{fmt::{self, Display, Formatter, format}, io::{self, Read, Stdin, Stdout, Write, stdin, stdout}};

use termion::{clear, cursor::{Goto, Left, Right}, event::Key, input::TermRead, raw::{IntoRawMode, RawTerminal}};

fn remove_from_string_with_index(str: &mut String, idx: usize) {
    if idx >= str.chars().count() {
        return;
    }
    if str.chars().count() == idx {
        str.pop();
    } else {
        str.remove(idx);
    }
}

#[derive(Debug)]
pub enum IoProviderError {
    ErrorWhilePrinting,
}

impl Display for IoProviderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "IoProvider Error [{:?}] Occured", self)
    }
}

impl std::error::Error for IoProviderError {}

pub trait IoProvider {
    fn get_line(&mut self) -> Result<String, IoProviderError>;
    fn write(&mut self, payload: String) -> Result<(), IoProviderError>;
    fn write_line(&mut self, payload: String) -> Result<(), IoProviderError>;
}

pub struct RawIoProvider {
    stdout: RawTerminal<Stdout>
}

impl RawIoProvider {
    pub fn new() -> io::Result<Self> {
        let stdout = stdout().into_raw_mode()?;

        Ok(Self { stdout })
    }

    pub fn init_sceen(&mut self) -> io::Result<()> {
        write!(self.stdout, "{} {}", clear::All, Goto(1,1))
    }

    fn cursor_left(&mut self) -> Result<(), IoProviderError> {
        self.write(format!("{}", Left(1)))
    }

    fn cursor_right(&mut self) -> Result<(), IoProviderError> {
        self.write(format!("{}", Right(1)))
    }

    fn delete_previous_char_and_move_cursor(&mut self) -> Result<(), IoProviderError> {
        self.write(format!("{} {}", Left(1), Left(1)))
    }
}

impl IoProvider for RawIoProvider {
    fn get_line(&mut self) -> Result<String, IoProviderError> {
        let mut current_line = String::new();
        let mut cursor = 0;
        
        for c in stdin().keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    self.write("\n\r".to_string())?;
                    break;
                }
                Key::Left => {
                    self.cursor_left()?;
                    cursor -= 1;
                },
                Key::Right => {
                    if cursor == current_line.chars().count() {
                        continue;
                    }
                    self.cursor_right()?;
                    cursor += 1;
                },
                Key::Backspace => {
                    if cursor == 0 {
                        continue;
                    }
                    self.delete_previous_char_and_move_cursor()?;
                    cursor -= 1;
                    remove_from_string_with_index(&mut current_line, cursor);
                    self.write(format!("{}", clear::AfterCursor))?;
                    self.write(format!("{}", current_line[cursor..].to_string()))?;
                    if cursor != current_line.chars().count() {
                        self.cursor_left()?;
                    }
                },
                Key::Char(ch) => {
                    self.write(ch.to_string())?;
                    current_line.push(ch);
                    cursor += 1;
                }
                _ => {}
            }
        }

        Ok(current_line)
    }
    
    fn write(&mut self, payload: String) -> Result<(), IoProviderError> {
        self.stdout.write(payload.as_bytes())
            .and(self.stdout.flush())
            .map_err(|_| IoProviderError::ErrorWhilePrinting)
    }
    
    fn write_line(&mut self, payload: String) -> Result<(), IoProviderError> {
        self.write(format!("\r{}\n\r", payload))
    }
}