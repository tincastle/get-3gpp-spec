use clap::Parser;

/// Simple CLI for fetching 3GPP spec info
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 3GPP spec number (positional)
    spec_number: String,

    /// Date string (optional)
    #[arg(short, long)]
    date: Option<String>,

    /// Release number (nonnegative integer)
    #[arg(short, long, value_parser = clap::value_parser!(u32))]
    release: Option<u32>,

    /// List flag (default: false)
    #[arg(short, long, default_value_t = false)]
    list: bool,
}

fn main() {
    let args = Args::parse();

    println!("spec_number: {}", args.spec_number);
    println!("date: {:?}", args.date);
    println!("release: {:?}", args.release);
    println!("list: {}", args.list);
}
