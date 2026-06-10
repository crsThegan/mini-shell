use core::error::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use super::command;
use super::{Command, ExecuteError, ParseError};

pub enum ShellMode {
    OneLiner(String),
    Full,
    File(String),
}

pub struct Shell {
    pub mode: ShellMode,
    pub ctx: Context,
}

impl Shell {
    pub fn new(cwd: PathBuf) -> Self {
        Shell {
            mode: ShellMode::Full,
            ctx: Context::new(cwd),
        }
    }

    pub fn from_mode(mode: ShellMode, cwd: PathBuf) -> Self {
        Shell {
            mode,
            ctx: Context::new(cwd),
        }
    }

    pub fn start(&mut self) {
        match self.mode {
            ShellMode::OneLiner(ref c) => match self.parse(c) {
                Ok(cmd) => {
                    if let Err(e) = self.execute(cmd) {
                        eprintln!("{e}");
                    }
                }
                Err(e) => eprintln!("{e}"),
            },
            ShellMode::File(ref path) => {
                if let Err(e) = self.execute_from_path(path) {
                    eprintln!("{e}");
                }
            }
            ShellMode::Full => todo!(),
        }
    }

    fn parse(&self, cmd_str: &str) -> Result<Box<dyn Command>, ParseError> {
        let cmd_str = String::from(cmd_str);

        let (maybe_name, rest) =
            cmd_str.split_once(' ').unwrap_or((&cmd_str, ""));
        match command::get_template(maybe_name) {
            Some(mut cmd) => {
                cmd.parse(
                    self.ctx.clone(),
                    rest.split_whitespace().map(|s| String::from(s)).collect(),
                )?;
                Ok(cmd)
            }
            None => Err(ParseError::new(
                &format!("'{}' unidentified", maybe_name),
                &cmd_str,
            )),
        }
    }

    fn execute(&mut self, cmd: Box<dyn Command>) -> Result<(), ExecuteError> {
        cmd.execute(&mut self.ctx)
    }

    fn execute_from_path(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            self.parse(&line)?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Context {
    pub cwd: PathBuf,
}

impl Context {
    pub fn new(cwd: PathBuf) -> Self {
        Context { cwd }
    }
}
