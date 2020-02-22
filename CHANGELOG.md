# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Support for stable Rust (2018 edition)

### Changed

- The atomic counter is now an `Arc<Mutex<u32>>` rather than an `AtomicUsize`.
  This change was made to ensure that the counter could be fetched and
  updated simultaneously without using the nightly-only `fetch_update`
- Updated dependencies
- Ensured documentation examples are run in the test suite

## [0.1.0] - 2019-02-04

### Added

- Initial release
- CUID & CUID slug generation
- Benchmark suite

[Unreleased]: https://github.com/mplanchard/cuid-rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mplanchard/cuid-rust/compare/b691e4c32e25d7239157e85598c74a9f59124417...v0.1.0
