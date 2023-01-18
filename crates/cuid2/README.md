# Cuid2

Secure, collision-resistant ids optimized for horizontal scaling and
performance.

This is a Rust implementation of the CUID2 algorithm, defined by its
reference implementation [here](https://github.com/paralleldrive/cuid2).

Please see that repository for a discussion of the benefits of CUIDs, as
well as for the improvements in CUID2 over the original CUID algorithm
(which is also implemented in Rust [here](https://docs.rs/cuid/latest/cuid/)).

## Usage

The simplest usage is to use the `create_id()` function to create an ID:

```
use cuid2;

let id = cuid2::create_id();

assert_eq!(24, id.len());
```

A `cuid()` alias is provided to make this more of a drop-in replacement for
the v1 cuid package:

```
use cuid2::cuid;

let id = cuid();

assert_eq!(24, id.len());
```

If you would like to customize aspects of CUID production, you can create
a constructor with customized properties:

```
use cuid2::CuidConstructor;

let constructor = CuidConstructor::new().with_length(32);

let id = constructor.create_id();

assert_eq!(32, id.len());
```

If installed with `cargo install`, this package also provides a `cuid2`
binary, which generates a CUID on the command line. It can be used like:

```
> cuid2
y3cfw1hafbtezzflns334sb2
```
