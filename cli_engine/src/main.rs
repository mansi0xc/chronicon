use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: Chronicon <command>");
        return;
    }

    match args[1].as_str() {
        "init" => init_repo(),
        "commit" => commit(),
        "log" => show_log(),
        _ => println!("Unknown command"),
    }
}

fn init_repo() {
    println!("Initializing chronicon repository...");
}

fn commit() {
    println!("Creating commit...");
}

fn show_log() {
    println!("Unvieling history...");
}
