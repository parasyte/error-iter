//! Iterators over `std::error::Error` sources on stable Rust.
//!
//! ```
//! use error_iter::ErrorExt as _;
//! use std::io;
//! use thiserror::Error;
//!
//! #[derive(Debug, Error)]
//! enum Error {
//!     #[error("I/O Error")]
//!     Io(#[from] io::Error),
//!
//!     #[error("Unknown error")]
//!     Unknown,
//! }
//!
//! fn do_something() {
//!     let inner = io::Error::new(io::ErrorKind::Other, "oh no!");
//!     let error = Error::from(inner);
//!
//!     eprintln!("Error: {}", error);
//!     for source in error.sources().skip(1) {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut count = 0_usize;
        let mut next = self.inner;
        while let Some(error) = next {
            match count.checked_add(1) {
                Some(new_count) => count = new_count,
                None => return (count, None),
            }

            next = error.source();
        }

        (count, Some(count))
    }
}

impl<'a> ExactSizeIterator for ErrorIterator<'a> {
    // TODO: Once stable, optimize by implementing `is_empty` manually.
    // See: https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html#method.is_empty
    // fn is_empty(&self) -> bool {
    //     self.inner.is_none()
    // }
}

impl<'a> std::iter::FusedIterator for ErrorIterator<'a> {}

/// Implement this trait on your error types for free iterators over their sources!
///
/// The default implementation provides iterators for any type that implements `std::error::Error`.
pub trait ErrorExt: std::error::Error + Sized + 'static {
    /// Create an iterator over the error and its recursive sources.
    ///
    /// ```
    /// use error_iter::ErrorExt as _;
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
    /// let error = Error::Nested(Box::new(Error::Leaf));
    ///
    /// let mut iter = error.sources();
    ///
    /// assert_eq!("Nested error: Leaf error".to_string(), iter.next().unwrap().to_string());
    /// assert_eq!("Leaf error".to_string(), iter.next().unwrap().to_string());
    /// assert!(iter.next().is_none());
    /// assert!(iter.next().is_none());
    /// ```
    fn sources(&self) -> ErrorIterator {
        ErrorIterator { inner: Some(self) }
    }
}

impl<T> ErrorExt for T where T: std::error::Error + Sized + 'static {}

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

    #[test]
    fn iter_sources_ok() {
        let error = Error::Nested(Box::new(Error::Nested(Box::new(Error::Leaf))));

        let mut iter = error.sources();

        assert_eq!((3, Some(3)), iter.size_hint());
        assert_eq!(
            "Nested error: Nested error: Leaf error".to_string(),
            iter.next().unwrap().to_string()
        );
        assert_eq!((2, Some(2)), iter.size_hint());
        assert_eq!(
            "Nested error: Leaf error".to_string(),
            iter.next().unwrap().to_string()
        );
        assert_eq!((1, Some(1)), iter.size_hint());
        assert_eq!("Leaf error".to_string(), iter.next().unwrap().to_string());
        assert_eq!((0, Some(0)), iter.size_hint());
        assert!(iter.next().is_none());
        assert_eq!((0, Some(0)), iter.size_hint());
        assert!(iter.next().is_none());
    }
}
