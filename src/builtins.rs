use std::env::{self, current_dir, set_current_dir};

pub fn run(cmd: &str, args: &[&str]) {
    match cmd {
        "cd" => match args.first() {
            None => {
                if let Ok(home_dir) = env::var("HOME") {
                    if let Err(e) = set_current_dir(&home_dir) {
                        eprintln!("cd: {}: {}", home_dir, e)
                    }
                } else {
                    eprintln!("$HOME is not set")
                }
            }
            Some(path) => {
                if let Err(e) = set_current_dir(path) {
                    eprintln!("cd: {}: {}", path, e)
                }
            }
        },
        "pwd" => {
            if !args.is_empty() {
                eprintln!("pwd: too many arguments");
            } else {
                match current_dir() {
                    Ok(cur_dir) => println!("{}", cur_dir.display()),
                    Err(e) => eprintln!("pwd: {}", e),
                }
            }
        }
        _ => unimplemented!(),
    }
}
