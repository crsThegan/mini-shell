use std::path::{Component, Path, PathBuf};

use crate::{Command, Context, ExecuteError, ParseError};

pub struct Cmd {
    path: PathBuf,
}

impl Cmd {
    pub fn new() -> Self {
        Cmd {
            path: PathBuf::new(),
        }
    }
}

impl Command for Cmd {
    fn parse(
        &mut self,
        ctx: Context,
        args: Vec<String>,
    ) -> Result<(), crate::ParseError> {
        if args.len() > 1 {
            return Err(ParseError::new("too many arguments", "cd"));
        }

        match args.get(0) {
            Some(v) => match v.as_str() {
                "." => self.path = ctx.cwd,
                _ if v.starts_with('/') => self.path = v.into(),
                rel => self.path = ctx.cwd.join(rel),
            },
            None => self.path = PathBuf::from("."),
        };

        self.path = pathify(self.path.as_path());

        Ok(())
    }

    fn execute(&self, ctx: &mut Context) -> Result<(), crate::ExecuteError> {
        match self.path.try_exists() {
            Ok(true) if self.path.is_dir() => {
                ctx.cwd = self.path.clone();
                Ok(())
            }
            Ok(true) => {
                Err(ExecuteError::new("path found, but it is not a dir", "cd"))
            }
            Ok(false) => Err(ExecuteError::new("path not found", "cd")),
            Err(e) => Err(ExecuteError::new(&e.to_string(), "cd")),
        }
    }
}

fn pathify(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }

    out
}
