# cuid-rust

[![Travis Build Status](https://travis-ci.com/mplanchard/cuid-rust.svg?branch=master "Master Status")](https://travis-ci.com/mplanchard/cuid-rust)
[![Crates.io](https://img.shields.io/crates/v/cuid "Crates.io")](https://crates.io/crates/cuid/)

Cuids are "Collision-resistant ids optimized for horizontal scaling and
binary search lookup performance."

This is a rust implementation of the CUID library, the original JavaScript
implementation of which may be found [here](https://github.com/ericelliott/cuid)

## Installation

In cargo.toml

```toml
cuid = "1.0.0"
```

## Usage

```rust
use cuid;

fn main() -> () {
    println!("{}", cuid.cuid().unwrap());
    println!("{}", cuid.slug().unwrap());
}
```

This package also provides a binary:

```sh
> cargo run cuid
cq64d5t05g4lx7twdb3t
```
