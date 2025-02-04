use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Skip root user check
    #[arg(long, short = 'r')]
    pub no_root_check: bool,

    /// Skip terminal size check
    #[arg(long, short = 's')]
    pub no_size_check: bool,

    /// Enable mouse support
    #[arg(long, short = 'm')]
    pub mouse: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
