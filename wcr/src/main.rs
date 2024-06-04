use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
/// Rust version of `wc`
struct Args {
    /// Input files
    files: Vec<String>,

    /// Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    /// Show character count
    #[arg(short('m'), long)]
    chars: bool,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,
}

// #[derive(PartialEq)]
struct FileInfo {
    number_lines: usize,
    number_words: usize,
    number_bytes: usize,
    number_chars: usize,
}

impl std::ops::Add for FileInfo {
    type Output = FileInfo;
    fn add(self, rhs: Self) -> Self::Output {
        FileInfo {
            number_lines: self.number_lines + rhs.number_lines,
            number_words: self.number_words + rhs.number_words,
            number_bytes: self.number_bytes + rhs.number_bytes,
            number_chars: self.number_chars + rhs.number_chars,
        }
    }
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
            |acc_info, single_info| acc_info + single_info,
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
    if [args.words, args.bytes, args.chars, args.lines]
        .iter()
        .all(|v| !v)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let mut files_info: Vec<FileInfo> = Vec::new();
    // let mut total_info:FileInfo
    for file_name in args.files {
        match std::fs::read_to_string(&file_name) {
            Err(err) => eprintln!("{}: {}: {}", env!("CARGO_PKG_NAME"), file_name, err),
            Ok(contents) => {
                let current_info = FileInfo {
                    number_lines: contents.lines().count(),
                    number_words: contents.split_whitespace().count(),
                    number_bytes: contents.as_bytes().iter().count(),
                    number_chars: contents.chars().count(),
                };
                println!(
                    "{}{}{}{} {:<}",
                    format_string(current_info.number_lines, args.lines),
                    format_string(current_info.number_words, args.words),
                    format_string(current_info.number_bytes, args.bytes),
                    format_string(current_info.number_chars, args.chars),
                    file_name,
                );
                files_info.push(current_info);
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
