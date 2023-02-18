use error_iter::ErrorIter as _;
use std::io::{Error as IoError, ErrorKind};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("I/O Error")]
    Io(#[from] IoError),

    #[error("Unknown error")]
    _Unknown,
}

fn main() {
    let error = Error::from(IoError::new(ErrorKind::Other, "oh no!"));

    eprintln!("Error: {}", error);
    for source in error.sources().skip(1) {
        eprintln!("  Caused by: {}", source);
    }
}
