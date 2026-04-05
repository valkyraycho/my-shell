use crate::tokenizer::{Token, tokenize};

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

#[derive(Debug, PartialEq)]
pub enum Operator {
    And,
    Or,
    Then,
}

#[derive(Debug, PartialEq)]
pub struct ChainedCommand {
    pub command: ParsedCommand,
    pub condition: Option<Operator>,
}

pub fn parse(input: &str) -> Vec<ChainedCommand> {
    if input.trim().is_empty() {
        return vec![ChainedCommand {
            command: ParsedCommand::Empty,
            condition: None,
        }];
    }
    let tokens = tokenize(input);
    let mut result = Vec::new();
    let mut current_segment = Vec::new();
    let mut current_condition = None;

    for token in tokens {
        match token {
            Token::And | Token::Or | Token::Semicolon => {
                let parsed_command = parse_segment(current_segment);
                result.push(ChainedCommand {
                    command: parsed_command,
                    condition: current_condition,
                });

                current_condition = Some(match token {
                    Token::And => Operator::And,
                    Token::Or => Operator::Or,
                    Token::Semicolon => Operator::Then,
                    _ => unreachable!(),
                });
                current_segment = Vec::new();
            }
            _ => current_segment.push(token),
        }
    }

    let command = parse_segment(current_segment);
    result.push(ChainedCommand {
        condition: current_condition,
        command,
    });

    result
}

fn parse_segment(segment: Vec<Token>) -> ParsedCommand {
    let commands = segment
        .split(|token| matches!(token, Token::Pipe))
        .collect::<Vec<&[Token]>>();
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

fn parse_single_command(tokens: &[Token]) -> ParsedCommand {
    let cmd = parse_simple_command(tokens);
    match cmd.name.as_str() {
        "exit" => ParsedCommand::Exit,
        name if is_builtin(name) => ParsedCommand::Builtin(cmd),
        _ => ParsedCommand::External(cmd),
    }
}

fn parse_simple_command(tokens: &[Token]) -> SimpleCommand {
    let mut name = None;
    let mut args = Vec::new();
    let mut stdin_redirect = None;
    let mut stdout_redirect = None;
    let mut append_redirect = None;

    let mut tokens_iter = tokens.iter();
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::RedirectIn => {
                if let Some(Token::Word(path)) = tokens_iter.next() {
                    stdin_redirect = Some(expand_tilde(path.clone()));
                }
            }
            Token::RedirectOut => {
                if let Some(Token::Word(path)) = tokens_iter.next() {
                    stdout_redirect = Some(expand_tilde(path.clone()));
                }
            }
            Token::Append => {
                if let Some(Token::Word(path)) = tokens_iter.next() {
                    append_redirect = Some(expand_tilde(path.clone()));
                }
            }
            Token::Word(w) => {
                if name.is_none() {
                    name = Some(w.clone())
                } else {
                    args.push(expand_tilde(w.clone()));
                }
            }
            _ => unreachable!(),
        }
    }
    SimpleCommand {
        name: name.unwrap_or_default(),
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

    fn single(cmd: ParsedCommand) -> Vec<ChainedCommand> {
        vec![ChainedCommand {
            condition: None,
            command: cmd,
        }]
    }

    // === Basic commands ===

    #[test]
    fn test_empty_input() {
        assert_eq!(parse(""), single(ParsedCommand::Empty));
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(parse("   "), single(ParsedCommand::Empty));
    }

    #[test]
    fn test_exit() {
        assert_eq!(parse("exit"), single(ParsedCommand::Exit));
    }

    #[test]
    fn test_builtin_cd_with_no_args() {
        assert_eq!(
            parse("cd"),
            single(ParsedCommand::Builtin(simple("cd", vec![])))
        );
    }

    #[test]
    fn test_builtin_cd_with_args() {
        assert_eq!(
            parse("cd home"),
            single(ParsedCommand::Builtin(simple("cd", vec!["home"])))
        );
    }

    #[test]
    fn test_builtin_pwd_with_no_args() {
        assert_eq!(
            parse("pwd"),
            single(ParsedCommand::Builtin(simple("pwd", vec![])))
        );
    }

    #[test]
    fn test_builtin_pwd_with_args() {
        assert_eq!(
            parse("pwd home"),
            single(ParsedCommand::Builtin(simple("pwd", vec!["home"])))
        );
    }

    #[test]
    fn test_external_command() {
        assert_eq!(
            parse("ls"),
            single(ParsedCommand::External(simple("ls", vec![])))
        );
    }

    #[test]
    fn test_external_command_with_args() {
        assert_eq!(
            parse("ls -la /tmp"),
            single(ParsedCommand::External(simple("ls", vec!["-la", "/tmp"])))
        );
    }

    // === Pipes ===

    #[test]
    fn test_two_command_pipeline() {
        assert_eq!(
            parse("ls | grep foo"),
            single(ParsedCommand::Pipeline(vec![
                simple("ls", vec![]),
                simple("grep", vec!["foo"]),
            ]))
        );
    }

    #[test]
    fn test_three_command_pipeline() {
        assert_eq!(
            parse("ls -la | grep foo | wc -l"),
            single(ParsedCommand::Pipeline(vec![
                simple("ls", vec!["-la"]),
                simple("grep", vec!["foo"]),
                simple("wc", vec!["-l"]),
            ]))
        );
    }

    #[test]
    fn test_pipeline_with_extra_whitespace() {
        assert_eq!(
            parse("ls   |   grep   foo"),
            single(ParsedCommand::Pipeline(vec![
                simple("ls", vec![]),
                simple("grep", vec!["foo"]),
            ]))
        );
    }

    // === I/O Redirection ===

    #[test]
    fn test_stdout_redirect() {
        assert_eq!(
            parse("echo hello > out.txt"),
            single(ParsedCommand::External(SimpleCommand {
                name: "echo".to_string(),
                args: vec!["hello".to_string()],
                stdin_redirect: None,
                stdout_redirect: Some("out.txt".to_string()),
                append_redirect: None,
            }))
        );
    }

    #[test]
    fn test_stdin_redirect() {
        assert_eq!(
            parse("cat < input.txt"),
            single(ParsedCommand::External(SimpleCommand {
                name: "cat".to_string(),
                args: vec![],
                stdin_redirect: Some("input.txt".to_string()),
                stdout_redirect: None,
                append_redirect: None,
            }))
        );
    }

    #[test]
    fn test_append_redirect() {
        assert_eq!(
            parse("echo hello >> out.txt"),
            single(ParsedCommand::External(SimpleCommand {
                name: "echo".to_string(),
                args: vec!["hello".to_string()],
                stdin_redirect: None,
                stdout_redirect: None,
                append_redirect: Some("out.txt".to_string()),
            }))
        );
    }

    #[test]
    fn test_stdin_and_stdout_redirect() {
        assert_eq!(
            parse("grep foo < input.txt > output.txt"),
            single(ParsedCommand::External(SimpleCommand {
                name: "grep".to_string(),
                args: vec!["foo".to_string()],
                stdin_redirect: Some("input.txt".to_string()),
                stdout_redirect: Some("output.txt".to_string()),
                append_redirect: None,
            }))
        );
    }

    #[test]
    fn test_redirect_with_multiple_args() {
        assert_eq!(
            parse("ls -la -h > out.txt"),
            single(ParsedCommand::External(SimpleCommand {
                name: "ls".to_string(),
                args: vec!["-la".to_string(), "-h".to_string()],
                stdin_redirect: None,
                stdout_redirect: Some("out.txt".to_string()),
                append_redirect: None,
            }))
        );
    }

    #[test]
    fn test_redirect_in_pipeline() {
        assert_eq!(
            parse("cat < input.txt | grep foo > output.txt"),
            single(ParsedCommand::Pipeline(vec![
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
            ]))
        );
    }

    // === Quoted Arguments ===

    #[test]
    fn test_double_quoted_argument() {
        assert_eq!(
            parse("echo \"hello world\""),
            single(ParsedCommand::External(simple("echo", vec!["hello world"])))
        );
    }

    #[test]
    fn test_single_quoted_argument() {
        assert_eq!(
            parse("echo 'hello world'"),
            single(ParsedCommand::External(simple("echo", vec!["hello world"])))
        );
    }

    #[test]
    fn test_multiple_quoted_arguments() {
        assert_eq!(
            parse("echo \"hello\" \"world\""),
            single(ParsedCommand::External(simple(
                "echo",
                vec!["hello", "world"]
            )))
        );
    }

    #[test]
    fn test_mixed_quoted_and_unquoted() {
        assert_eq!(
            parse("echo hello \"big world\""),
            single(ParsedCommand::External(simple(
                "echo",
                vec!["hello", "big world"]
            )))
        );
    }

    #[test]
    fn test_single_quotes_preserve_double_quotes() {
        assert_eq!(
            parse("echo 'hello \"world\"'"),
            single(ParsedCommand::External(simple(
                "echo",
                vec!["hello \"world\""]
            )))
        );
    }

    #[test]
    fn test_double_quotes_preserve_single_quotes() {
        assert_eq!(
            parse("echo \"hello 'world'\""),
            single(ParsedCommand::External(simple(
                "echo",
                vec!["hello 'world'"]
            )))
        );
    }

    #[test]
    fn test_quoted_redirect_filename() {
        assert_eq!(
            parse("echo hello > \"my file.txt\""),
            single(ParsedCommand::External(SimpleCommand {
                name: "echo".to_string(),
                args: vec!["hello".to_string()],
                stdin_redirect: None,
                stdout_redirect: Some("my file.txt".to_string()),
                append_redirect: None,
            }))
        );
    }

    // === Tilde Expansion ===

    #[test]
    fn test_tilde_as_argument() {
        let home = std::env::var("HOME").unwrap();
        assert_eq!(
            parse("cd ~"),
            single(ParsedCommand::Builtin(simple("cd", vec![&home])))
        );
    }

    #[test]
    fn test_tilde_with_path() {
        let home = std::env::var("HOME").unwrap();
        let expected = format!("{}/projects", home);
        assert_eq!(
            parse("ls ~/projects"),
            single(ParsedCommand::External(simple("ls", vec![&expected])))
        );
    }

    #[test]
    fn test_tilde_in_redirect() {
        let home = std::env::var("HOME").unwrap();
        let expected = format!("{}/out.txt", home);
        assert_eq!(
            parse("echo hello > ~/out.txt"),
            single(ParsedCommand::External(SimpleCommand {
                name: "echo".to_string(),
                args: vec!["hello".to_string()],
                stdin_redirect: None,
                stdout_redirect: Some(expected),
                append_redirect: None,
            }))
        );
    }

    #[test]
    fn test_tilde_not_at_start() {
        assert_eq!(
            parse("echo foo~bar"),
            single(ParsedCommand::External(simple("echo", vec!["foo~bar"])))
        );
    }

    // === Chained Commands ===

    #[test]
    fn test_and_operator() {
        assert_eq!(
            parse("echo hello && echo world"),
            vec![
                ChainedCommand {
                    condition: None,
                    command: ParsedCommand::External(simple("echo", vec!["hello"])),
                },
                ChainedCommand {
                    condition: Some(Operator::And),
                    command: ParsedCommand::External(simple("echo", vec!["world"])),
                },
            ]
        );
    }

    #[test]
    fn test_or_operator() {
        assert_eq!(
            parse("ls /fake || echo fallback"),
            vec![
                ChainedCommand {
                    condition: None,
                    command: ParsedCommand::External(simple("ls", vec!["/fake"])),
                },
                ChainedCommand {
                    condition: Some(Operator::Or),
                    command: ParsedCommand::External(simple("echo", vec!["fallback"])),
                },
            ]
        );
    }

    #[test]
    fn test_semicolon() {
        assert_eq!(
            parse("echo hello ; echo world"),
            vec![
                ChainedCommand {
                    condition: None,
                    command: ParsedCommand::External(simple("echo", vec!["hello"])),
                },
                ChainedCommand {
                    condition: Some(Operator::Then),
                    command: ParsedCommand::External(simple("echo", vec!["world"])),
                },
            ]
        );
    }

    #[test]
    fn test_chain_with_pipeline() {
        assert_eq!(
            parse("echo hello && ls | grep foo"),
            vec![
                ChainedCommand {
                    condition: None,
                    command: ParsedCommand::External(simple("echo", vec!["hello"])),
                },
                ChainedCommand {
                    condition: Some(Operator::And),
                    command: ParsedCommand::Pipeline(vec![
                        simple("ls", vec![]),
                        simple("grep", vec!["foo"]),
                    ]),
                },
            ]
        );
    }

    #[test]
    fn test_three_way_chain() {
        assert_eq!(
            parse("cmd1 && cmd2 || cmd3"),
            vec![
                ChainedCommand {
                    condition: None,
                    command: ParsedCommand::External(simple("cmd1", vec![])),
                },
                ChainedCommand {
                    condition: Some(Operator::And),
                    command: ParsedCommand::External(simple("cmd2", vec![])),
                },
                ChainedCommand {
                    condition: Some(Operator::Or),
                    command: ParsedCommand::External(simple("cmd3", vec![])),
                },
            ]
        );
    }
}
