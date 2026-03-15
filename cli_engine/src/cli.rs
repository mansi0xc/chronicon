use std::env;

use crate::objects;
use crate::repo;
use crate::storage;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "init" => repo::init_repo(),
        "add" => {
            if args.len() < 3 {
                print_help();
                return;
            }

            let file = &args[2];
            let data = storage::read_file(file);
            let hash = objects::store_blob(&data);
            println!("Stored object {}", hash);
        }
        _ => println!("Unknown Command"),
    }

    fn print_help() {
        println!("Chronicon - document version control");

        println!("Commands:");

        println!("  chronicon init");

        println!("  chronicon add <file>");
    }
}
