#[derive(Debug, PartialEq)]
pub enum ParsedCommand<'a> {
    Empty,
    Exit,
    Builtin(SimpleCommand<'a>),
    External(SimpleCommand<'a>),
    Pipeline(Vec<SimpleCommand<'a>>),
}

#[derive(Debug, PartialEq)]
pub struct SimpleCommand<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}

pub fn parse(input: &str) -> ParsedCommand<'_> {
    if input.trim().is_empty() {
        return ParsedCommand::Empty;
    }

    let commands = input.split('|').collect::<Vec<&str>>();
    if commands.len() == 1 {
        return parse_single_command(commands[0]);
    }

    ParsedCommand::Pipeline(
        commands
            .iter()
            .map(|cmd| parse_simple_command(cmd))
            .collect(),
    )
}

fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "cd" | "pwd")
}

fn parse_single_command(input: &str) -> ParsedCommand<'_> {
    let cmd = parse_simple_command(input);
    match cmd.name {
        "exit" => ParsedCommand::Exit,
        name if is_builtin(name) => ParsedCommand::Builtin(cmd),
        _ => ParsedCommand::External(cmd),
    }
}

fn parse_simple_command(input: &str) -> SimpleCommand<'_> {
    let mut elements = input.split_whitespace();
    let (cmd, args) = (elements.next().unwrap(), elements.collect::<Vec<&str>>());
    SimpleCommand { name: cmd, args }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(parse(""), ParsedCommand::Empty);
    }

    #[test]
    fn test_whietespace() {
        assert_eq!(parse("   "), ParsedCommand::Empty);
    }

    #[test]
    fn test_exit() {
        assert_eq!(parse("exit"), ParsedCommand::Exit);
    }

    #[test]
    fn test_builtin_cd_with_no_args() {
        assert_eq!(
            parse("cd"),
            ParsedCommand::Builtin(SimpleCommand {
                name: "cd",
                args: vec![]
            })
        );
    }

    #[test]
    fn test_builtin_cd_with_args() {
        assert_eq!(
            parse("cd home"),
            ParsedCommand::Builtin(SimpleCommand {
                name: "cd",
                args: vec!["home"]
            })
        );
    }

    #[test]
    fn test_builtin_pwd_with_no_args() {
        assert_eq!(
            parse("pwd"),
            ParsedCommand::Builtin(SimpleCommand {
                name: "pwd",
                args: vec![]
            })
        );
    }

    #[test]
    fn test_builtin_pwd_with_args() {
        assert_eq!(
            parse("pwd home"),
            ParsedCommand::Builtin(SimpleCommand {
                name: "pwd",
                args: vec!["home"]
            })
        );
    }

    #[test]
    fn test_external_command() {
        assert_eq!(
            parse("ls"),
            ParsedCommand::External(SimpleCommand {
                name: "ls",
                args: vec![]
            })
        );
    }

    #[test]
    fn test_external_command_with_args() {
        assert_eq!(
            parse("ls -la /tmp"),
            ParsedCommand::External(SimpleCommand {
                name: "ls",
                args: vec!["-la", "/tmp"]
            })
        );
    }

    // === Pipes ===

    #[test]
    fn test_two_command_pipeline() {
        assert_eq!(
            parse("ls | grep foo"),
            ParsedCommand::Pipeline(vec![
                SimpleCommand {
                    name: "ls",
                    args: vec![]
                },
                SimpleCommand {
                    name: "grep",
                    args: vec!["foo"]
                },
            ])
        );
    }

    #[test]
    fn test_three_command_pipeline() {
        assert_eq!(
            parse("ls -la | grep foo | wc -l"),
            ParsedCommand::Pipeline(vec![
                SimpleCommand {
                    name: "ls",
                    args: vec!["-la"]
                },
                SimpleCommand {
                    name: "grep",
                    args: vec!["foo"]
                },
                SimpleCommand {
                    name: "wc",
                    args: vec!["-l"]
                },
            ])
        );
    }

    #[test]
    fn test_pipeline_with_extra_whitespace() {
        assert_eq!(
            parse("ls   |   grep   foo"),
            ParsedCommand::Pipeline(vec![
                SimpleCommand {
                    name: "ls",
                    args: vec![]
                },
                SimpleCommand {
                    name: "grep",
                    args: vec!["foo"]
                },
            ])
        );
    }
}
