use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: mkdir <directory>");
        std::process::exit(1);
    }

    let directory = &args[1];

    if let Err(err) = fs::create_dir(directory) {
        eprintln!("Failed to create directory '{}': {}", directory, err);
        std::process::exit(1);
    }
}
