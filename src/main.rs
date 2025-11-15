use anyhow::Result;
use compile_typst_site::internals::config::Config;
use compile_typst_site::internals::entrypoint;

fn main() -> Result<()> {
    entrypoint::run(&Config::new())?;

    Ok(())
}
