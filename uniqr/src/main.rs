use anyhow::{anyhow, Result};
use clap::Parser;
use std::{fs, io::Write};

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
    // read input file
    let contents = fs::read_to_string(&args.input_file)
        .map_err(|err| anyhow!("{}: {}: {}", env!("CARGO_PKG_NAME"), args.input_file, err))?;

    let mut output_target: Box<dyn Write> = match args.output_file {
        None => Box::new(std::io::stdout()),
        Some(file_name) => Box::new(
            fs::File::create(&file_name)
                .map_err(|err| anyhow!("{}: {}: {}", env!("CARGO_PKG_NAME"), file_name, err))?,
        ),
    };

    // logic of count duplicated lines
    let count_result: Vec<(usize, String)> = count_lines(contents);

    // print result
    if args.count {
        for (count, line) in count_result {
            writeln!(output_target, "{:>7} {:<}", count, line)?;
        }
    } else {
        for (_count, line) in count_result {
            writeln!(output_target, "{:<}", line)?;
        }
    }
    Ok(())
}

fn count_lines(contents: String) -> Vec<(usize, String)> {
    // create temporary variables
    let mut count_result: Vec<(usize, String)> = Vec::new();
    let mut previous_line: Option<&str> = None;
    let mut count: usize = 1;

    // main logic except the last line
    for current_line in contents.lines() {
        match previous_line {
            None => {}
            Some(previous) if previous == current_line => count += 1,
            Some(previous) => {
                count_result.push((count, previous.to_string()));
                count = 1;
            }
        }
        previous_line = Some(current_line);
    }

    // take care of the last line
    if let Some(last_line) = previous_line {
        count_result.push((count, last_line.to_string()));
    }

    // return the result
    count_result
}
