use error_iter::ErrorExt as _;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("I/O Error")]
    Io(#[from] io::Error),

    #[error("Unknown error")]
    _Unknown,
}

fn main() {
    let inner = io::Error::new(io::ErrorKind::Other, "oh no!");
    let error = Error::from(inner);

    eprintln!("Error: {}", error);
    for source in error.sources().skip(1) {
        eprintln!("  Caused by: {}", source);
    }
}
