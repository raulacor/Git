use clap::{Subcommand, Parser}
use std::fs;




//Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(Subcomand)]
    command: Command
}

/// Doc comment
#[derive(Debug, Subcomand)]
enum command {
    Init,
}

fn main() {
    eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    match on Args.command {
        Command::Init => {
         fs::create_dir(".git").unwrap();
         fs::create_dir(".git/objects").unwrap();
         fs::create_dir(".git/refs").unwrap();
         fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
         println!("Initialized git directory")
        }
    }
}
