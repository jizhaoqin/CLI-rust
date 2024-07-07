use anyhow::Result;
use clap::{ArgAction, Parser};
// use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
struct Args {
    #[arg(default_value = ".")]
    paths: Vec<String>,

    #[arg(short = 'n', action=ArgAction::Append, num_args=0..100)]
    names: Vec<String>,
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    // TODO: main logic
    for path in args.paths.iter() {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry_result| match entry_result {
                Err(err) => {
                    eprint!("{}: {}", env!("CARGO_PKG_NAME"), err);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(|entry: &DirEntry| {
                args.names.is_empty()
                    || args
                        .names
                        .iter()
                        .any(|name| entry.path().display().to_string().contains(name))
            })
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }
    println!("{:?}", args.names);
    Ok(())
}
