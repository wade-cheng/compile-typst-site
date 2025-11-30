use std::process::exit;

use compile_typst_site::internals::config::Config;
use compile_typst_site::internals::entrypoint;

fn main() {
    entrypoint::run(&Config::new()).unwrap_or_else(|e| {
        log::error!("{:?}", e);
        exit(1);
    });
}
