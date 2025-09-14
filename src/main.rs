use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

pub mod commands;

//Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Doc comment
#[derive(Debug, Subcommand)]
enum Command {
    /// Doc comment
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,

        object_hash: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,

        file: PathBuf,
    }
}


fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    //println!("Logs from your program will appear here!");

    match args.command {
        Command::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        Command::CatFile {
            pretty_print, 
            object_hash,
        } => commands::cat_file::invoke(pretty_print, &object_hash)?, 
        Command::HashObject {
            write, 
            file,
        } => commands::hash_object::invoke(write, &file)?,
    }
    Ok(())
}