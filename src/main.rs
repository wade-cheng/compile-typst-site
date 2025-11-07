use std::process::exit;

use compile_typst_site::config::Config;

fn main() {
    compile_typst_site::entrypoint::run(&Config::new()).unwrap_or_else(|err| {
        eprintln!("{:?}", err);
        exit(1)
    });
}
