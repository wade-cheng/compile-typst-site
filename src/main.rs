use std::process::exit;

fn main() {
    compile_typst_site::entrypoint::run().unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1)
    });
}
