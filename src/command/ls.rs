use chrono::{DateTime, Local};

use super::{Arg, Cmd, CmdName, Flag};
use crate::{ExecuteError, ParseError};
use core::fmt;
use std::{
    collections::HashSet,
    fmt::Write,
    fs::{self, FileType},
    io,
    os::unix::fs::{MetadataExt, PermissionsExt},
};

use std::ffi::CStr;

fn parse(args: Vec<String>) -> Result<Cmd, ParseError> {
    let mut cmd = get();
    let mut used_flags = HashSet::new();

    let mut check_flag_if_used = |f: char| {
        if !used_flags.insert(f) {
            return Err(ParseError::new(
                &format!("-{} flag was already set", f),
                &format!("ls -{}", f),
            ));
        }
        Ok(())
    };

    for substr in args.iter() {
        let mut s = substr.clone();

        if s.starts_with('-') {
            for flag in s.split_off(1).chars() {
                match flag {
                    'l' => {
                        if let Err(e) = check_flag_if_used('l') {
                            return Err(e);
                        }
                        cmd.flags.push((String::from("l"), None));
                    }
                    'a' => {
                        if let Err(e) = check_flag_if_used('a') {
                            return Err(e);
                        }
                        cmd.flags.push((String::from("a"), None));
                    }
                    wrong => {
                        return Err(ParseError::new(
                            "unidentified flag",
                            &format!("ls -{wrong}"),
                        ));
                    }
                };
            }
        } else {
            cmd.args.push(s);
        }
    }

    if args.len() == 0 {
        cmd.args.push(String::from("."));
    }

    Ok(cmd)
}

fn execute(args: &Vec<Arg>, flags: &Vec<Flag>) -> Result<(), ExecuteError> {
    let mut list_all = false;
    let mut all_info = false;

    let mut nonblocking_errs: Vec<ExecuteError> = Vec::new();

    for f in flags.iter() {
        match f.0.as_str() {
            "a" => list_all = true,
            "l" => all_info = true,
            _ => unreachable!(),
        };
    }

    for arg in args.iter() {
        match fs::read_dir(arg) {
            Err(msg) => nonblocking_errs.push(ExecuteError::new(
                &format!("{arg} not found: {msg}"),
                &format!("ls {arg}"),
            )),
            Ok(dir) => {
                let mut buf = String::new();
                let mut n_entries = 0;

                if args.len() > 1 {
                    println!("{arg}:",);
                }

                for maybe_entry in dir.into_iter() {
                    if let Ok(entry) = maybe_entry {
                        let entry_name =
                            entry.file_name().to_string_lossy().into_owned();

                        if entry_name.starts_with('.') && !list_all {
                            continue;
                        }

                        if all_info {
                            match get_all_info(&mut buf, &entry) {
                                Err(e) => {
                                    nonblocking_errs.push(ExecuteError::new(
                                        &format!(
                                            "could not get info about {entry_name}: {e}"),
                                            "ls"));
                                    continue;
                                }
                                Ok(false) => n_entries += 1,
                                Ok(true) => todo!(),
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// On success returns if an entry was a dir.
fn get_all_info<T: Write>(
    mut buf: &mut T,
    entry: &fs::DirEntry,
) -> io::Result<bool> {
    let is_dir = write_entry_type(&mut buf, entry.file_type()?);
    write_metadata(&mut buf, &entry.metadata()?)?;

    Ok(is_dir)
}

fn write_metadata<T: Write>(
    mut buf: &mut T,
    metadata: &fs::Metadata,
) -> io::Result<()> {
    let perms = metadata.permissions().mode();

    let owner_perms = perms & 0o700;
    if owner_perms & 0o400 != 0 {
        write!(&mut buf, "r");
    } else {
        write!(&mut buf, "-");
    }

    if owner_perms & 0o200 != 0 {
        write!(&mut buf, "w");
    } else {
        write!(&mut buf, "-");
    }

    if owner_perms & 0o100 != 0 {
        write!(&mut buf, "x");
    } else {
        write!(&mut buf, "-");
    }

    let group_perms = perms & 0o070;
    if group_perms & 0o040 != 0 {
        write!(&mut buf, "r");
    } else {
        write!(&mut buf, "-");
    }

    if group_perms & 0o020 != 0 {
        write!(&mut buf, "w");
    } else {
        write!(&mut buf, "-");
    }

    if group_perms & 0o010 != 0 {
        write!(&mut buf, "x");
    } else {
        write!(&mut buf, "-");
    }

    let general_perms = perms & 0o007;
    if general_perms & 0o004 != 0 {
        write!(&mut buf, "r");
    } else {
        write!(&mut buf, "-");
    }

    if general_perms & 0o002 != 0 {
        write!(&mut buf, "w");
    } else {
        write!(&mut buf, "-");
    }

    if general_perms & 0o001 != 0 {
        write!(&mut buf, "x");
    } else {
        write!(&mut buf, "-");
    }

    write!(&mut buf, " {} ", metadata.nlink());

    let uid = metadata.uid();
    unsafe {
        let pwd = libc::getpwuid(uid);
        if !pwd.is_null() {
            let owner_name = CStr::from_ptr((*pwd).pw_name).to_string_lossy();

            write!(&mut buf, "{:>6} ", owner_name);
        }
    }

    let gid = metadata.gid();
    unsafe {
        let grp = libc::getgrgid(gid);
        if !grp.is_null() {
            let group_name = CStr::from_ptr((*grp).gr_name).to_string_lossy();

            write!(&mut buf, "{:>6} ", group_name);
        }
    }

    write!(&mut buf, "{:>4} ", metadata.size());

    let date: DateTime<Local> = metadata.modified()?.into();
    write!(&mut buf, "{} ", date.format("%b %e %H:%M"));

    Ok(())
}

/// Returns if an entry was a dir.
fn write_entry_type<T: Write>(mut buf: &mut T, entry_type: FileType) -> bool {
    if entry_type.is_dir() {
        write!(&mut buf, "d");
        return true;
    } else if entry_type.is_file() {
        write!(&mut buf, "-");
    } else if entry_type.is_symlink() {
        write!(&mut buf, "s");
    } else {
        write!(&mut buf, "|");
        todo!()
    }
    false
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
