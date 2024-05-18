use clap::{arg, ArgAction, Command};

pub fn set_flags() -> Command {
    Command::new("renlang")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Stack based programming language")
        .subcommand_required(true)
        .subcommand(Command::new("com").about("Compile the program"))
        .subcommand(Command::new("sim").about("Simulate the program"))
        .arg(
            arg!(-f - -file <FILE_PATH>)
                .required(true)
                .action(ArgAction::Set)
                .help("Mention the file path of the program"),
        )
}
