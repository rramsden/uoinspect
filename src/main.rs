mod utils;
mod verdata;
mod structs;
mod gump;
mod index;
mod lookup;

use std::path::Path;
use std::env::args;

fn main() {
    match std::env::args().nth(1) {
        Some(argument) => process_subcommand(argument),
        None => print_help()
    }
}

fn process_subcommand(cmd: String) {
    let args: Vec<String> = args().collect();

    match &cmd[..] {
        "parse" => parse( &args ),
        "lookup" => lookup( &args ),
        _ => println!("Unknown subcommand.")
    }
}

fn check_missing(args: &Vec<String>, n: usize) {
    if args.len() - 2 != n {
        panic!("Expected {} arguments but received {}.", n, args.len() - 2 );
    }
}

fn lookup(args: &Vec<String>) {
    check_missing(args, 2);

    let file_path = &args[2];
    let entry_id = &args[3];

    lookup::from_index(file_path, entry_id.parse::<usize>().unwrap())
}

fn parse(args: &Vec<String>) {
    check_missing(args, 1);

    let path = Path::new(&args[2]);
    let file_name = path
        .file_name().unwrap().to_str().unwrap().to_lowercase();

    match path.exists() {
        true => {
            match &file_name[..] {
                "verdata.mul" => verdata::to_json(&args[2]),
                _ => {
                    if file_name.ends_with("idx.mul") || file_name.ends_with("idx") {
                        index::to_json(&args[2]);
                    } else {
                        println!("Sorry, we don't know how to process this file.")
                    }
                }
            }
        
        },
        false => println!("Could not find '{}'", &args[2])
    }
}

fn print_help() {
    let string = r#"UOInspect 0.0.1
Richard Ramsden
An Ultima Online file inspector written in Rust

USAGE:
    uoinspect <SUBCOMMAND> <ARGUMENTS>

SUBCOMMANDS:
    json <filename>              Export contents of a mul file to JSON
    lookup <Filename> <EntryID>  Read binary contents using index file

    "#;

    println!("{}", string)
}
