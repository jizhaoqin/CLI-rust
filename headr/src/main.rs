use anyhow::Result;
use clap::Parser;
use std::fs;

#[derive(Parser)]
/// RUST version of `head`
struct Args {
    /// Input files
    files: Vec<String>,

    #[arg(short('n'), long, default_value = "10")]
    /// Print the first LINES of each file
    lines: u64,

    #[arg(short('c'), long)]
    /// Print the first BYTES of each file
    bytes: Option<u64>,
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    for file_name in &args.files {
        if args.files.len() > 1 {
            println!("==> {} <==", file_name);
        }
        match fs::read_to_string(file_name) {
            Err(err) => eprintln!(
                "{}: cannot open '{}' for reading: {}",
                env!("CARGO_PKG_NAME"),
                file_name,
                err
            ),
            Ok(contents) => {
                if let Some(bytes) = args.bytes {
                    let contents = contents.as_bytes();
                    let buffer = if bytes as usize > contents.len() {
                        contents
                    } else {
                        &contents[..bytes as usize]
                    };
                    print!("{}", String::from_utf8_lossy(buffer));
                } else {
                    for line in contents.lines().take(args.lines as usize) {
                        println!("{}", line)
                    }
                }
            }
        }
        println!()
    }
    Ok(())
}
