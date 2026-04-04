use std::io::{Write, stdin, stdout};

use my_shell::{
    builtins, executor,
    parser::{ParsedCommand, SimpleCommand, parse},
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
            ParsedCommand::Builtin(SimpleCommand { name, args, .. }) => {
                builtins::run(name, &args);
            }
            ParsedCommand::External(SimpleCommand { name, args, .. }) => {
                executor::run(name, &args);
            }
            ParsedCommand::Pipeline(commands) => executor::run_pipeline(&commands),
        };
    }
}
