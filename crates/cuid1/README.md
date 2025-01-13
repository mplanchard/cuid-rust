# cuid1

[![Build Status](https://github.com/mplanchard/cuid-rust/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/mplanchard/cuid-rust/actions/workflows/ci.yml?query=branch%3Amaster)
[![Crates.io](https://img.shields.io/crates/v/cuid "Crates.io")](https://crates.io/crates/cuid1/)
[![docs.rs](https://docs.rs/cuid/badge.svg)](https://docs.rs/cuid1/)

Cuids are "Collision-resistant ids optimized for horizontal scaling and
binary search lookup performance."

This is a rust implementation of the CUID library, the original JavaScript
implementation of which may be found [here](https://github.com/ericelliott/cuid)

This library provides the `cuid1` algorithm:
- `cuid1` is a k-sortable ID that is extremely fast to generate with
  very good randomness properties

**NOTE:** It is the position of the original authors of CUID that
`cuid1` is "insecure" due to its being k-sortable and potentially
exposing information about generation order and/or time of
generation. It is my position that these properties apply to a number
of very good ID-generating algorithms (such as UUIDv7), and it is
therefore up to the users of this crate to choose an ID
appropriately. Therefore, this library will continue to support v1
CUIDs for the foreseeable future. See the original authors' position in more detail [here](https://github.com/paralleldrive/cuid2?tab=readme-ov-file#note-on-k-sortablesequentialmonotonically-increasing-ids).

If you only need `cuid2`, you can use the `cuid2` crate: [cuid2 crate](https://docs.rs/cuid2/latest/cuid2/).

## Installation

In cargo.toml

```toml
cuid1 = "0.1.0"
```

Or install the binary:

```sh
> cargo install cuid1
```

## Usage

```rust
use cuid1;

fn main() -> () {
    // CUIDs and slugs
    println!("{}", cuid1::cuid().unwrap());
    println!("{}", cuid::cuid_slug().unwrap());
}
```

CUIDs are safe to use across threads. When used in a multithreaded
context, threads generating v1 CUIDs share the same atomic counter,
which is used as a component of the generated CUID.

This package also provides a binary if installed via `cargo install`.

Its default behavior is to generate a CUID:

```sh
> cuid1
clzuo0vcy4djkx3p2e4p0j355
```

You can also generate a slug:

```sh
>  cuid1 --slug
9bag9gxz78
```

See `cuid1 --help` for more information.

## Performance

Performance is one of the primary concerns of this library (see
[Benchmarking](#benchmarking), below).

This implementation is currently about 20x faster than the reference JavaScript
implementation for v1 CUIDs.

It takes about 280 nanoseconds to generate a CUID, or 200 nanoseconds
to generate a CUID slug, on relatively modern desktop hardware.

In a long-running process or thread, CUID generation is faster, since the system
fingerprint is calculated once and then re-used for the lifetime of the process.
In this case, CUID generation takes about 125 ns.

## Tests

Tests can be run with

```text
cargo test -p cuid1
```

Some tests are ignored because they're slow. To run them:

```text
cargo test -p cuid1 -- collisions::test --ignored --test-threads=1
```

Some tests require to be run in a single thread and are ignored by default.
To run them:

```text
cargo test -p cuid1 -- collisions::single_thread --ignored --test-threads=1
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
