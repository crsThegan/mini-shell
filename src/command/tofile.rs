use std::{fs::OpenOptions, io, os::fd::AsRawFd, path::PathBuf};

use libc::STDOUT_FILENO;

use crate::{Command, Context, ExecuteError, ParseError, parser};

pub struct Cmd {
    cmd: Option<Box<dyn Command>>,
    filename: PathBuf,
}

impl Cmd {
    pub fn new() -> Self {
        Cmd {
            cmd: None,
            filename: PathBuf::new(),
        }
    }
}

impl Command for Cmd {
    fn parse(
        &mut self,
        ctx: Context,
        args: Vec<String>,
    ) -> Result<(), ParseError> {
        if args.len() != 2 {
            return Err(ParseError::new(
                "wrong number of arguments (2 needed)",
                "tofile",
            ));
        }

        if let Some(pipe) = parser::maybe_pipe(&args[0], &ctx)? {
            self.cmd = Some(pipe);
        } else {
            self.cmd = Some(parser::single_cmd(&args[0], &ctx)?);
        }

        self.filename = PathBuf::from(&args[1]);

        Ok(())
    }

    fn execute(&self, ctx: &mut Context) -> Result<(), ExecuteError> {
        let cmd = match &self.cmd {
            Some(c) => c,
            None => {
                return Err(ExecuteError::new(
                    "could not find command to execute",
                    "tofile",
                ));
            }
        };

        let file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)
        {
            Ok(f) => f,
            Err(e) => {
                return Err(ExecuteError::new(&e.to_string(), "tofile"));
            }
        };

        let pid = unsafe { libc::fork() };

        if pid == 0 {
            unsafe {
                libc::dup2(file.as_raw_fd(), STDOUT_FILENO);
            }
            drop(file);

            ctx.forked = true;
            cmd.execute(ctx)?;
            unsafe { libc::_exit(0) };
        } else if pid > 0 {
            let mut status = 0;
            unsafe { libc::waitpid(pid, &mut status, 0) };
        } else {
            return Err(ExecuteError::new(
                &io::Error::last_os_error().to_string(),
                "tofile",
            ));
        }

        Ok(())
    }
}
