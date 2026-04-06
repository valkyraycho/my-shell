use std::env::{self, current_dir, set_current_dir};

use crate::parser::SimpleCommand;

pub fn run(command: &SimpleCommand) -> i32 {
    match command.name.as_str() {
        "cd" => match command.args.first() {
            None => {
                if let Ok(home_dir) = env::var("HOME") {
                    if let Err(e) = set_current_dir(&home_dir) {
                        eprintln!("cd: {}: {}", home_dir, e);
                        return 1;
                    }
                    0
                } else {
                    eprintln!("$HOME is not set");
                    1
                }
            }
            Some(path) => {
                if let Err(e) = set_current_dir(path) {
                    eprintln!("cd: {}: {}", path, e);
                    return 1;
                }
                0
            }
        },
        "pwd" => {
            if !command.args.is_empty() {
                eprintln!("pwd: too many arguments");
                1
            } else {
                match current_dir() {
                    Ok(cur_dir) => {
                        println!("{}", cur_dir.display());
                        0
                    }
                    Err(e) => {
                        eprintln!("pwd: {}", e);
                        1
                    }
                }
            }
        }
        "export" => {
            if let Some(arg) = command.args.first() {
                if let Some((key, value)) = arg.split_once('=') {
                    unsafe {
                        std::env::set_var(key, value);
                    };
                } else {
                    eprintln!("export: invalid format, expected KEY=VALUE");
                    return 1;
                }
            } else {
                eprintln!("export: missing argument");
                return 1;
            }
            0
        }
        _ => unimplemented!(),
    }
}
