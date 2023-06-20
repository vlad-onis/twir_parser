use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// String to search for on TWIR
    #[arg(short, long)]
    search: String,
}

pub fn get_search_arg() -> String {
    let args = Args::parse();
    args.search
}
