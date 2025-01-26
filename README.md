# Renlang

**A stack-based programming language written in Rust**  
Explore interpreter/compiler fundamentals through lexical analysis, parsing, and stack-based execution.

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange)](https://www.rust-lang.org/)

## Features
-  **Stack-based operations** (push, pop, arithmetic)
-  **Lexer and parser** implementation for syntax analysis
-  **Simulation mode** for step-by-step execution tracing
-  **Compilation** to intermediate representation

---

## Installation

### Prerequisites
- Rust toolchain ([rustup](https://rustup.rs/))

### Build from Source
```
git clone https://github.com/shivkr6/renlang.git
cd renlang
cargo install --path .
```
### Usage
```Stack-based programming language interpreter

USAGE:
    renlang --file <FILE_PATH> <COMMAND>

COMMANDS:
    com     Compile to intermediate representation
    sim     Execute in simulation mode with stack tracing
    help    Show help

OPTIONS:
    -f, --file <FILE_PATH>    Path to .ren program file
    -h, --help                Print help
    -V, --version             Print version
```
