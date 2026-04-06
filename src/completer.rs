use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

pub struct ShellHelper;

impl Helper for ShellHelper {}
impl Highlighter for ShellHelper {}
impl Hinter for ShellHelper {
    type Hint = String;
}
impl Validator for ShellHelper {}

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
        let partial = &line[start..pos];

        if partial.is_empty() {
            return Ok((start, vec![]));
        }

        let mut matches = if start == 0 {
            complete_commands(partial)
        } else {
            complete_paths(partial)
        };

        matches.sort_by(|a, b| a.display.cmp(&b.display));
        matches.dedup_by(|a, b| a.display == b.display);

        Ok((start, matches))
    }
}

fn complete_commands(partial: &str) -> Vec<Pair> {
    let mut matches = Vec::new();

    // Builtins
    let builtins = ["cd", "pwd", "exit", "export"];
    for cmd in builtins {
        if cmd.starts_with(partial) {
            matches.push(Pair {
                display: cmd.to_string(),
                replacement: cmd.to_string(),
            });
        }
    }

    // PATH executables
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in path_var.split(':') {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with(partial) {
                        matches.push(Pair {
                            display: name.clone(),
                            replacement: name,
                        });
                    }
                }
            }
        }
    }

    matches
}

fn complete_paths(partial: &str) -> Vec<Pair> {
    let mut matches = Vec::new();
    let pattern = format!("{}*", partial);

    if let Ok(paths) = glob::glob(&pattern) {
        for entry in paths.filter_map(|e| e.ok()) {
            let mut name = entry.to_string_lossy().to_string();
            if entry.is_dir() {
                name.push('/');
            }
            matches.push(Pair {
                display: name.clone(),
                replacement: name,
            });
        }
    }

    matches
}
