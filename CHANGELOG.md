# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Emit a `Content-Length` header when serving HTTP.

### Added

- Docs.
- Logging durations of actions.
- Uppercase extensions have their mime types guessed instead of assuming binary.
- Logging looks more consistent (more things using `log` framework).
- `color` flag to CLI args.

### Changed

- Default output coloring now depends on whether the output is to a terminal, instead of always.

## [2.1.0-rc3] - 2025-12-02

### Added

- `disable_incremental` config to rebuild entire site on any change.

## [2.1.0-rc2] - 2025-11-30

### Changed

- Add a `rust-toolchain.toml` to try to fix `dist` for `aarch64-pc-windows-msvc`. See commit message for verbose details.

## [2.1.0-rc1] - 2025-11-30

### Fixed

- Not having `typst` installed should no longer hang at, e.g., the generating and writing file listing step.

## [2.1.0-rc0] - 2025-11-30

### Added

- `file_listing_extra_args`, `compilation_extra_args` to config file

### Changed

- Docs landing page/README.md parity.
- Recompile from scratch when a file is created.
- Clean up dependencies (see https://wade-cheng.com/blog/optimizing-compile-times/).
- Supply test suite Typst via `actions/setup-typst` instead of `cargo binstall`
  - `binstall` is sometimes forced to compile from scratch. Hm.
- Give `PassthroughCopyGlobs` config field an output in `-v`
- Logging changes/improvements
  - Log all modules, not just from our own binary (start seeing logs from crates e.g. in `-t` verbosity)
  - Always log sterr instead of only doing so on subprocess error.
  - Don't crash on ignorable errors while serving. i.e,
    when serve or watch mode is on, errors are demoted to warnings.
  - We also no longer join some threads but ehhhhh i'm sure it's fine. I can revisit it.

### Fixed

- More paths compatible with windows. `file_listing` should work on windows now.

## [2.0.3] - 2025-11-18

### Added

- test cases

### Changed

- Yet more docs.

## [2.0.2] - 2025-11-16

### Fixed

- watching (or serving) a project with no templates folder no longer fails.

## [2.0.1] - 2025-11-16

### Changed

- fix version in `Cargo.toml` still saying 1.0.0.
- exclude `tests/integration_test_contents` from cargo publish. For size.

## [2.0.0] - 2025-11-16

### Added

- `--path` arg to CLI.
- Yet more docs.
- Implement local file serving, binding to the output of linear search over ports 8000 to 8050.

### Changed

- Replace `$PROJECT_ROOT` with the project root path in the `init` and `post_processing_typ` config file fields.
- Disallow `$` in those fields.
- Split publically available library code into `stable` and `internal` modules; only stable code follows semver, and internal code is provided for convenience but with no guarantees.
- Move all library code into `internal` for now.

## [1.0.0] - 2025-11-14

### Added

- First release!
