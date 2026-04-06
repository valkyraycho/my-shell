# my-shell

A Unix shell built from scratch in Rust.

## Features

- **REPL loop** — interactive prompt with EOF (Ctrl-D) handling
- **External commands** — runs any program found in `$PATH` (e.g. `ls -la`, `grep foo`)
- **Built-in commands** — `cd`, `pwd`, `exit`, `export`
- **Pipelines** — chain commands with `|` (e.g. `ls | grep src | wc -l`)
- **I/O redirection** — `>`, `<`, `>>` for redirecting input and output
- **Signal handling** — Ctrl-C kills running commands, not the shell
- **Quoted arguments** — single and double quote support (e.g. `echo "hello world"`)
- **Logical operators** — `&&`, `||`, and `;` for conditional and sequential execution
- **Tilde expansion** — `~` expands to `$HOME`
- **Environment variables** — `export FOO=bar`, `echo $FOO`, `$?` for exit status
- **Glob expansion** — wildcard matching (e.g. `ls *.rs`, `cat src/*.toml`)
- **Tab completion** — file paths and commands from `$PATH`
- **Line editing** — arrow key history (persisted across sessions), cursor movement via rustyline

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.85+ required for edition 2024)
- macOS or Linux (uses Unix-specific APIs)

## Installation

```sh
git clone https://github.com/valkyraycho/my-shell.git
cd my-shell

# Run directly
cargo run

# Or install to your system
cargo install --path .
my-shell
```

## Running Tests

```sh
cargo test
```

## Usage

```
> pwd
/Users/you/projects/my-shell

> cd src

> ls
builtins.rs  executor.rs  lib.rs  main.rs  parser.rs  tokenizer.rs

> ls -la | grep ".rs" | wc -l
       5

> echo hello && echo world
hello
world

> ls /fake || echo "fallback"
ls: /fake: No such file or directory
fallback

> export NAME=world
> echo hello $NAME
hello world

> echo *.rs
builtins.rs completer.rs executor.rs lib.rs main.rs parser.rs tokenizer.rs

> exit
```

## Architecture

```
src/
├── main.rs       — REPL loop and command dispatch
├── lib.rs        — module re-exports
├── tokenizer.rs  — character-level tokenizer with quote handling
├── parser.rs     — command classification, pipeline splitting
├── builtins.rs   — cd, pwd, export
├── executor.rs   — single command execution and pipeline wiring
└── completer.rs  — tab completion for commands and file paths
```

**Tokenizer** walks input character-by-character with lookahead, producing structured tokens (`Word`, `Pipe`, `And`, `Or`, `Semicolon`, `RedirectIn`, `RedirectOut`, `Append`). Handles single/double quotes and multi-character operators (`&&`, `||`, `>>`).

**Parser** consumes the token stream in two layers: first splitting by `&&`/`||`/`;` into chained commands, then splitting each chain segment by `|` into pipelines. Each pipeline segment is classified as `Empty`, `Exit`, `Builtin`, `External`, or `Pipeline`.

**Executor** spawns child processes via `std::process::Command`. Pipelines connect consecutive processes by piping `stdout` → `stdin` using OS-level file descriptors. All pipeline stages run concurrently.

**Completer** provides tab completion via rustyline's `Completer` trait. Completes commands (builtins + `$PATH` executables) for the first word, and file paths for arguments.
