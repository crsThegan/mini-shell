use core::error::Error;
use std::{
    env,
    ffi::CStr,
    fs::File,
    io::{self, BufRead, BufReader, Write},
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
    pub fn new() -> Self {
        Shell {
            mode: ShellMode::Full,
            ctx: Context::new(),
        }
    }

    pub fn from_mode(mode: ShellMode) -> Self {
        Shell {
            mode,
            ctx: Context::new(),
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
            ShellMode::Full => {
                while !self.ctx.done {
                    match self.prompt() {
                        Err(e) => eprintln!("{e}"),
                        Ok(cmd_str) => match self.parse(&cmd_str) {
                            Ok(cmd) => {
                                if let Err(e) = self.execute(cmd) {
                                    eprintln!("{e}");
                                }
                            }
                            Err(e) => eprintln!("{e}"),
                        },
                    }
                }
            }
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

    fn prompt(&self) -> io::Result<String> {
        let uname = get_uname().unwrap_or("?".to_string());
        let cwd = env::current_dir()?
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        print!("{} {}$ ", uname, cwd);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }
}

#[derive(Clone)]
pub struct Context {
    pub done: bool,
}

impl Context {
    pub fn new() -> Self {
        Context { done: false }
    }
}

fn get_uname() -> Option<String> {
    unsafe {
        let uid = libc::getuid();
        let pwd = libc::getpwuid(uid);

        if pwd.is_null() {
            return None;
        }

        Some(
            CStr::from_ptr((*pwd).pw_name)
                .to_string_lossy()
                .into_owned(),
        )
    }
}
