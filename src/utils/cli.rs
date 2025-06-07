use clap::Parser;

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Skip root user check
    #[arg(long, short = 'r')]
    pub no_root_check: bool,

    /// Skip Linux check
    #[arg(long, short = 'l')]
    pub no_linux_check: bool,

    /// Skip terminal size check
    #[arg(long, short = 's')]
    pub no_size_check: bool,

    /// Enable mouse support
    #[arg(long, short = 'm')]
    pub mouse: bool,
}

#[must_use]
pub fn parse_args() -> Args {
    Args::parse()
}
