use crate::tokenizer::tokenize;

#[derive(Debug, PartialEq)]
pub enum ParsedCommand {
    Empty,
    Exit,
    Builtin(SimpleCommand),
    External(SimpleCommand),
    Pipeline(Vec<SimpleCommand>),
}

#[derive(Debug, PartialEq)]
pub struct SimpleCommand {
    pub name: String,
    pub args: Vec<String>,
    pub stdin_redirect: Option<String>,
    pub stdout_redirect: Option<String>,
    pub append_redirect: Option<String>,
}

pub fn parse(input: &str) -> ParsedCommand {
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

fn parse_single_command(input: &str) -> ParsedCommand {
    let cmd = parse_simple_command(input);
    match cmd.name.as_str() {
        "exit" => ParsedCommand::Exit,
        name if is_builtin(name) => ParsedCommand::Builtin(cmd),
        _ => ParsedCommand::External(cmd),
    }
}

fn parse_simple_command(input: &str) -> SimpleCommand {
    let mut tokens = tokenize(input).into_iter();

    let mut args = Vec::new();
    let name = tokens.next().unwrap();

    let mut stdin_redirect = None;
    let mut stdout_redirect = None;
    let mut append_redirect = None;

    while let Some(token) = tokens.next() {
        match token.as_str() {
            ">>" => append_redirect = tokens.next().map(expand_tilde),
            "<" => stdin_redirect = tokens.next().map(expand_tilde),
            ">" => stdout_redirect = tokens.next().map(expand_tilde),
            _ => args.push(expand_tilde(token)),
        }
    }

    SimpleCommand {
        name,
        args,
        stdin_redirect,
        stdout_redirect,
        append_redirect,
    }
}

fn expand_tilde(token: String) -> String {
    if token == "~" || token.starts_with("~/") {
        let home = std::env::var("HOME").unwrap();
        token.replacen("~", &home, 1)
    } else {
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple(name: &str, args: Vec<&str>) -> SimpleCommand {
        SimpleCommand {
            name: name.to_string(),
            args: args.into_iter().map(String::from).collect(),
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
                name: "echo".to_string(),
                args: vec![String::from("hello")],
                stdin_redirect: None,
                stdout_redirect: Some("out.txt".to_string()),
                append_redirect: None,
            })
        );
    }

    #[test]
    fn test_stdin_redirect() {
        assert_eq!(
            parse("cat < input.txt"),
            ParsedCommand::External(SimpleCommand {
                name: "cat".to_string(),
                args: vec![],
                stdin_redirect: Some("input.txt".to_string()),
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
                name: "echo".to_string(),
                args: vec!["hello".to_string()],
                stdin_redirect: None,
                stdout_redirect: None,
                append_redirect: Some("out.txt".to_string()),
            })
        );
    }

    #[test]
    fn test_stdin_and_stdout_redirect() {
        assert_eq!(
            parse("grep foo < input.txt > output.txt"),
            ParsedCommand::External(SimpleCommand {
                name: "grep".to_string(),
                args: vec!["foo".to_string()],
                stdin_redirect: Some("input.txt".to_string()),
                stdout_redirect: Some("output.txt".to_string()),
                append_redirect: None,
            })
        );
    }

    #[test]
    fn test_redirect_with_multiple_args() {
        assert_eq!(
            parse("ls -la -h > out.txt"),
            ParsedCommand::External(SimpleCommand {
                name: "ls".to_string(),
                args: vec!["-la".to_string(), "-h".to_string()],
                stdin_redirect: None,
                stdout_redirect: Some("out.txt".to_string()),
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
                    name: "cat".to_string(),
                    args: vec![],
                    stdin_redirect: Some("input.txt".to_string()),
                    stdout_redirect: None,
                    append_redirect: None,
                },
                SimpleCommand {
                    name: "grep".to_string(),
                    args: vec!["foo".to_string()],
                    stdin_redirect: None,
                    stdout_redirect: Some("output.txt".to_string()),
                    append_redirect: None,
                },
            ])
        );
    }

    // === Quoted Arguments ===

    #[test]
    fn test_double_quoted_argument() {
        assert_eq!(
            parse("echo \"hello world\""),
            ParsedCommand::External(simple("echo", vec!["hello world"]))
        );
    }

    #[test]
    fn test_single_quoted_argument() {
        assert_eq!(
            parse("echo 'hello world'"),
            ParsedCommand::External(simple("echo", vec!["hello world"]))
        );
    }

    #[test]
    fn test_multiple_quoted_arguments() {
        assert_eq!(
            parse("echo \"hello\" \"world\""),
            ParsedCommand::External(simple("echo", vec!["hello", "world"]))
        );
    }

    #[test]
    fn test_mixed_quoted_and_unquoted() {
        assert_eq!(
            parse("echo hello \"big world\""),
            ParsedCommand::External(simple("echo", vec!["hello", "big world"]))
        );
    }

    #[test]
    fn test_single_quotes_preserve_double_quotes() {
        assert_eq!(
            parse("echo 'hello \"world\"'"),
            ParsedCommand::External(simple("echo", vec!["hello \"world\""]))
        );
    }

    #[test]
    fn test_double_quotes_preserve_single_quotes() {
        assert_eq!(
            parse("echo \"hello 'world'\""),
            ParsedCommand::External(simple("echo", vec!["hello 'world'"]))
        );
    }

    #[test]
    fn test_quoted_redirect_filename() {
        assert_eq!(
            parse("echo hello > \"my file.txt\""),
            ParsedCommand::External(SimpleCommand {
                name: "echo".to_string(),
                args: vec!["hello".to_string()],
                stdin_redirect: None,
                stdout_redirect: Some("my file.txt".to_string()),
                append_redirect: None,
            })
        );
    }

    // === Tilde Expansion ===

    #[test]
    fn test_tilde_as_argument() {
        let home = std::env::var("HOME").unwrap();
        assert_eq!(
            parse("cd ~"),
            ParsedCommand::Builtin(simple("cd", vec![&home]))
        );
    }

    #[test]
    fn test_tilde_with_path() {
        let home = std::env::var("HOME").unwrap();
        let expected = format!("{}/projects", home);
        assert_eq!(
            parse("ls ~/projects"),
            ParsedCommand::External(simple("ls", vec![&expected]))
        );
    }

    #[test]
    fn test_tilde_in_redirect() {
        let home = std::env::var("HOME").unwrap();
        let expected = format!("{}/out.txt", home);
        assert_eq!(
            parse("echo hello > ~/out.txt"),
            ParsedCommand::External(SimpleCommand {
                name: "echo".to_string(),
                args: vec!["hello".to_string()],
                stdin_redirect: None,
                stdout_redirect: Some(expected),
                append_redirect: None,
            })
        );
    }

    #[test]
    fn test_tilde_not_at_start() {
        assert_eq!(
            parse("echo foo~bar"),
            ParsedCommand::External(simple("echo", vec!["foo~bar"]))
        );
    }
}
