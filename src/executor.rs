use std::{
    fs::File,
    io::ErrorKind,
    process::{Child, ChildStdout, Command, Stdio},
};

use crate::parser::SimpleCommand;

pub fn run(command: &SimpleCommand) {
    let mut cmd = Command::new(&command.name);
    cmd.args(&command.args);

    if let Err(e) = apply_redirects(&mut cmd, command) {
        eprintln!("{}", e);
        return;
    }

    if let Err(err) = cmd.status() {
        match err.kind() {
            ErrorKind::NotFound => eprintln!("{}: command not found", command.name),
            _ => eprintln!("{}: {}", command.name, err),
        }
    }
}

fn apply_redirects(cmd: &mut Command, command: &SimpleCommand) -> Result<(), String> {
    if let Some(path) = &command.stdin_redirect {
        let file = File::open(path).map_err(|e| format!("{}: {}", path, e))?;
        cmd.stdin(Stdio::from(file));
    }
    if let Some(path) = &command.stdout_redirect {
        let file = File::create(path).map_err(|e| format!("{}: {}", path, e))?;
        cmd.stdout(Stdio::from(file));
    }
    if let Some(path) = &command.append_redirect {
        let file = File::options()
            .append(true)
            .create(true)
            .open(path)
            .map_err(|e| format!("{}: {}", path, e))?;
        cmd.stdout(Stdio::from(file));
    }
    Ok(())
}
pub fn run_pipeline(commands: &[SimpleCommand]) {
    let mut previous_stdout: Option<ChildStdout> = None;
    let mut children: Vec<Child> = Vec::new();

    for (i, command) in commands.iter().enumerate() {
        let is_last = i == commands.len() - 1;
        let mut cmd = Command::new(&command.name);
        cmd.args(&command.args);

        if let Some(output) = previous_stdout.take() {
            cmd.stdin(Stdio::from(output));
        }

        if !is_last {
            cmd.stdout(Stdio::piped());
        }

        if let Err(e) = apply_redirects(&mut cmd, command) {
            eprintln!("{}", e);
            return;
        }

        match cmd.spawn() {
            Ok(mut child) => {
                previous_stdout = child.stdout.take();
                children.push(child);
            }
            Err(e) => {
                eprintln!("{}: {}", command.name, e);
                return;
            }
        }
    }

    for child in &mut children {
        let _ = child.wait();
    }
}
