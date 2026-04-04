use std::{
    env::{self, current_dir, set_current_dir},
    io::{Write, stdin, stdout},
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
                None => set_current_dir(env::var("HOME").expect("HOME is not set"))
                    .expect("failed to change directory"),
                Some(path) => set_current_dir(path).expect("failed to change directory"),
            },
            Some("pwd") => {
                println!("{}", current_dir().expect("failed to get cwd").display());
            }
            Some("exit") => break,
            _ => {
                Command::new(command.unwrap())
                    .args(&arguments)
                    .status()
                    .expect("failed");
            }
        };
    }
}
