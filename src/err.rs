use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ExecuteError {
    msg: String,
    cmd_name: String,
}

impl ExecuteError {
    pub fn new(msg: &str, cmd_name: &str) -> Self {
        ExecuteError {
            msg: String::from(msg),
            cmd_name: String::from(cmd_name),
        }
    }
}

impl Error for ExecuteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to execute cmd '{}': {}", self.cmd_name, self.msg)
    }
}

#[derive(Debug)]
pub struct ParseError {
    msg: String,
    parsed_str: String,
}

impl ParseError {
    pub fn new(msg: &str, parsed_str: &str) -> Self {
        ParseError {
            msg: String::from(msg),
            parsed_str: String::from(parsed_str),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse '{}': {}", self.parsed_str, self.msg)
    }
}
