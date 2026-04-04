use std::io::{Write, stdin, stdout};

use my_shell::{
    builtins, executor,
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
            ParsedCommand::Builtin(command) => {
                builtins::run(&command);
            }
            ParsedCommand::External(command) => {
                executor::run(&command);
            }
            ParsedCommand::Pipeline(commands) => executor::run_pipeline(&commands),
        };
    }
}
