use super::{Cmd, CmdName};
use crate::{ExecuteError, ParseError};
use std::{
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
};

use std::ffi::CStr;

fn parse(args: Vec<String>) -> Result<Cmd, ParseError> {
    let mut cmd = get();

    for substr in args.iter() {
        let mut s = substr.clone();

        if s.starts_with('-') {
            for flag in s.split_off(1).chars() {
                match flag {
                    'l' => cmd.flags.push((String::from("l"), None)),
                    'a' => cmd.flags.push((String::from("a"), None)),
                    _ => {
                        return Err(ParseError::new(
                            "ls: unidentified flag",
                            &s,
                        ));
                    }
                };
            }
        } else {
            cmd.args.push(s);
        }
    }

    Ok(cmd)
}

fn execute(parsed: Cmd) -> Result<(), ExecuteError> {
    let mut list_all = false;
    let mut all_info = false;

    for f in parsed.flags.iter() {
        match f.0.as_str() {
            "a" => list_all = true,
            "l" => all_info = true,
            _ => unreachable!(),
        };
    }

    if parsed.args.len() == 0 {
        parsed.args.push(String::from("."));
    }

    for arg in parsed.args.iter() {
        match fs::read_dir(arg) {
            Err(msg) => println!("ls: {arg} not found: {msg}"),
            Ok(dir) => {
                for maybe_entry in dir {
                    if let Ok(entry) = maybe_entry {
                        let entry_name =
                            entry.file_name().into_string().unwrap_or(todo!());

                        if entry_name.starts_with('.') && !list_all {
                            continue;
                        }

                        if all_info {
                            println!("total {}", Vec::from_iter(dir).len());
                            get_all_info(arg, &entry)?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn get_all_info(
    dirname: &str,
    entry: &fs::DirEntry,
) -> Result<(), ExecuteError> {
    match entry.file_type() {
        Err(msg) => println!("ls {dirname}: {msg}"),
        Ok(entry_type) => {
            if entry_type.is_dir() {
                print!("d");
            } else if entry_type.is_file() {
                print!("-");
            } else if entry_type.is_symlink() {
                print!("s");
            } else {
                print!("|");
                todo!()
            }
        }
    };

    let maybe_metadata = entry.metadata();
    match maybe_metadata {
        Err(msg) => {
            return Err(ExecuteError::new(
                &msg.to_string(),
                &format!("ls {dirname}"),
            ));
        }
        Ok(metadata) => {
            let perms = metadata.permissions().mode();

            let owner_perms = perms & 0o700;
            if owner_perms & 0o400 != 0 {
                print!("r");
            } else {
                print!("-");
            }

            if owner_perms & 0o200 != 0 {
                print!("w");
            } else {
                print!("-");
            }

            if owner_perms & 0o100 != 0 {
                print!("x");
            } else {
                print!("-");
            }

            let group_perms = perms & 0o070;
            if group_perms & 0o040 != 0 {
                print!("r");
            } else {
                print!("-");
            }

            if group_perms & 0o020 != 0 {
                print!("w");
            } else {
                print!("-");
            }

            if group_perms & 0o010 != 0 {
                print!("x");
            } else {
                print!("-");
            }

            let general_perms = perms & 0o007;
            if general_perms & 0o004 != 0 {
                print!("r");
            } else {
                print!("-");
            }

            if general_perms & 0o002 != 0 {
                print!("w");
            } else {
                print!("-");
            }

            if general_perms & 0o001 != 0 {
                print!("x");
            } else {
                print!("-");
            }

            print!(" {} ", metadata.nlink());

            let uid = metadata.uid();
            unsafe {
                let pwd = libc::getpwuid(uid);
                if !pwd.is_null() {
                    let owner_name =
                        CStr::from_ptr((*pwd).pw_name).to_string_lossy();

                    print!("{} ", owner_name);
                }
            }

            let gid = metadata.gid();
            unsafe {
                let grp = libc::getgrgid(gid);
                if !grp.is_null() {
                    let group_name =
                        CStr::from_ptr((*grp).gr_name).to_string_lossy();

                    print!("{} ", group_name);
                }
            }

            print!("{} ", metadata.size());

            match metadata.modified() {
                Err(msg) => {
                    return Err(ExecuteError::new(
                        &msg.to_string(),
                        &format!("ls {}", dirname),
                    ));
                }
                Ok(date) => {
                    todo!()
                }
            }
        }
    };

    Ok(())
}

fn get() -> Cmd {
    Cmd {
        name: CmdName::Ls,
        args: Vec::new(),
        flags: Vec::new(),
        parser: Box::new(parse),
        executor: Box::new(execute),
    }
}
