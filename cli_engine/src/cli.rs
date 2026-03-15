use std::env;

use crate::repo;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "init" => repo::init_repo(),
        _ => println!("Unknown Command"),
    }

    fn print_help() {
        println!("Chronicon - document version control");

        println!("Usage:");
        println!("  chronicon init");
    }
}
