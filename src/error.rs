use error_iter::ErrorIter as _;
use notify_debouncer_full;
use onlyargs::CliError;
use onlyerror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Argument parsing error.
    Cli(#[from] CliError),
    /// I/O error.
    Io(#[from] std::io::Error),
    /// notify crate error.
    Notify(#[from] notify_debouncer_full::notify::Error),
    /// toml crate error.
    Toml(#[from] toml::de::Error),
    /// globset crate error.
    Globset(#[from] globset::Error),
}

impl Error {
    pub fn print_msg(&self) {
        eprintln!("Error: {self}");
        for source in self.sources().skip(1) {
            eprintln!("  Caused by: {source}");
        }

        if matches!(self, Error::Cli(_)) {
            eprintln!("Try {} --help", std::env::args().next().unwrap());
        }
    }
}
