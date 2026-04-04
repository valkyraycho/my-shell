pub enum ParsedCommand<'a> {
    Empty,
    Exit,
    Builtin(SimpleCommand<'a>),
    External(SimpleCommand<'a>),
    Pipeline(Vec<SimpleCommand<'a>>),
}

pub struct SimpleCommand<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}

pub fn parse(input: &str) -> ParsedCommand<'_> {
    let mut elements = input.split_whitespace();
    let (command, arguments) = (elements.next(), elements.collect::<Vec<&str>>());

    match command {
        None => ParsedCommand::Empty,
        Some("exit") => ParsedCommand::Exit,
        Some(cmd) if is_builtin(cmd) => ParsedCommand::Builtin(SimpleCommand {
            name: cmd,
            args: arguments,
        }),
        Some(cmd) => ParsedCommand::External(SimpleCommand {
            name: cmd,
            args: arguments,
        }),
    }
}

fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "cd" | "pwd")
}
