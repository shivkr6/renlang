use std::fs::File;
use std::io::Write;
use std::process::Command;

mod cli;
mod lexing;
enum Operations {
    Push = 0,
    Plus = 1,
    Minus = 2,
    Dump = 3,
    CountOps = 4,
}
fn main() -> std::io::Result<()> {
    // Parse the command line arguments using the generated Args struct
    let args = cli::set_flags().get_matches();

    let filepath = args.get_one::<String>("file").unwrap().trim().to_string();
    let mut program = parse_program(filepath);
    match args.subcommand() {
        Some(("sim", _sub_matches)) => simulate_program(&mut program),
        Some(("com", _sub_matches)) => {
            let compilation_result = compile_program(&mut program, "output.asm");
            let _nasm_com = Command::new("nasm")
                .arg("-felf64")
                .arg("output.asm")
                .output()
                .expect("failed to compile nasm assembly");
            let _nasm_link = Command::new("ld")
                .arg("-o")
                .arg("output")
                .arg("output.o")
                .output()
                .expect("failed to compile nasm assembly");

            compilation_result
        }
        _ => unreachable!("invalid subcommand"),
    }
}

fn push(x: i32) -> (Operations, Option<i32>) {
    (Operations::Push, Some(x))
}

fn plus() -> (Operations, Option<i32>) {
    (Operations::Plus, None)
}

fn minus() -> (Operations, Option<i32>) {
    (Operations::Minus, None)
}

fn dump() -> (Operations, Option<i32>) {
    (Operations::Dump, None)
}

fn parse_word_as_op(token: (String, usize, usize, String)) -> (Operations, Option<i32>) {
    assert_eq!(
        Operations::CountOps as u32,
        4,
        "Exhaustive handling of Operations while parsing"
    );
    let filepath = token.0;
    let row = token.1;
    let col = token.2;
    let word = token.3;
    if word == "+" {
        plus()
    } else if word == "-" {
        minus()
    } else if word == "." {
        dump()
    } else if word.parse::<i32>().is_err() {
        let err = "invalid digit found in file";
        eprintln!("{}:{}:{}: {}: '{}'", filepath, row, col, err, word);
        std::process::exit(1);
    } else {
        push(word.parse::<i32>().unwrap())
    }
}

fn parse_program(filepath: String) -> Vec<(Operations, Option<i32>)> {
    let vec_string = lexer::tokenize_file(&filepath);
    vec_string.into_iter().map(parse_word_as_op).collect()
}

fn simulate_program(program: &mut [(Operations, Option<i32>)]) -> std::io::Result<()> {
    let mut stack = Vec::new();
    for op in program.iter() {
        match op.0 {
            Operations::Push => stack.push(op.1.unwrap()),
            Operations::Plus => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            Operations::Minus => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            }
            Operations::Dump => {
                let a = stack.pop().unwrap();
                println!("{}", a)
            }
            Operations::CountOps => unreachable!("COUNT OPS IS NOT REACHABLE"),
        }
    }
    Ok(())
}

fn compile_program(
    program: &mut [(Operations, Option<i32>)],
    out_filepath: &str,
) -> std::io::Result<()> {
    let mut out = File::create(out_filepath)
        .unwrap_or_else(|error| panic!("Invalid output file path: {:?}", error));

    // Defining the dump function in assembly
    writeln!(out, "segment .text")?;
    writeln!(out, "dump:")?;
    writeln!(out, "    mov     r9, -3689348814741910323")?;
    writeln!(out, "    sub     rsp, 40")?;
    writeln!(out, "    mov     BYTE [rsp+31], 10")?;
    writeln!(out, "    lea     rcx, [rsp+30]")?;
    writeln!(out, ".L2:")?;
    writeln!(out, "    mov     rax, rdi")?;
    writeln!(out, "    lea     r8, [rsp+32]")?;
    writeln!(out, "    mul     r9")?;
    writeln!(out, "    mov     rax, rdi")?;
    writeln!(out, "    sub     r8, rcx")?;
    writeln!(out, "    shr     rdx, 3")?;
    writeln!(out, "    lea     rsi, [rdx+rdx*4]")?;
    writeln!(out, "    add     rsi, rsi")?;
    writeln!(out, "    sub     rax, rsi")?;
    writeln!(out, "    add     eax, 48")?;
    writeln!(out, "    mov     BYTE [rcx], al")?;
    writeln!(out, "    mov     rax, rdi")?;
    writeln!(out, "    mov     rdi, rdx")?;
    writeln!(out, "    mov     rdx, rcx")?;
    writeln!(out, "    sub     rcx, 1")?;
    writeln!(out, "    cmp     rax, 9")?;
    writeln!(out, "    ja      .L2")?;
    writeln!(out, "    lea     rax, [rsp+32]")?;
    writeln!(out, "    mov     edi, 1")?;
    writeln!(out, "    sub     rdx, rax")?;
    writeln!(out, "    xor     eax, eax")?;
    writeln!(out, "    lea     rsi, [rsp+32+rdx]")?;
    writeln!(out, "    mov     rdx, r8")?;
    writeln!(out, "    mov     rax, 1")?;
    writeln!(out, "    syscall")?;
    writeln!(out, "    add     rsp, 40")?;
    writeln!(out, "    ret")?;
    writeln!(out, "global _start")?;
    writeln!(out, "_start:")?;

    for op in program.iter() {
        match op.0 {
            Operations::Push => {
                writeln!(out, "    ;; -- push {} --", op.1.unwrap())?;
                writeln!(out, "    push {}", op.1.unwrap())?;
            }
            Operations::Plus => {
                writeln!(out, "    ;; -- plus --")?;
                writeln!(out, "    pop rax")?;
                writeln!(out, "    pop rbx")?;
                writeln!(out, "    add rax, rbx")?;
                writeln!(out, "    push rax")?;
            }
            Operations::Minus => {
                writeln!(out, "    ;; -- minus --")?;
                writeln!(out, "    pop rax")?;
                writeln!(out, "    pop rbx")?;
                writeln!(out, "    sub rbx, rax")?;
                writeln!(out, "    push rbx")?;
            }
            Operations::Dump => {
                writeln!(out, "    ;; -- dump --")?;
                writeln!(out, "    pop rdi")?;
                writeln!(out, "    call dump")?;
            }
            Operations::CountOps => unreachable!("CountOps is not reachable while compiling"),
        }
    }
    writeln!(out, "    mov rax, 60")?;
    writeln!(out, "    mov rdi, 0")?;
    writeln!(out, "    syscall")?;

    Ok(())
}
