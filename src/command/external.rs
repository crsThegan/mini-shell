use std::{os::unix::process::CommandExt, path::PathBuf, process};

use crate::{Command, ExecuteError};

pub struct Cmd {
    path: PathBuf,
    args: Vec<String>,
}

impl Cmd {
    pub fn new(path: &PathBuf) -> Self {
        Cmd {
            path: path.clone(),
            args: Vec::new(),
        }
    }
}

impl Command for Cmd {
    fn parse(&mut self, _ctx: crate::Context, args: Vec<String>) -> Result<(), crate::ParseError> {
        self.args = args.clone();
        Ok(())
    }

    fn execute(&self, ctx: &mut crate::Context) -> Result<(), crate::ExecuteError> {
        let mut cmd = process::Command::new(&self.path);
        let mut handler = &mut cmd;

        for a in self.args.iter() {
            handler = handler.arg(a);
        }

        if !ctx.forked {
            match handler.status() {
                Ok(_) => Ok(()),
                Err(e) => Err(ExecuteError::new(
                    &e.to_string(),
                    &self.path.to_string_lossy(),
                )),
            }
        } else {
            ctx.forked = false;

            let e = handler.exec();

            eprintln!("{e}");
            unsafe {
                libc::_exit(127);
            }
        }
    }
}
