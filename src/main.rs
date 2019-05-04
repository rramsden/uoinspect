mod utils;
mod verdata;
mod structs;
mod gump;
mod index;

use std::path::Path;

fn main() {
    match std::env::args().nth(1) {
        Some(argument) => process_subcommand(argument),
        None => print_help()
    }
}

fn process_subcommand(cmd: String) {
    match &cmd[..] {
        "parse" => {
            match std::env::args().nth(2) {
                Some(file_path) => parse(file_path),
                None => println!("No filename given.")
            }
        },
        _ => println!("Unknown subcommand.")
    }
}

fn parse(file_path: String) {
    let path = Path::new(&file_path[..]);
    let file_name = path
        .file_name().unwrap().to_str().unwrap().to_lowercase();

    match path.exists() {
        true => {
            match &file_name[..] {
                "verdata.mul" => verdata::to_json(file_path),
                _ => {
                    if file_name.ends_with("idx.mul") || file_name.ends_with("idx") {
                        index::to_json(file_path);
                    } else {
                        println!("Sorry, we don't know how to process this file.")
                    }
                }

            }
        
        },
        false => println!("Could not find '{}'", file_path)
    }
}

fn print_help() {
    let string = r#"UOInspect 0.0.1
Richard Ramsden
An Ultima Online file inspector written in Rust

USAGE:
    uoinspect <SUBCOMMAND> <ARGUMENTS>

SUBCOMMANDS:
    json <FILENAME>         Export contents of a mul file to JSON

    "#;

    println!("{}", string)
}
