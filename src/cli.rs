use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long)]
    pub no_root_check: bool,

    #[arg(long)]
    pub no_size_check: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
