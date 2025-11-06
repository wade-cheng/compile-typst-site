fn main() {
    compile_typst_site::entrypoint::run().unwrap_or_else(|err| err.print_msg());
}
