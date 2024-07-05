use anyhow::Result;
use clap::{ArgAction, Parser};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
struct Args {
    #[arg(default_value = ".")]
    paths: Vec<String>,

    #[arg(short('n'), value_parser(Regex::new), action(ArgAction::Append), num_args(0..))]
    names: Vec<Regex>,
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let name_filter = |entry: &DirEntry| {
        args.names.is_empty()
            || args
                .names
                .iter()
                .any(|name| name.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in args.paths.iter() {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry_result| match entry_result {
                Err(err) => {
                    eprint!("{}: {}", env!("CARGO_PKG_NAME"), err);
                    None
                }
                Ok(entry) => Some(entry),
            }) // filter 不改变类型, map 不改变长度, filter_map 既改变类型又改变长度
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }
    Ok(())
}
