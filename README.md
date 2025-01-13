# cuid-rust

[![Build Status](https://github.com/mplanchard/cuid-rust/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/mplanchard/cuid-rust/actions/workflows/ci.yml?query=branch%3Amaster)
[![Crates.io](https://img.shields.io/crates/v/cuid "Crates.io")](https://crates.io/crates/cuid/)
[![docs.rs](https://docs.rs/cuid/badge.svg)](https://docs.rs/cuid/)

This repository is the home of the [cuid], [cuid1], and [cuid2]
crates. The original CUID standard is marked as
[deprecated](https://github.com/paralleldrive/cuid2#improvements-over-cuid)
in favor of CUID2, but we intent to continue supporting v1 CUIDs
indefinitely, since their k-sortability is only insecure in specific
use-cases, while their sortability is quite useful in other
use-cases.

The `cuid` crate provides a wrapper around both versions:
- https://github.com/mplanchard/cuid-rust/tree/master/crates/cuid

In addition, both `cuid1` and `cuid2` may be used as standalone crates:
- https://github.com/mplanchard/cuid-rust/tree/master/crates/cuid1
- https://github.com/mplanchard/cuid-rust/tree/master/crates/cuid2

Please see the individual crates' READMEs for more information.

If you are upgrading to `cuid` 2.0, you must swap usages of non-version-specific
calls like `cuid()` for the desired version, i.e. `cuid1()` or `cuid2()`.

## Development

A [`flake.nix`](https://nixos.wiki/wiki/Flakes) file is provided for easy
installation of dependencies via the nix package manager. To start a shell with
all of the dependencies available, run:

```text
nix develop
```

Or, if you are not yet using flakes, a `shell.nix` shim is provided, so you can
do the same with

```text
nix-shell
```

If you use [direnv](https://direnv.net/), the `.envrc` file will automatically
source the nix packages into your shell (or editor environment, if you use a
direnv plugin).

[cuid]: https://crates.io/crates/cuid/
[cuid1]: https://crates.io/crates/cuid1/
[cuid2]: https://crates.io/crates/cuid2/
