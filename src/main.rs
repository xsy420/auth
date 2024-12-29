use anyhow::Result;
use auth::{root, run, shutdown, startup};

fn main() -> Result<()> {
    if root::check_root() {
        root::show_root_warning()?;
        return Ok(());
    }

    startup()?;
    let result = run();
    shutdown()?;
    result
}
