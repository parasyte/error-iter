//! Iterators over `std::error::Error` sources on stable Rust.
//!
//! ```
//! use error_iter::ErrorIter;
//! use std::io::{Error as IoError, ErrorKind};
//! use thiserror::Error;
//!
//! #[derive(Debug, Error)]
//! enum Error {
//!     #[error("I/O Error")]
//!     Io(#[from] IoError),
//!
//!     #[error("Unknown error")]
//!     Unknown,
//! }
//!
//! impl ErrorIter for Error {}
//!
//! fn do_something() {
//!     let error = Error::from(IoError::new(ErrorKind::Other, "oh no!"));
//!
//!     eprintln!("Error: {}", error);
//!     for source in error.chain().skip(1) {
//!         eprintln!("  Caused by: {}", source);
//!     }
//! }
//! ```

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]

pub struct ErrorIterator<'a> {
    inner: Option<&'a (dyn std::error::Error + 'static)>,
}

impl<'a> Iterator for ErrorIterator<'a> {
    type Item = &'a (dyn std::error::Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(error) = self.inner.take() {
            self.inner = error.source();
            return Some(error);
        }

        None
    }
}

/// Implement this trait on your error types for free iterators over their sources!
///
/// The default implementation provides iterators for any type that implements `std::error::Error`.
pub trait ErrorIter: std::error::Error + Sized + 'static {
    /// Create an iterator over the error and its chained sources.
    ///
    /// ```
    /// use error_iter::ErrorIter;
    /// use thiserror::Error;
    ///
    /// #[derive(Debug, Error)]
    /// enum Error {
    ///     #[error("Nested error: {0}")]
    ///     Nested(#[source] Box<Error>),
    ///
    ///     #[error("Leaf error")]
    ///     Leaf,
    /// }
    ///
    /// impl ErrorIter for Error {}
    ///
    /// let error = Error::Nested(Box::new(Error::Leaf));
    ///
    /// let mut iter = error.chain();
    ///
    /// assert_eq!("Nested error: Leaf error".to_string(), iter.next().unwrap().to_string());
    /// assert_eq!("Leaf error".to_string(), iter.next().unwrap().to_string());
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// ```
    fn chain(&self) -> ErrorIterator {
        ErrorIterator { inner: Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use thiserror::Error;

    #[derive(Debug, Error)]
    enum Error {
        #[error("Nested error: {0}")]
        Nested(#[source] Box<Error>),

        #[error("Leaf error")]
        Leaf,
    }

    impl ErrorIter for Error {}

    #[test]
    fn iter_chain_ok() {
        let error = Error::Nested(Box::new(Error::Nested(Box::new(Error::Leaf))));

        let mut iter = error.chain();

        assert_eq!(
            "Nested error: Nested error: Leaf error".to_string(),
            iter.next().unwrap().to_string()
        );
        assert_eq!(
            "Nested error: Leaf error".to_string(),
            iter.next().unwrap().to_string()
        );
        assert_eq!("Leaf error".to_string(), iter.next().unwrap().to_string());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }
}
