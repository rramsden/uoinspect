mod parser;

fn main() {
    match std::env::args().nth(1) {
        Some(argument) => process_subcommand(argument),
        None => print_help()
    }
}

fn process_subcommand(cmd: String) {
    match cmd.as_ref() {
        "parse" => {
            match std::env::args().nth(2) {
                Some(file_name) => process_mul(file_name),
                None => println!("No filename given")
            }
        },
        _ => println!("Can't identify filename")
    }
}

fn process_mul(file_name: String) {
    match file_name.as_ref() {
        "verdata.mul" => {
            parser::verdata(file_name)
        },
        _ => println!("This file is not supported")
    }
}

fn print_help() {
    let string = r#"UOInspect 0.0.1
Richard Ramsden
An Ultima Online file inspector written in Rust

USAGE:
    uoinspect <SUBCOMMAND> <ARGUMENTS>

SUBCOMMANDS:
    parse <FILENAME>         Read contents of a mul file

    "#;

    println!("{}", string)
}
