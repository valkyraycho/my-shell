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
    pub stdin_redirect: Option<&'a str>,
    pub stdout_redirect: Option<&'a str>,
    pub append_redirect: Option<&'a str>,
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
    SimpleCommand {
        name: cmd,
        args,
        stdin_redirect: None,
        stdout_redirect: None,
        append_redirect: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple<'a>(name: &'a str, args: Vec<&'a str>) -> SimpleCommand<'a> {
        SimpleCommand {
            name,
            args,
            stdin_redirect: None,
            stdout_redirect: None,
            append_redirect: None,
        }
    }

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
        assert_eq!(parse("cd"), ParsedCommand::Builtin(simple("cd", vec![])));
    }

    #[test]
    fn test_builtin_cd_with_args() {
        assert_eq!(
            parse("cd home"),
            ParsedCommand::Builtin(simple("cd", vec!["home"]))
        );
    }

    #[test]
    fn test_builtin_pwd_with_no_args() {
        assert_eq!(parse("pwd"), ParsedCommand::Builtin(simple("pwd", vec![])));
    }

    #[test]
    fn test_builtin_pwd_with_args() {
        assert_eq!(
            parse("pwd home"),
            ParsedCommand::Builtin(simple("pwd", vec!["home"]))
        );
    }

    #[test]
    fn test_external_command() {
        assert_eq!(parse("ls"), ParsedCommand::External(simple("ls", vec![])));
    }

    #[test]
    fn test_external_command_with_args() {
        assert_eq!(
            parse("ls -la /tmp"),
            ParsedCommand::External(simple("ls", vec!["-la", "/tmp"]))
        );
    }

    // === Pipes ===

    #[test]
    fn test_two_command_pipeline() {
        assert_eq!(
            parse("ls | grep foo"),
            ParsedCommand::Pipeline(vec![simple("ls", vec![]), simple("grep", vec!["foo"]),])
        );
    }

    #[test]
    fn test_three_command_pipeline() {
        assert_eq!(
            parse("ls -la | grep foo | wc -l"),
            ParsedCommand::Pipeline(vec![
                simple("ls", vec!["-la"]),
                simple("grep", vec!["foo"]),
                simple("wc", vec!["-l"]),
            ])
        );
    }

    #[test]
    fn test_pipeline_with_extra_whitespace() {
        assert_eq!(
            parse("ls   |   grep   foo"),
            ParsedCommand::Pipeline(vec![simple("ls", vec![]), simple("grep", vec!["foo"]),])
        );
    }

    // === I/O Redirection ===

    #[test]
    fn test_stdout_redirect() {
        assert_eq!(
            parse("echo hello > out.txt"),
            ParsedCommand::External(SimpleCommand {
                name: "echo",
                args: vec!["hello"],
                stdin_redirect: None,
                stdout_redirect: Some("out.txt"),
                append_redirect: None,
            })
        );
    }

    #[test]
    fn test_stdin_redirect() {
        assert_eq!(
            parse("cat < input.txt"),
            ParsedCommand::External(SimpleCommand {
                name: "cat",
                args: vec![],
                stdin_redirect: Some("input.txt"),
                stdout_redirect: None,
                append_redirect: None,
            })
        );
    }

    #[test]
    fn test_append_redirect() {
        assert_eq!(
            parse("echo hello >> out.txt"),
            ParsedCommand::External(SimpleCommand {
                name: "echo",
                args: vec!["hello"],
                stdin_redirect: None,
                stdout_redirect: None,
                append_redirect: Some("out.txt"),
            })
        );
    }

    #[test]
    fn test_stdin_and_stdout_redirect() {
        assert_eq!(
            parse("grep foo < input.txt > output.txt"),
            ParsedCommand::External(SimpleCommand {
                name: "grep",
                args: vec!["foo"],
                stdin_redirect: Some("input.txt"),
                stdout_redirect: Some("output.txt"),
                append_redirect: None,
            })
        );
    }

    #[test]
    fn test_redirect_with_multiple_args() {
        assert_eq!(
            parse("ls -la -h > out.txt"),
            ParsedCommand::External(SimpleCommand {
                name: "ls",
                args: vec!["-la", "-h"],
                stdin_redirect: None,
                stdout_redirect: Some("out.txt"),
                append_redirect: None,
            })
        );
    }

    #[test]
    fn test_redirect_in_pipeline() {
        assert_eq!(
            parse("cat < input.txt | grep foo > output.txt"),
            ParsedCommand::Pipeline(vec![
                SimpleCommand {
                    name: "cat",
                    args: vec![],
                    stdin_redirect: Some("input.txt"),
                    stdout_redirect: None,
                    append_redirect: None,
                },
                SimpleCommand {
                    name: "grep",
                    args: vec!["foo"],
                    stdin_redirect: None,
                    stdout_redirect: Some("output.txt"),
                    append_redirect: None,
                },
            ])
        );
    }
}
