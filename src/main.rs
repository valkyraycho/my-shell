use std::{
    env::{self, current_dir, set_current_dir},
    io::{ErrorKind, Write, stdin, stdout},
    process::Command,
};

fn main() {
    let mut input = String::new();
    loop {
        input.clear();
        print!("> ");
        stdout().flush().expect("failed to flush");
        if stdin().read_line(&mut input).expect("failed to read line") == 0 {
            break;
        };

        let mut elements = input.split_whitespace();
        let (command, arguments) = (elements.next(), elements.collect::<Vec<&str>>());

        match command {
            None => continue,
            Some("cd") => match arguments.first() {
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
            Some("pwd") => match current_dir() {
                Ok(cur_dir) => println!("{}", cur_dir.display()),
                Err(e) => eprintln!("pwd: {}", e),
            },
            Some("exit") => break,
            _ => {
                let cmd = command.unwrap();
                if let Err(err) = Command::new(cmd).args(&arguments).status() {
                    match err.kind() {
                        ErrorKind::NotFound => eprintln!("{}: command not found", cmd),
                        _ => eprintln!("{}: {}", cmd, err),
                    }
                }
            }
        };
    }
}
