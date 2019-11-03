use error_iter::ErrorIter;
use std::io::{Error as IoError, ErrorKind};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("I/O Error")]
    Io(#[from] IoError),

    #[error("Unknown error")]
    _Unknown,
}

impl ErrorIter for Error {}

fn main() {
    let error = Error::from(IoError::new(ErrorKind::Other, "oh no!"));

    eprintln!("Error: {}", error);
    for source in error.iter_sources() {
        eprintln!("  Caused by: {}", source);
    }
}
