pub enum ParsedCommand<'a> {
    Empty,
    Exit,
    Builtin { name: &'a str, args: Vec<&'a str> },
    External { name: &'a str, args: Vec<&'a str> },
}

pub fn parse(input: &str) -> ParsedCommand<'_> {
    let mut elements = input.split_whitespace();
    let (command, arguments) = (elements.next(), elements.collect::<Vec<&str>>());

    match command {
        None => ParsedCommand::Empty,
        Some("exit") => ParsedCommand::Exit,
        Some(cmd) if is_builtin(cmd) => ParsedCommand::Builtin {
            name: cmd,
            args: arguments,
        },
        Some(cmd) => ParsedCommand::External {
            name: cmd,
            args: arguments,
        },
    }
}

fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "cd" | "pwd")
}
