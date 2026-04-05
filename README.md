# my-shell

A Unix shell built from scratch in Rust.

## Features

- **REPL loop** — interactive prompt with EOF (Ctrl-D) handling
- **External commands** — runs any program found in `$PATH` (e.g. `ls -la`, `grep foo`)
- **Built-in commands** — `cd`, `pwd`, `exit`
- **Pipelines** — chain commands with `|` (e.g. `ls | grep src | wc -l`)
- **I/O redirection** — `>`, `<`, `>>` for redirecting input and output
- **Signal handling** — Ctrl-C kills running commands, not the shell
- **Quoted arguments** — single and double quote support (e.g. `echo "hello world"`)
- **Line editing** — arrow key history, cursor movement via rustyline

## Getting Started

```sh
cargo build
cargo run
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

> exit
```

## Architecture

```
src/
├── main.rs       — REPL loop and command dispatch
├── lib.rs        — module re-exports
├── tokenizer.rs  — character-level tokenizer with quote handling
├── parser.rs     — command classification, pipeline splitting
├── builtins.rs   — cd, pwd
└── executor.rs   — single command execution and pipeline wiring
```

**Tokenizer** walks input character-by-character, handling single/double quotes and whitespace splitting.

**Parser** splits input on `|`, tokenizes each segment, and classifies commands into `Empty`, `Exit`, `Builtin`, `External`, or `Pipeline` variants.

**Executor** spawns child processes via `std::process::Command`. Pipelines connect consecutive processes by piping `stdout` → `stdin` using OS-level file descriptors. All pipeline stages run concurrently.
