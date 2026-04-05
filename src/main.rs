use std::{
    io::{Write, stdin, stdout},
    sync::atomic::{AtomicBool, Ordering},
};

use my_shell::{
    builtins, executor,
    parser::{ParsedCommand, parse},
};

static RUNNING_COMMAND: AtomicBool = AtomicBool::new(false);

fn main() {
    ctrlc::set_handler(|| {
        println!();
        if !RUNNING_COMMAND.load(Ordering::Relaxed) {
            print!("> ");
            stdout().flush().expect("failed to flush");
        }
    })
    .expect("failed to set signal handler");
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
                RUNNING_COMMAND.store(true, Ordering::Relaxed);
                builtins::run(&command);
                RUNNING_COMMAND.store(false, Ordering::Relaxed);
            }
            ParsedCommand::External(command) => {
                RUNNING_COMMAND.store(true, Ordering::Relaxed);
                executor::run(&command);
                RUNNING_COMMAND.store(false, Ordering::Relaxed);
            }
            ParsedCommand::Pipeline(commands) => {
                RUNNING_COMMAND.store(true, Ordering::Relaxed);
                executor::run_pipeline(&commands);
                RUNNING_COMMAND.store(false, Ordering::Relaxed);
            }
        };
    }
}
