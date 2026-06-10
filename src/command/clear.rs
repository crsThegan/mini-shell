use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

use crate::{Command, ExecuteError};

pub struct Cmd {}

impl Cmd {
    pub fn new() -> Self {
        Cmd {}
    }
}

impl Command for Cmd {
    fn parse(
        &mut self,
        _ctx: crate::Context,
        args: Vec<String>,
    ) -> Result<(), crate::ParseError> {
        if args.len() > 0 {
            println!("clear: No arguments required");
        }
        Ok(())
    }

    fn execute(
        &self,
        _ctx: &mut crate::Context,
    ) -> Result<(), crate::ExecuteError> {
        match execute!(stdout(), Clear(ClearType::All)) {
            Err(e) => Err(ExecuteError::new(&e.to_string(), "clear")),
            Ok(()) => {
                if let Err(e) = execute!(stdout(), MoveTo(0, 0)) {
                    Err(ExecuteError::new(&e.to_string(), "clear"))
                } else {
                    Ok(())
                }
            }
        }
    }
}
