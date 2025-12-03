//! Set up logging.

use log::LevelFilter;
use simple_logger::SimpleLogger;

use crate::internals::config::Config;

const DEFAULT_LEVEL: LevelFilter = LevelFilter::Info;

fn log_level_filter(config: &Config) -> LevelFilter {
    if config.trace {
        LevelFilter::Trace
    } else if config.verbose {
        LevelFilter::Debug
    } else {
        DEFAULT_LEVEL
    }
}

/// Initialize logging so the [`log`] crate works.
pub fn init(config: &Config) {
    let log_level_filter = log_level_filter(config);

    SimpleLogger::new()
        .with_level(log_level_filter)
        .init()
        .unwrap();
}

/// Initialize logging so the [`log`] crate works.
///
/// Uses the defaults without requiring an entire [`Config`] struct to be created.
pub fn init_default() {
    SimpleLogger::new()
        .with_level(DEFAULT_LEVEL)
        .init()
        .unwrap();
}
