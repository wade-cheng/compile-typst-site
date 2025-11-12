use anyhow::Result;
use compile_typst_site::config::Config;

fn main() -> Result<()> {
    compile_typst_site::entrypoint::run(&Config::new())?;

    Ok(())
}
