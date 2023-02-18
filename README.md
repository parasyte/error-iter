# `error-iter`

[![Crates.io](https://img.shields.io/crates/v/error-iter)](https://crates.io/crates/error-iter "Crates.io version")
[![Documentation](https://img.shields.io/docsrs/error-iter)](https://docs.rs/error-iter "Documentation")
[![GitHub actions](https://img.shields.io/github/actions/workflow/status/parasyte/error-iter/ci.yml?branch=main)](https://github.com/parasyte/error-iter/actions "CI")
[![GitHub activity](https://img.shields.io/github/last-commit/parasyte/error-iter)](https://github.com/parasyte/error-iter/commits "Commit activity")
[![GitHub Sponsors](https://img.shields.io/github/sponsors/parasyte)](https://github.com/sponsors/parasyte "Sponsors")
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Use [`Error::sources`](https://doc.rust-lang.org/nightly/std/error/trait.Error.html#method.sources) on stable Rust.

## MSRV

Supports Rust 1.37.0 and higher.

Compiles on Rust 1.31.0, but does not return the tail source. (Tests fail on anything less than 1.37.0.)

## What is this?

`Error::sources` is incredibly useful for providing error context in Rust applications. For motivation, see [RFC 2504](https://github.com/rust-lang/rfcs/blob/master/text/2504-fix-error.md). This iterator is available in nightly compilers with [#58520](https://github.com/rust-lang/rust/issues/58520) tracking stabilization.

This crate does not attempt to be 100% compatible with the stabilization effort, but does want to provide very similar functionality to stable Rust.

## Show me

`cargo run --example simple`

## Why not a derive macro?

PRs welcome! :)
