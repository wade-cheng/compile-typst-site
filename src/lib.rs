//! Compile a site directory structure from Typst files to HTML.
//!
//! Currently, we use out-of-process calls for everything. Might stand to refactor to pure functions as core functionality? Hmm.
//!
//! Also, not really usable as a crate because we set up argument parsing to generate our config. Hmm.
//!
//! The `stable` module, if it exists, is for anything that abides by semantic versioning.
//! The `internals` module is not guaranteed to follow semantic versioning---it is included for your convenience, but use it your own risk.
//!
//! Because `config` module sets everything up, it cannot assume logging is initialized.
//! Also excepting `logging` itself, other modules do have the assumption that logging is initialized.

pub mod internals;
