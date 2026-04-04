use std::{io::ErrorKind, process::Command};

pub fn run(cmd: &str, args: &[&str]) {
    if let Err(err) = Command::new(cmd).args(args).status() {
        match err.kind() {
            ErrorKind::NotFound => eprintln!("{}: command not found", cmd),
            _ => eprintln!("{}: {}", cmd, err),
        }
    }
}
