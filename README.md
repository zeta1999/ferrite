# Ferrite - Session Types DSL for Rust

[![Crates.io][crates-badge]][crates-url]
[![Apache licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/ferrite-session.svg
[crates-url]: https://crates.io/crates/ferrite-session
[license-badge]: https://img.shields.io/crates/l/ferrite-session.svg
[license-url]: https://github.com/ferrite-rs/ferrite/blob/master/LICENSE
[actions-badge]: https://github.com/ferrite-rs/ferrite/workflows/Cargo%20Tests/badge.svg
[actions-url]: https://github.com/ferrite-rs/ferrite/actions

## Overview

Ferrite is a DSL for writing session type programs in Rust.
This is an ongoing research work by [Soares Chen](https://maybevoid.com/)
and [Stephanie Balzer](http://www.cs.cmu.edu/~balzers/) to implement
session types in Rust.

## Documentation

A work-in-progress documentation for Ferrite is available as a
[book](https://maybevoid.com/ferrite-book/).

A draft technical report for Ferrite is currently available at
[Arxiv](https://arxiv.org/abs/2009.13619).

Auto generated rustdoc for Ferrite is available
[here](https://maybevoid.com/ferrite-doc/ferrite/index.html).

## Build Instructions

The library code requires nightly version of Rust to be compiled.
You can use `rustup` to install Rust nightly as follows:

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup default nightly
```

After that, the library can be build with `cargo` as follows:

```bash
cargo build
```

## Runnnig Demo

A number of demo executables are available in the [`src/bin`](src/bin) directory.
To run a particular demo, use `cargo run` with the name of the demo file.
For example:

```bash
cargo run --bin hello
cargo run --bin shared
```

## Code Organization

### `ferrite-session`

  - [`src/base`](ferrite-session/src/base) - Core constructs for Ferrite
  - [`src/functional`](ferrite-session/src/function) - Functional programming constructs such as type application and natural numbers.
  - [`src/protocol`](ferrite-session/src/protocol) - Type definitions for session types
  - [`src/session`](ferrite-session/src/session) - Term constructors
  - [`src/public.rs`](ferrite-session/src/public.rs) - Public API exposed by Ferrite

### Demo

  - [`src/bin`](ferrite-demo/src/bin) - Demo executables

## Acknowledgement

This material is based upon work supported by the National Science Foundation under Grant No. CCF-1718267.
Any opinions, findings, and conclusions or recommendations expressed in this material are those of the author(s)
and do not necessarily reflect the views of the National Science Foundation.
