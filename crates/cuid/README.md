# cuid

[![Build Status](https://github.com/mplanchard/cuid-rust/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/mplanchard/cuid-rust/actions/workflows/ci.yml?query=branch%3Amaster)
[![Crates.io](https://img.shields.io/crates/v/cuid "Crates.io")](https://crates.io/crates/cuid/)
[![docs.rs](https://docs.rs/cuid/badge.svg)](https://docs.rs/cuid/)

Cuids are "Collision-resistant ids optimized for horizontal scaling and
binary search lookup performance."

This is a rust implementation of the CUID library, the original JavaScript
implementation of which may be found [here](https://github.com/ericelliott/cuid)

This library provides two CUID algorithms: `cuid1` and `cuid2`.
- `cuid1` is a k-sortable ID that is extremely fast to generate with
  very good randomness properties
- `cuid2` provides stronger randomness guarantees, but is slower to
  generate and is not k-sortable

**NOTE:** It is the position of the original authors of CUID that
`cuid1` is "insecure" due to its being k-sortable and potentially
exposing information about generation order and/or time of
generation. It is my position that these properties apply to a number
of very good ID-generating algorithms (such as UUIDv7), and it is
therefore up to the users of this crate to choose an ID
appropriately. Therefore, this library will continue to support v1
CUIDs for the foreseeable future. See the original authors' position in more detail [here](https://github.com/paralleldrive/cuid2?tab=readme-ov-file#note-on-k-sortablesequentialmonotonically-increasing-ids).

By default, both `cuid1` and `cuid2` algorithms are provided. If you
only need one, you can disable default features and select just the one
you need, for example to only include v2 CUIDs:

```toml
cuid = { version = "2.0.0", default-features = false, features = ["v2"] }
```

You can also use the [cuid1] or [cuid2] crates independently.

## Upgrading to version 2.0

Functions like `cuid()` have been removed in favor of version-specific
functions, like `cuid1()`. You can either update the function calls or
replace your use of the cuid crate with the [cuid1] or [cuid2] crate.

## Installation

In cargo.toml

```toml
cuid = "2.0.0"
```

Or install the binary:

```sh
> cargo install cuid
```

## Usage

```rust
use cuid;

fn main() -> () {
    // V1 CUIDs and slugs
    println!("{}", cuid::cuid1().unwrap());
    println!("{}", cuid::cuid1_slug().unwrap());

    // V2 CUIDs and slugs
    println!("{}", cuid::cuid2());
    println!("{}", cuid::cuid2_slug());

    // There is a flexible constructor for v2 CUIDs to customize
    // the length, counter function, and fingerprinter (note that
    // these are const functions, so you can create a static
    // constructor if desired.
    static CONSTRUCTOR: Cuid2Constructor = Cuid2Constructor::new()
        .with_length(20)
        .with_counter(const_counter)
        .with_fingerprinter(const_fingerprinter);
    println!("{}", constructor.create_id());
}

const fn const_counter() -> u64 {
    42
}

const fn const_fingerprinter() -> String {
    "fingers".to_string()
}
```

Both algorithms are safe to use across threads. When used in a
multithreaded context, threads generating v1 CUIDs share the same
atomic counter, which is used as a component of the generated CUID,
while each thread generating v2 CUIDs has its own atomic counter.

This package also provides a binary if installed via `cargo install`.

Its default behavior is to generate a CUID:

```sh
> cuid
clzuo0vcy4djkx3p2e4p0j355
```

You can also generate a slug:

```sh
>  cuid --slug
9bag9gxz78
```

Or v2 CUIDs/slugs:

```sh
> cuid --v2
scwl6p78dpjwvdtg1rqgvi1p

>  cuid --v2 --slug
pa00dip6j2
```

See `cuid --help` for more information.

## Performance

Performance is one of the primary concerns of this library (see
[Benchmarking](#benchmarking), below).

This implementation is currently about 20x faster than the reference JavaScript
implementation for v1 CUIDs. I have not benchmarked the JavaScript v2
CUIDs.

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
[cuid1]: https://crates.io/crates/cuid1/
[cuid2]: https://crates.io/crates/cuid2/
