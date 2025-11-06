//! Compile a site directory structure from Typst files to HTML.
//!
//! Currently, we use out-of-process calls for everything. Might stand to refactor to pure functions as core functionality? Hmm.
//!
//! Also, not really usable as a crate because we set up argument parsing to generate our config. Hmm.

pub mod compile;
pub mod config;
pub mod entrypoint;
pub mod logging;
