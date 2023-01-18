# cuid-rust

[![Build Status](https://github.com/mplanchard/cuid-rust/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/mplanchard/cuid-rust/actions/workflows/ci.yml?query=branch%3Amaster)
[![Crates.io](https://img.shields.io/crates/v/cuid "Crates.io")](https://crates.io/crates/cuid2/)
[![docs.rs](https://docs.rs/cuid/badge.svg)](https://docs.rs/cuid2/badge.svg)

This repository is the home of the [cuid] and [cuid2] crates. The original
CUID standard is marked as [deprecated](https://github.com/paralleldrive/cuid2#improvements-over-cuid)
in favor of CUID2, so please prefer the `cuid2` crate.

If you are using the `cuid` crate already, you can also use the `cuid2()`
function from that crate.

Please see the individual crates' READMEs for more information.

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
[cuid2]: https://crates.io/crates/cuid2/
