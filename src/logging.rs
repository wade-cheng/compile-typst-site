//! Set up logging.

use log::LevelFilter;
use simple_logger::SimpleLogger;

use crate::config::Config;

pub fn init(config: &Config) {
    SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level(
            "compile_typst_site",
            if config.trace {
                LevelFilter::Trace
            } else if config.verbose {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
        )
        .init()
        .unwrap();
}
