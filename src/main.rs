use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::process::Command;

mod cli;
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

fn file_to_string(filepath: String) -> String {
    let mut file =
        File::open(filepath).unwrap_or_else(|error| panic!("Invalid input file path: {:?}", error));
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn convert_string_to_vec_word(file_str: String) -> Vec<String> {
    file_str.split_whitespace().map(str::to_string).collect()
    // Now I have the operations as a vector ["3", "2", "+", "."]
}

fn parse_word_as_op(word: String) -> (Operations, Option<i32>) {
    assert_eq!(
        Operations::CountOps as u32,
        4,
        "Exhaustive handling of Operations while parsing"
    );
    if word == "+" {
        plus()
    } else if word == "-" {
        minus()
    } else if word == "." {
        dump()
    } else {
        push(word.parse::<i32>().unwrap())
    }
}

fn parse_program(filepath: String) -> Vec<(Operations, Option<i32>)> {
    let file_string = file_to_string(filepath);
    let vec_string = convert_string_to_vec_word(file_string);
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
    writeln!(&mut out, "segment .text")?;
    writeln!(&mut out, "dump:")?;
    writeln!(&mut out, "    mov     r9, -3689348814741910323")?;
    writeln!(&mut out, "    sub     rsp, 40")?;
    writeln!(&mut out, "    mov     BYTE [rsp+31], 10")?;
    writeln!(&mut out, "    lea     rcx, [rsp+30]")?;
    writeln!(&mut out, ".L2:")?;
    writeln!(&mut out, "    mov     rax, rdi")?;
    writeln!(&mut out, "    lea     r8, [rsp+32]")?;
    writeln!(&mut out, "    mul     r9")?;
    writeln!(&mut out, "    mov     rax, rdi")?;
    writeln!(&mut out, "    sub     r8, rcx")?;
    writeln!(&mut out, "    shr     rdx, 3")?;
    writeln!(&mut out, "    lea     rsi, [rdx+rdx*4]")?;
    writeln!(&mut out, "    add     rsi, rsi")?;
    writeln!(&mut out, "    sub     rax, rsi")?;
    writeln!(&mut out, "    add     eax, 48")?;
    writeln!(&mut out, "    mov     BYTE [rcx], al")?;
    writeln!(&mut out, "    mov     rax, rdi")?;
    writeln!(&mut out, "    mov     rdi, rdx")?;
    writeln!(&mut out, "    mov     rdx, rcx")?;
    writeln!(&mut out, "    sub     rcx, 1")?;
    writeln!(&mut out, "    cmp     rax, 9")?;
    writeln!(&mut out, "    ja      .L2")?;
    writeln!(&mut out, "    lea     rax, [rsp+32]")?;
    writeln!(&mut out, "    mov     edi, 1")?;
    writeln!(&mut out, "    sub     rdx, rax")?;
    writeln!(&mut out, "    xor     eax, eax")?;
    writeln!(&mut out, "    lea     rsi, [rsp+32+rdx]")?;
    writeln!(&mut out, "    mov     rdx, r8")?;
    writeln!(&mut out, "    mov     rax, 1")?;
    writeln!(&mut out, "    syscall")?;
    writeln!(&mut out, "    add     rsp, 40")?;
    writeln!(&mut out, "    ret")?;
    writeln!(&mut out, "global _start")?;
    writeln!(&mut out, "_start:")?;

    for op in program.iter() {
        match op.0 {
            Operations::Push => {
                writeln!(&mut out, "    ;; -- push {} --", op.1.unwrap())?;
                writeln!(&mut out, "    push {}", op.1.unwrap())?;
            }
            Operations::Plus => {
                writeln!(&mut out, "    ;; -- plus --")?;
                writeln!(&mut out, "    pop rax")?;
                writeln!(&mut out, "    pop rbx")?;
                writeln!(&mut out, "    add rax, rbx")?;
                writeln!(&mut out, "    push rax")?;
            }
            Operations::Minus => {
                writeln!(&mut out, "    ;; -- minus --")?;
                writeln!(&mut out, "    pop rax")?;
                writeln!(&mut out, "    pop rbx")?;
                writeln!(&mut out, "    sub rbx, rax")?;
                writeln!(&mut out, "    push rbx")?;
            }
            Operations::Dump => {
                writeln!(&mut out, "    ;; -- dump --")?;
                writeln!(&mut out, "    pop rdi")?;
                writeln!(&mut out, "    call dump")?;
            }
            Operations::CountOps => unreachable!("CountOps is not reachable while compiling"),
        }
    }
    writeln!(&mut out, "    mov rax, 60")?;
    writeln!(&mut out, "    mov rdi, 0")?;
    writeln!(&mut out, "    syscall")?;

    Ok(())
}
