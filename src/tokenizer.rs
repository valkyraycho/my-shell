#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    Pipe,
    And,
    Or,
    Semicolon,
    RedirectIn,
    RedirectOut,
    Append,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_word = String::new();
    let mut quote_char: Option<char> = None;

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match (c, quote_char) {
            // opening quote
            ('"' | '\'', None) => quote_char = Some(c),
            // closing the matching quote
            (q, Some(open)) if q == open => quote_char = None,
            // spaces outside a quote
            (c, None) if c.is_ascii_whitespace() => {
                flush_word(&mut current_word, &mut tokens);
            }
            ('|', None) => {
                flush_word(&mut current_word, &mut tokens);
                if chars.peek() == Some(&'|') {
                    chars.next();
                    tokens.push(Token::Or);
                } else {
                    tokens.push(Token::Pipe);
                }
            }
            ('>', None) => {
                flush_word(&mut current_word, &mut tokens);
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Append);
                } else {
                    tokens.push(Token::RedirectOut);
                }
            }
            ('&', None) => {
                flush_word(&mut current_word, &mut tokens);
                if chars.peek() == Some(&'&') {
                    chars.next();
                    tokens.push(Token::And);
                }
            }
            (';', None) => {
                flush_word(&mut current_word, &mut tokens);
                tokens.push(Token::Semicolon);
            }
            ('<', None) => {
                flush_word(&mut current_word, &mut tokens);
                tokens.push(Token::RedirectIn);
            }

            _ => {
                current_word.push(c);
            }
        }
    }

    if !current_word.is_empty() {
        tokens.push(Token::Word(current_word.clone()));
    }

    tokens
}

fn flush_word(current_word: &mut String, tokens: &mut Vec<Token>) {
    if !current_word.is_empty() {
        tokens.push(Token::Word(current_word.clone()));
        current_word.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    fn w(s: &str) -> Token {
        Word(s.to_string())
    }

    // === Words and quotes ===

    #[test]
    fn test_basic_words() {
        assert_eq!(
            tokenize("echo hello world"),
            vec![w("echo"), w("hello"), w("world")]
        );
    }

    #[test]
    fn test_double_quotes() {
        assert_eq!(
            tokenize("echo \"hello world\""),
            vec![w("echo"), w("hello world")]
        );
    }

    #[test]
    fn test_single_quotes() {
        assert_eq!(
            tokenize("echo 'hello world'"),
            vec![w("echo"), w("hello world")]
        );
    }

    #[test]
    fn test_extra_whitespace() {
        assert_eq!(tokenize("  echo   hello  "), vec![w("echo"), w("hello")]);
    }

    // === Pipes ===

    #[test]
    fn test_pipe() {
        assert_eq!(
            tokenize("ls | grep foo"),
            vec![w("ls"), Pipe, w("grep"), w("foo")]
        );
    }

    #[test]
    fn test_pipe_no_spaces() {
        assert_eq!(
            tokenize("ls|grep foo"),
            vec![w("ls"), Pipe, w("grep"), w("foo")]
        );
    }

    // === Logical operators ===

    #[test]
    fn test_and_operator() {
        assert_eq!(
            tokenize("echo hello && echo world"),
            vec![w("echo"), w("hello"), And, w("echo"), w("world")]
        );
    }

    #[test]
    fn test_or_operator() {
        assert_eq!(
            tokenize("echo hello || echo world"),
            vec![w("echo"), w("hello"), Or, w("echo"), w("world")]
        );
    }

    // === Semicolons ===

    #[test]
    fn test_semicolon() {
        assert_eq!(
            tokenize("echo hello ; echo world"),
            vec![w("echo"), w("hello"), Semicolon, w("echo"), w("world")]
        );
    }

    // === Redirects ===

    #[test]
    fn test_redirect_out() {
        assert_eq!(
            tokenize("echo hello > out.txt"),
            vec![w("echo"), w("hello"), RedirectOut, w("out.txt")]
        );
    }

    #[test]
    fn test_redirect_in() {
        assert_eq!(
            tokenize("cat < input.txt"),
            vec![w("cat"), RedirectIn, w("input.txt")]
        );
    }

    #[test]
    fn test_append() {
        assert_eq!(
            tokenize("echo hello >> out.txt"),
            vec![w("echo"), w("hello"), Append, w("out.txt")]
        );
    }

    // === Operators inside quotes should be treated as words ===

    #[test]
    fn test_pipe_in_quotes() {
        assert_eq!(
            tokenize("echo \"hello | world\""),
            vec![w("echo"), w("hello | world")]
        );
    }

    #[test]
    fn test_and_in_quotes() {
        assert_eq!(
            tokenize("echo \"hello && world\""),
            vec![w("echo"), w("hello && world")]
        );
    }

    #[test]
    fn test_semicolon_in_quotes() {
        assert_eq!(
            tokenize("echo \"hello ; world\""),
            vec![w("echo"), w("hello ; world")]
        );
    }

    // === Mixed ===

    #[test]
    fn test_complex_pipeline_with_redirect() {
        assert_eq!(
            tokenize("cat < in.txt | grep foo > out.txt"),
            vec![
                w("cat"),
                RedirectIn,
                w("in.txt"),
                Pipe,
                w("grep"),
                w("foo"),
                RedirectOut,
                w("out.txt")
            ]
        );
    }

    #[test]
    fn test_chain_with_pipe() {
        assert_eq!(
            tokenize("echo hello && ls | grep foo"),
            vec![
                w("echo"),
                w("hello"),
                And,
                w("ls"),
                Pipe,
                w("grep"),
                w("foo")
            ]
        );
    }
}
