use anyhow::Context;
use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use std::ffi::CStr;
use std::fs;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;


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
}

enum Kind {
    Blob
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
        }=>{
            anyhow::ensure!(pretty_print, "mode isn't supported yet");
            //TODO: shortest unique
            let mut f = std::fs::File::open(format!(
                ".git/objects/{}/{}", 
                &object_hash[..2], 
                &object_hash[2..]
            ))
            .context("open in .gti/objects")?;
            let mut z = ZlibDecoder::new(f);
            let mut z = BufReader::new(z);
            let mut buf = Vec::new();
            z.read_until(0, &mut buf)
                .context("read header from .git/objects")?;
            let header = CStr::from_bytes_with_nul(&buf)
                .expect("There is exactly one nul at the end");
            let header = header
                .to_str()
                .context(".git/obnjects file header isn't valid UTF-8")?;
            let Some((kind, size)) = header.split_once(' ') else {
                anyhow::bail!(
                    ".git/objects file header did not start with a known type: '{header}'"
                );
            };
            let kind = match kind {
                "blob" => Kind::Blob,
                _=> anyhow::bail!("do not yet know how to print a '{kind}'"),     
            };
            let size = size
                .parse::<u64>()
                .context(".git/objects file header has invalid size: {size}")?;
            // NOTE: this won't error if the decomprtessed file is too long but won't spam stdout
            // making it not vulnarable to zipbombs.
            let mut z = z.take(size);
            match kind {
                Kind::Blob => {
                    let stdout = std::io::stdout();
                    let mut stdout = stdout.lock();
                    let n = std::io::copy(&mut z, &mut stdout)
                        .context("write .git/objects file to stdout")?;
                    anyhow::ensure!(n == size, ".git/objects file was not the expected size (expected: {size}, actual: {n})");
                }
            }
        }
    }

    Ok(())
}