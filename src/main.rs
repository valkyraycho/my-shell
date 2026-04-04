use std::io::{Write, stdin, stdout};

fn main() {
    let mut input = String::new();
    loop {
        input.clear();
        print!("> ");
        stdout().flush().expect("failed to flush");
        if stdin().read_line(&mut input).expect("failed to read line") == 0 {
            break;
        };
        let line = input.trim();
        if line.is_empty() {
            continue;
        }
        if line == "exit" {
            break;
        }
        println!("your input: {}", line);
    }
}
