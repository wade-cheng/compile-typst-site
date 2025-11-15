# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `--path` arg to CLI.
- Yet more docs.

### Changed

- Replace `$PROJECT_ROOT` with the project root path. This is technically breaking. Maybe I should disallow `$` in those fields?
- Split publically available library code into `stable` and `internal` modules; only stable code follows semver, and internal code is provided for convenience but with no guarantees.
- Move all library code into `internal` for now.

## [1.0.0] - 2025-11-14

### Added

- First release!
