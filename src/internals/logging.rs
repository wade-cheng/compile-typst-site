//! Set up logging.

use log::LevelFilter;
use simple_logger::SimpleLogger;

use crate::internals::config::Config;

fn log_level_filter(config: &Config) -> LevelFilter {
    if config.trace {
        LevelFilter::Trace
    } else if config.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    }
}

pub fn init(config: &Config) {
    let log_level_filter = log_level_filter(config);

    SimpleLogger::new()
        .with_level(log_level_filter)
        .init()
        .unwrap();
}
