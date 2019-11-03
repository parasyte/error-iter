# `error-iter`

Use [`Error::iter_chain`](https://doc.rust-lang.org/stable/std/error/trait.Error.html#method.iter_chain) and [`Error::iter_sources`](https://doc.rust-lang.org/stable/std/error/trait.Error.html#method.iter_sources) on stable Rust.

## MSRV

Supported Rust 1.37.0 and higher.

Compiles on Rust 1.31.0, but does not return the tail source. (Tests fail on anything less than 1.37.0.)

## What is this?

`iter_chain` and `iter_sources` are incredibly useful for providing context on error in Rust applications. For motivation, see [RFC 2504](https://github.com/rust-lang/rfcs/blob/master/text/2504-fix-error.md). These iterators are available in nightly compilers with [#58520](https://github.com/rust-lang/rust/issues/58520) tracking stabilization.

This crate does not attempt to be 100% compatible with the stabilization effort, but does want to provide very similar functionality to stable Rust.

## Show me

`cargo run --example simple`

```rust
use error_iter::ErrorIter;
use std::io::{Error as IoError, ErrorKind};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("I/O Error")]
    Io(#[from] IoError),

    #[error("Unknown error")]
    Unknown,
}

impl ErrorIter for Error {}

fn main () {
    let error = Error::from(IoError::new(ErrorKind::Other, "oh no!"));

    eprintln!("Error: {}", error);
    for source in iter_sources(&error) {
        eprintln!("  Caused by: {}", source);
    }
}
```

## Why not a derive macro?

PRs welcome! :)
