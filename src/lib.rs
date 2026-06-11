pub mod command;
pub mod err;
pub mod parser;
pub mod shell;

pub use command::Command;
pub use err::{ExecuteError, ParseError};
pub use shell::{Context, Shell, ShellMode};

fn print_help(err_msg: &str) {
    println!("error: {err_msg}");
    println!("usage: tsh [-e <COMMAND> | -f <PATH>]")
}

fn parse_flags(args: &Vec<String>) -> Result<ShellMode, String> {
    match args[1].as_str() {
        "-e" => Ok(ShellMode::OneLiner(args[2].clone())),
        "-f" => Ok(ShellMode::File(args[2].clone())),
        _ => Err(String::from("Incorrect specified flag")),
    }
}

pub fn run(args: Vec<String>) {
    let mut s: Shell;

    match args.len() {
        1 => {
            s = Shell::new();
        }
        3 => match parse_flags(&args) {
            Ok(mode) => {
                s = Shell::from_mode(mode);
            }
            Err(msg) => {
                print_help(msg.as_str());
                return;
            }
        },
        _ => {
            print_help("Incorrect number of arguments");
            return;
        }
    }

    s.start();
}
