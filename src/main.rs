mod cli;
enum Operations {
    Push = 0,
    Plus = 1,
    Minus = 2,
    Dump = 3,
}
fn main() {
    // Parse the command line arguments using the generated Args struct
    let args = cli::set_flags().get_matches();

    let filepath = args.get_one::<String>("file").unwrap().trim().to_string();
    println!("{}", filepath);
    // TODO: Unhardcode program
    let mut program = vec![
        push(34),
        push(35),
        plus(),
        dump(),
        push(500),
        push(80),
        minus(),
        dump(),
    ];

    simulate_program(&mut program);
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

fn simulate_program(program: &mut [(Operations, Option<i32>)]) {
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
        }
    }
}
