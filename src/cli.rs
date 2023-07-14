pub use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// String to search for on TWIR issue archive
    #[arg(short, long)]
    pub search: String,

    /// If set, this option will always make the tool search online
    #[arg(short, long, default_value_t = false)]
    pub online: bool,

    /// Limits the ONLINE search to the last <limit> number of twir issues
    #[arg(short, long, default_value_t = 500)]
    pub limit: i32,

    #[arg(short, long, default_value_t = false)]
    pub update_cache: bool,
}

pub fn get_search_arg() -> String {
    let args = Args::parse();
    args.search
}
