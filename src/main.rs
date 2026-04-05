use my_shell::{
    builtins, executor,
    parser::{ParsedCommand, parse},
};
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new().expect("failed to create editor");
    let history_file = format!("{}/.my_shell_history", std::env::var("HOME").unwrap());
    let _ = rl.load_history(&history_file);

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

    rl.save_history(&history_file)?;
    Ok(())
}
