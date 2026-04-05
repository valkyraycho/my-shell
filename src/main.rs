use my_shell::{
    builtins, executor,
    parser::{Operator, ParsedCommand, parse},
};
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new().expect("failed to create editor");
    let history_file = format!("{}/.my_shell_history", std::env::var("HOME").unwrap());
    let _ = rl.load_history(&history_file);
    let mut last_status = 0;

    ctrlc::set_handler(|| {}).expect("failed to set signal handler");
    'outer: loop {
        match rl.readline("> ") {
            Ok(input) => {
                let _ = rl.add_history_entry(&input);
                for chained in parse(&input) {
                    let should_run = match &chained.condition {
                        None | Some(Operator::Then) => true,
                        Some(Operator::And) => last_status == 0,
                        Some(Operator::Or) => last_status != 0,
                    };
                    if !should_run {
                        continue;
                    }
                    match chained.command {
                        ParsedCommand::Empty => {}
                        ParsedCommand::Exit => break 'outer,
                        ParsedCommand::Builtin(cmd) => last_status = builtins::run(&cmd),
                        ParsedCommand::External(cmd) => last_status = executor::run(&cmd),
                        ParsedCommand::Pipeline(cmds) => {
                            last_status = executor::run_pipeline(&cmds)
                        }
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => continue,
            Err(_) => break,
        }
    }

    rl.save_history(&history_file)?;
    Ok(())
}
