use std::process::exit;

use compile_typst_site::internals::config::Config;
use compile_typst_site::internals::entrypoint;
use compile_typst_site::internals::logging;

fn main() {
    Config::new()
        .inspect(|config| logging::init(config))
        .inspect_err(|_| logging::init_default())
        .and_then(|config| entrypoint::run(&config))
        .unwrap_or_else(|e| {
            log::error!("{:?}", e);
            exit(1);
        });
}
