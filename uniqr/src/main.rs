use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs;
use std::io::Write;

#[derive(Parser)]
struct Args {
    input_file: String,

    output_file: Option<String>,

    #[arg(short, long)]
    count: bool,
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    // open input file and handle errors
    let contents = fs::read_to_string(&args.input_file)
        .map_err(|err| anyhow!("{}: {}: {}", env!("CARGO_PKG_NAME"), args.input_file, err))?;

    // open output file and handle errors, if `args.output_file` is given; otherwise print to `stdout`
    let mut out_file: Box<dyn Write> = match &args.output_file {
        Some(out_name) => Box::new(
            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(out_name)
                .map_err(|err| anyhow!("{}: {}: {}", env!("CARGO_PKG_NAME"), out_name, err))?,
        ),
        None => Box::new(std::io::stdout()),
    };

    // omit adjacent duplicated lines and count repetition
    let count_result = count_lines(contents);

    // print `count_result` to the output target
    if args.count {
        for (count, line) in count_result {
            writeln!(out_file, "{:>7} {:<}", count, line)?;
        }
    } else {
        for (_count, line) in count_result {
            writeln!(out_file, "{:<}", line)?;
        }
    }
    Ok(())
}

fn count_lines(contents: String) -> Vec<(usize, String)> {
    let mut count_result: Vec<(usize, String)> = Vec::new();
    let mut count = 1;
    let mut previous_line: Option<&str> = None;

    for line in contents.lines() {
        match previous_line {
            Some(prev) if prev == line => {
                count += 1;
            }
            Some(prev) => {
                count_result.push((count, prev.to_string()));
                count = 1;
            }
            None => {}
        }
        previous_line = Some(line);
    }
    if let Some(last_line) = previous_line {
        count_result.push((count, last_line.to_string()));
    }

    count_result
}
