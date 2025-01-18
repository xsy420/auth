use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Skip root user check
    #[arg(long)]
    pub no_root_check: bool,

    /// Skip terminal size check
    #[arg(long)]
    pub no_size_check: bool,

    /// Enable mouse support
    #[arg(long)]
    pub mouse: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
