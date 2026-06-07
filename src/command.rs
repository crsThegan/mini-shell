mod ls;

use crate::{ExecuteError, ParseError};

pub enum CmdName {
    Ls,
    Cd,
    Mkdir,
    Rm,
    Cp,
    Mv,
}

pub struct Cmd {
    name: CmdName,
    args: Vec<String>,
    flags: Vec<Flag>,
    parser: Parser,
    executor: Executor,
}
type Flag = (String, Option<String>);
type Parser = Box<dyn Fn(Vec<String>) -> Result<Cmd, ParseError>>;
type Executor = Box<dyn Fn() -> Result<(), ExecuteError>>;

fn parse_cmd_name(cmd_name: &str) -> Option<CmdName> {
    match cmd_name {
        "ls" => Some(CmdName::Ls),
        "cd" => Some(CmdName::Cd),
        "mkdir" | "md" => Some(CmdName::Mkdir),
        "rm" => Some(CmdName::Rm),
        "cp" => Some(CmdName::Cp),
        "mv" => Some(CmdName::Mv),
        _ => None,
    }
}

impl Cmd {
    pub fn parse(cmd_str: &str) -> Result<Cmd, ParseError> {
        let cmd_str = String::from(cmd_str);

        let (maybe_name, rest) =
            cmd_str.split_once(' ').unwrap_or((&cmd_str, ""));
        match parse_cmd_name(maybe_name) {
            Some(name) => (get_template(name).parser)(
                rest.split(' ').map(|s| String::from(s)).collect(),
            ),
            None => Err(ParseError::new(
                &format!("'{}' unidentified", maybe_name),
                &cmd_str,
            )),
        }
    }

    pub fn execute(&self) -> Result<(), ExecuteError> {
        (self.executor)()
    }
}

fn get_template(cmd_name: CmdName) -> Cmd {
    todo!()
}
