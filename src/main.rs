use my_shell::{
    builtins, executor,
    parser::{ParsedCommand, parse},
};
use rustyline::DefaultEditor;

fn main() {
    let mut rl = DefaultEditor::new().expect("failed to create editor");
    ctrlc::set_handler(|| {}).expect("failed to set signal handler");
    loop {
        match rl.readline("> ") {
            Ok(input) => {
                let _ = rl.add_history_entry(&input);
                match parse(&input) {
                    ParsedCommand::Empty => continue,
                    ParsedCommand::Exit => break,
                    ParsedCommand::Builtin(command) => {
                        builtins::run(&command);
                    }
                    ParsedCommand::External(command) => {
                        executor::run(&command);
                    }
                    ParsedCommand::Pipeline(commands) => {
                        executor::run_pipeline(&commands);
                    }
                };
            }
            Err(rustyline::error::ReadlineError::Interrupted) => continue,
            Err(_) => break,
        }
    }
}
