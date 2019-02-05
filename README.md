# cuid-rust

![Travis Build Status](https://travis-ci.com/mplanchard/cuid-rust.svg?branch=master "Master Status")

Cuids are "Collision-resistant ids optimized for horizontal scaling and
binary search lookup performance."

This is a rust implementation of the CUID library, the original JavaScript
implementation of which may be found [here](https://github.com/ericelliott/cuid)

## Usage

```rust
use cuid;

fn main() -> () {
    println!("{}", cuid.cuid());
    println!("{}", cuid.slug());
}
```
