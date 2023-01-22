# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [cuid 1.3.1]

### Changed

- Updated deprecation warnings to mention the `cuid2` crate

## [cuid 1.3.0], [cuid2 0.1.0]

### Deprecated

- The CUID v1 algorithm is now [deprecated](https://github.com/mplanchard/cuid-rust/issues/4),
  so all cuid v1 functions have been marked as such. Please use the new `cuid2`
  crate or the `cuid2()` function re-exported from the `cuid` crate.

### Added

- The CI suite now runs both `cargo clippy` and `cargo audit` ([edb22b5])
- Added nix files for dependency management ([8d2c180])
- Added the `cuid2` crate, providing library and binary for v2 of the CUID
  standard
- Added `cuid2()` function in the `cuid` crate

### Changed

- Upgraded to use the Rust 2021 edition ([cdc594c])
- Switched from `lazy_static` to `once_cell` ([3333dd4])
- Added `inline` directive to external functions ([8714f67])

## [1.2.0] - 2021-10-03

### Added

- Added `--slug`, `--version`, and `--help` arguments to `cuid` binary ([b93b5b3])

### Changed

- ~10-20% performance improvement overall through optimization of 
`pad_with_char()`([b5503d6])

## [1.1.0] - 2021-08-03

### Fixed

- 0-length strings now return `false` when being checked by `is_cuid()` rather
  than panicking ([a4fca2f], reported by [@DeppLearning](https://github.com/DeppLearning))

### Changed

- Improved specificity of `is_cuid()` and `is_slug()` checks ([a4fca2f])
- Use references for `is_cuid()` and `is_slug()` rather than converting to
  strings ([a4fca2f])

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

[unreleased]: https://github.com/mplanchard/cuid-rust/compare/cuid2-v0.1.0...HEAD
[cuid 1.3.1]: https://github.com/mplanchard/cuid-rust/compare/cuid-v1.3.0...cuid-v1.3.1
[cuid2 1.3.1]: https://github.com/mplanchard/cuid-rust/compare/v1.2.0...cuid2-v0.1.0
[cuid2 0.1.0]: https://github.com/mplanchard/cuid-rust/compare/v1.2.0...cuid2-v0.1.0
[cuid 1.3.0]: https://github.com/mplanchard/cuid-rust/compare/v1.2.0...cuid-v1.3.0
[1.2.0]: https://github.com/mplanchard/cuid-rust/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/mplanchard/cuid-rust/compare/v1.0.2...v1.1.0
[1.0.2]: https://github.com/mplanchard/cuid-rust/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/mplanchard/cuid-rust/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/mplanchard/cuid-rust/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/mplanchard/cuid-rust/compare/b691e4c32e25d7239157e85598c74a9f59124417...v0.1.0
[@rasendubi]: https://github.com/rasendubi
[8714f67]: https://github.com/mplanchard/cuid-rust/commit/8714f67
[3333dd4]: https://github.com/mplanchard/cuid-rust/commit/3333dd4
[cdc594c]: https://github.com/mplanchard/cuid-rust/commit/cdc594c
[edb22b5]: https://github.com/mplanchard/cuid-rust/commit/edb22b5
[8d2c180]: https://github.com/mplanchard/cuid-rust/commit/8d2c180
[a4fca2f]: https://github.com/mplanchard/cuid-rust/commit/a4fca2f
[b5503d6]: https://github.com/mplanchard/cuid-rust/commit/b5503d6 
[b93b5b3]: https://github.com/mplanchard/cuid-rust/commit/b93b5b3 
