//! Set up logging.

use log::LevelFilter;
use simple_logger::SimpleLogger;

use crate::config::CONFIG;

pub fn init() {
    SimpleLogger::new()
        .without_timestamps()
        .with_level(LevelFilter::Off)
        .with_module_level(
            "compile_typst_site",
            if CONFIG.trace {
                LevelFilter::Trace
            } else if CONFIG.verbose {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
        )
        .init()
        .unwrap();
}
