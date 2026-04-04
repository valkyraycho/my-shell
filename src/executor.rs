use std::{
    io::ErrorKind,
    process::{Child, ChildStdout, Command, Stdio},
};

use crate::parser::SimpleCommand;

pub fn run(cmd: &str, args: &[&str]) {
    if let Err(err) = Command::new(cmd).args(args).status() {
        match err.kind() {
            ErrorKind::NotFound => eprintln!("{}: command not found", cmd),
            _ => eprintln!("{}: {}", cmd, err),
        }
    }
}

pub fn run_pipeline(commands: &[SimpleCommand]) {
    let mut previous_stdout: Option<ChildStdout> = None;
    let mut children: Vec<Child> = Vec::new();

    for (i, cmd) in commands.iter().enumerate() {
        let is_last = i == commands.len() - 1;
        let mut command = Command::new(cmd.name);
        command.args(&cmd.args);

        if let Some(output) = previous_stdout.take() {
            command.stdin(Stdio::from(output));
        }

        if !is_last {
            command.stdout(Stdio::piped());
        }

        match command.spawn() {
            Ok(mut child) => {
                previous_stdout = child.stdout.take();
                children.push(child);
            }
            Err(e) => {
                eprintln!("{}: {}", cmd.name, e);
                return;
            }
        }
    }

    for child in &mut children {
        let _ = child.wait();
    }
}
