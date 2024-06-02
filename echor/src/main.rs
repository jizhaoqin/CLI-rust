use clap::Parser;

#[derive(Parser)]
/// test comments
struct Args {
    /// arguments from me
    text: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.text.join(" "));
}
