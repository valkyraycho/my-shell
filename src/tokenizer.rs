pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote_char: Option<char> = None;

    for c in input.chars() {
        match (c, quote_char) {
            // opening quote
            ('"' | '\'', None) => quote_char = Some(c),

            // closing the matching quote
            (q, Some(open)) if q == open => quote_char = None,

            // spaces outside a quote
            (c, None) if c.is_ascii_whitespace() => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        tokens.push(current.clone());
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(tokenize("echo hello world"), vec!["echo", "hello", "world"]);
    }

    #[test]
    fn test_double_quotes() {
        assert_eq!(
            tokenize("echo \"hello world\""),
            vec!["echo", "hello world"]
        );
    }

    #[test]
    fn test_single_quotes() {
        assert_eq!(tokenize("echo 'hello world'"), vec!["echo", "hello world"]);
    }
}
