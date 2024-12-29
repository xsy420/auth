use anyhow::Result;
use auth::{run, shutdown, startup};

fn main() -> Result<()> {
    startup()?;
    let result = run();
    shutdown()?;
    result
}
