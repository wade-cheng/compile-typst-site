# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
