use clap::Parser;

/// C preprocessor
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Nothing
    #[arg(short, long)]
    name: String,
}

fn main() {
    let _args = Args::parse();
}
