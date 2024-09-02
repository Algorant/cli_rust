use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cat`
struct Args {
    // Input files
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    // Number lines
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    // Number nonblank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

// This was suggested by avante I am not sure it is correct
fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(reader) => {
                for line in reader.lines() {
                    match line {
                        Ok(content) => println!("{}", content),
                        Err(err) => return Err(anyhow::anyhow!("Error reading line: {}", err)),
                    }
                }
            },
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
