use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    files: Vec<String>,

    #[arg(short, long)]
    lines: bool,

    #[arg(short, long)]
    words: bool,

    #[arg(short('c'), long)]
    bytes: bool,

    #[arg(short('m'), long)]
    chars: bool,
}

struct FileInfo {
    number_lines: usize,
    number_words: usize,
    number_bytes: usize,
    number_chars: usize,
}

impl std::iter::Sum for FileInfo {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            FileInfo {
                number_lines: 0,
                number_words: 0,
                number_bytes: 0,
                number_chars: 0,
            },
            |acc_info, current_info| FileInfo {
                number_lines: acc_info.number_lines + current_info.number_lines,
                number_words: acc_info.number_words + current_info.number_words,
                number_bytes: acc_info.number_bytes + current_info.number_bytes,
                number_chars: acc_info.number_chars + current_info.number_chars,
            },
        )
    }
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run(mut args: Args) -> Result<()> {
    if [args.lines, args.words, args.bytes, args.chars]
        .iter()
        .all(|show| !show)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    let mut files_info: Vec<FileInfo> = Vec::new();
    for file_name in args.files {
        match std::fs::read_to_string(&file_name) {
            Err(err) => eprintln!("{}: {}: {}", env!("CARGO_PKG_NAME"), file_name, err),
            Ok(contents) => {
                let file_info = FileInfo {
                    number_lines: contents.lines().count(),
                    number_words: contents.split_whitespace().count(),
                    number_bytes: contents.as_bytes().iter().count(),
                    number_chars: contents.chars().count(),
                };
                println!(
                    "{}{}{}{} {:<}",
                    format_string(file_info.number_lines, args.lines),
                    format_string(file_info.number_words, args.words),
                    format_string(file_info.number_bytes, args.bytes),
                    format_string(file_info.number_chars, args.chars),
                    file_name,
                );
                files_info.push(file_info);
            }
        }
    }
    let total_info: FileInfo = files_info.into_iter().sum();
    println!(
        "{}{}{}{} {:<}",
        format_string(total_info.number_lines, args.lines),
        format_string(total_info.number_words, args.words),
        format_string(total_info.number_bytes, args.bytes),
        format_string(total_info.number_chars, args.chars),
        "total",
    );
    Ok(())
}

fn format_string(value: usize, show: bool) -> String {
    if show {
        format!("{:>5}", value)
    } else {
        "".to_string()
    }
}
