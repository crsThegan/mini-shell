use crate::Command;

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
            println!("exit: No arguments required");
        }
        Ok(())
    }

    fn execute(
        &self,
        ctx: &mut crate::Context,
    ) -> Result<(), crate::ExecuteError> {
        ctx.done = true;
        Ok(())
    }
}
