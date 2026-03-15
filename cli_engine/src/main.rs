use std::{env, fs};

mod cli;
mod commits;
mod objects;
mod repo;
mod storage;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     println!("Usage: Chronicon <command>");
    //     return;
    // }

    // match args[1].as_str() {
    //     "init" => repo::init_repo(),
    //     "commit" => commit(),
    //     "log" => show_log(),
    //     _ => println!("Unknown command"),
    // }

    cli::run();
}

fn commit() {
    println!("Creating commit...");
}

fn show_log() {
    println!("Unvieling history...");
}
