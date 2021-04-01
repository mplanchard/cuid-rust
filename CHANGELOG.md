# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.2] - 2021-04-01

### Changed

- Significant performance improvements for multithreaded usage (~30-35%) due to
  the counter now using `AtomicU32` again rather than `Arc<Mutex<32>>`, enabled
  thanks to the stabilization of [`fetch_update`](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU32.html#method.fetch_update)
  ([3a5cc2e](https://github.com/mplanchard/cuid-rust/commit/3a5cc2e))
- Generating CUIDs and slugs is now ~15-20% faster, thanks to reductions in the
  number of allocated strings and improvements to how space is pre-allocated
  during the generation of radix strings ([a09bfad](https://github.com/mplanchard/cuid-rust/commit/a09bfad))
- Updated non-v1 requirements to require minor version compatibility
- Updated `rand` requirement from `~0.7` to `~0.8.0`
- Switched from Travis to GH Actions

## [1.0.1] - 2020-10-01

### Fixed

- Generated CUIDs now have a consistent length, aligning with the
  reference implementation (#2, thanks [@rasendubi]!)

## [1.0.0] - 2020-02-22

### Added

- Support for stable Rust (2018 edition)
- The crate now includes a simple binary (`cuid`) that can be used to generate
  CUIDs on the commandline

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

[unreleased]: https://github.com/mplanchard/cuid-rust/compare/v1.0.2...HEAD
[1.0.2]: https://github.com/mplanchard/cuid-rust/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/mplanchard/cuid-rust/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/mplanchard/cuid-rust/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/mplanchard/cuid-rust/compare/b691e4c32e25d7239157e85598c74a9f59124417...v0.1.0
[@rasendubi]: https://github.com/rasendubi
