use std::{
    io::{ErrorKind, Write, stdin, stdout},
    process::Command,
};

use my_shell::{
    builtins,
    parser::{ParsedCommand, parse},
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

        match parse(&input) {
            ParsedCommand::Empty => continue,
            ParsedCommand::Exit => break,
            ParsedCommand::Builtin { name, args } => {
                builtins::run(name, &args);
            }
            ParsedCommand::External { name: cmd, args } => {
                if let Err(err) = Command::new(cmd).args(&args).status() {
                    match err.kind() {
                        ErrorKind::NotFound => eprintln!("{}: command not found", cmd),
                        _ => eprintln!("{}: {}", cmd, err),
                    }
                }
            }
        };
    }
}
