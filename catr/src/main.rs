use anyhow::Result;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    /// Input files
    files: Vec<String>,

    #[arg(short = 'n', long = "number")]
    /// Number the line
    show_line_number: bool,
}

fn main() {
    // let args = Args::parse();
    if let Err(err) = run(Args::parse()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let mut line_number = 0;
    for file_name in args.files {
        match fs::read_to_string(&file_name) {
            Err(err) => eprintln!("{}: {}: {}", env!("CARGO_PKG_NAME"), file_name, err),
            Ok(contents) => {
                if args.show_line_number {
                    for line in contents.lines() {
                        line_number += 1;
                        println!("{:>6}  {}", line_number, line);
                    }
                } else {
                    for line in contents.lines() {
                        line_number += 1;
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}
