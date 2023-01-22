# cuid-rust

[![Build Status](https://github.com/mplanchard/cuid-rust/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/mplanchard/cuid-rust/actions/workflows/ci.yml?query=branch%3Amaster)
[![Crates.io](https://img.shields.io/crates/v/cuid "Crates.io")](https://crates.io/crates/cuid/)
[![docs.rs](https://docs.rs/cuid/badge.svg)](https://docs.rs/cuid/)

Cuids are "Collision-resistant ids optimized for horizontal scaling and
binary search lookup performance."

This is a rust implementation of the CUID library, the original JavaScript
implementation of which may be found [here](https://github.com/ericelliott/cuid)

## Installation

In cargo.toml

```toml
cuid = "1.2.0"
```

Or install the binary:

```sh
> cargo install cuid
```

## Usage

```rust
use cuid;

fn main() -> () {
    println!("{}", cuid::cuid().unwrap());
    println!("{}", cuid::slug().unwrap());
}
```

`cuid` is safe to use across threads. When used in a multithreaded context, all
threads share the same atomic counter, which is used as a component of the
generated CUID.

This package also provides a binary if installed via `cargo install`. 

Its default behavior is to generate a CUID:

```sh
> cuid
ckmqrwysb0000iz5p4u1b79hd
```

You can also generate a slug:

```sh
>  cuid --slug
4k0000dszy
```

See `cuid --help` for more information.

## Performance

Performance is one of the primary concerns of this library (see
[Benchmarking](#benchmarking), below).

This implementation is currently about 20x faster than the reference JavaScript
implementation.

It takes about 280 nanoseconds to generate a CUID, or 200 nanoseconds
to generate a CUID slug, on relatively modern desktop hardware.

In a long-running process or thread, CUID generation is faster, since the system
fingerprint is calculated once and then re-used for the lifetime of the process.
In this case, CUID generation takes about 125 ns.

## Tests

Tests can be run with

```text
cargo test -p cuid
```

Some tests are ignored because they're slow. To run them:

```text
cargo test -p cuid -- collisions::test --ignored --test-threads=1
```

Some tests require to be run in a single thread and are ignored by default.
To run them:

```text
cargo test -p cuid -- collisions::single_thread --ignored --test-threads=1
```

## Benchmarking

Inline benchmarks are available when running with the nightly toolchain. There
are also criterion bnechmarks in [`benches/cuid.rs`][benches].

If you're on a Linux system, it's recommended to run benchmarks with the
maximum possible priority, via `nice`, in order to avoid confounding effects
from other processes running on the system:

```text
$ nice -n -20 cargo bench
```

Note that you may need to run `nice` as root:

``` text
sudo nice -n -20 su <username> -l -c "cd $PWD && cargo bench"
```

[benches]: ./benches/cuid.rs
