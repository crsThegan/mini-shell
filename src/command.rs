mod cd;
mod exit;
mod external;

use std::{env, path::PathBuf};

use crate::{Context, ExecuteError, ParseError};

pub trait Command {
    fn parse(
        &mut self,
        ctx: Context,
        args: Vec<String>,
    ) -> Result<(), ParseError>;
    fn execute(&self, ctx: &mut Context) -> Result<(), ExecuteError>;
}

pub fn get_template(cmd_name: &str) -> Option<Box<dyn Command>> {
    match cmd_name {
        "cd" => Some(Box::new(cd::Cmd::new())),
        "export" => todo!(),
        "alias" => todo!(),
        "unset" => todo!(),
        "exit" => Some(Box::new(exit::Cmd::new())),
        other => match find_external(other) {
            Some(path) => Some(Box::new(external::Cmd::new(&path))),
            None => None,
        },
    }
}

fn find_external(cmd_name: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;

    for dir in env::split_paths(&path) {
        let candidate = dir.join(cmd_name);

        if candidate.is_file() {
            return Some(candidate);
        }
    }

    None
}
