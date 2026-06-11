use crate::{Command, Context, ParseError, command};

pub fn single_cmd(
    cmd_str: &str,
    ctx: &Context,
) -> Result<Box<dyn Command>, ParseError> {
    let (mut maybe_name, rest) =
        cmd_str.split_once(' ').unwrap_or((&cmd_str, ""));

    maybe_name = maybe_name.trim();
    match command::get_template(maybe_name) {
        Some(mut cmd) => {
            cmd.parse(
                ctx.clone(),
                rest.split_whitespace()
                    .map(|s| String::from(s.trim()))
                    .collect(),
            )?;
            Ok(cmd)
        }
        None => Err(ParseError::new(
            &format!("'{}' unidentified", maybe_name),
            &cmd_str,
        )),
    }
}

pub fn maybe_pipe(
    cmd_str: &str,
    ctx: &Context,
) -> Result<Option<Box<dyn Command>>, ParseError> {
    let cmds: Vec<&str> = cmd_str.split('|').map(|s| s.trim()).collect();

    if cmds.len() > 1 {
        let mut pipe = command::get_template("pipe").unwrap();
        let (r, l) = cmds.split_last().unwrap();
        let l = l.join(" | ");

        pipe.parse(ctx.clone(), vec![l, String::from(r.to_owned())])?;

        return Ok(Some(pipe));
    }
    Ok(None)
}
