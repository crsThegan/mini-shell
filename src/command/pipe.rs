use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

use crate::parser;
use crate::{Command, Context, ExecuteError, ParseError};

pub struct Cmd {
    args: Vec<Box<dyn Command>>,
}

impl Cmd {
    pub fn new() -> Self {
        Cmd { args: Vec::new() }
    }
}

impl Command for Cmd {
    fn parse(&mut self, ctx: Context, args: Vec<String>) -> Result<(), ParseError> {
        if args.len() != 2 {
            return Err(ParseError::new(
                "wrong number of arguments (needs 2)",
                "pipe",
            ));
        }

        for arg in args.iter() {
            if let Some(pipe) = parser::maybe_pipe(arg, &ctx)? {
                self.args.push(pipe);
            } else {
                let cmd = parser::single_cmd(arg, &ctx)?;
                self.args.push(cmd);
            }
        }

        Ok(())
    }

    fn execute(&self, ctx: &mut Context) -> Result<(), ExecuteError> {
        let mut fds = [0; 2];

        unsafe {
            libc::pipe(fds.as_mut_ptr());
        }

        let read_end = unsafe { OwnedFd::from_raw_fd(fds[0]) };
        let write_end = unsafe { OwnedFd::from_raw_fd(fds[1]) };

        let pid1 = unsafe { libc::fork() };

        if pid1 == 0 {
            unsafe {
                libc::dup2(write_end.as_raw_fd(), libc::STDOUT_FILENO);
            }
            drop(read_end);
            drop(write_end);

            ctx.forked = true;
            self.args[0].execute(ctx)?;
            unsafe { libc::_exit(0) };
        } else if pid1 > 0 {
            let pid2 = unsafe { libc::fork() };

            if pid2 == 0 {
                unsafe {
                    libc::dup2(read_end.as_raw_fd(), libc::STDIN_FILENO);
                }
                drop(read_end);
                drop(write_end);

                ctx.forked = true;
                self.args[1].execute(ctx)?;
                unsafe { libc::_exit(0) };
            } else if pid2 > 0 {
                drop(read_end);
                drop(write_end);

                let mut status = 0;
                unsafe {
                    libc::waitpid(pid1, &mut status, 0);
                    libc::waitpid(pid2, &mut status, 0);
                }
            } else {
                return Err(ExecuteError::new(
                    &io::Error::last_os_error().to_string(),
                    "pipe",
                ));
            }
        } else {
            return Err(ExecuteError::new(
                &io::Error::last_os_error().to_string(),
                "pipe",
            ));
        }

        Ok(())
    }
}
