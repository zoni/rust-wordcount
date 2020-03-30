//! This module contains the actual `count_words` implementation.
//!
//! Code inside this file is meant to demonstrate library code, that is code
//! which might be used as a library by other projects.
//!
//! Such code should use the regular [`Result<T, E>`] type to provide
//! interoperability, rather than forcing people to adopt a specific error
//! handling library.
//!
//! The errors returned by this library are variants of [`WordCountError`].
//! They implement the standard library's [`std::error::Error`] trait using a
//! convenient derive macro provided by [`thiserror`].
//!
//! [`Result<T, E>`]: https://doc.rust-lang.org/std/result/index.html
//! [`WordCountError`]: enum.WordCountError.html
//! [`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
//! [`thiserror`]: ../thiserror/index.html

use std::io::prelude::*;
use std::io::BufReader;
use thiserror::Error;

/// WordCountError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum WordCountError {
    /// Represents an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    #[error("Source contains no data")]
    EmptySource,

    /// Represents a failure to read from input.
    #[error("Read error")]
    ReadError { source: std::io::Error },

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

/// Count the number of words in the given input.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use wordcount::wordcounter;
///
/// let mut f = Cursor::new(String::from("foobar"));
/// let wordcount = wordcounter::count_words(&mut f).unwrap();
/// assert_eq!(wordcount, 1);
/// ```
pub fn count_words<R: Read>(input: &mut R) -> Result<u32, WordCountError> {
    let reader = BufReader::new(input);
    let mut wordcount = 0;
    for line in reader.lines() {
        let line = line.map_err(|source| WordCountError::ReadError { source })?;
        for _word in line.split_whitespace() {
            wordcount += 1;
        }
    }

    if wordcount == 0 {
        return Err(WordCountError::EmptySource);
    }

    Ok(wordcount)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor, ErrorKind};

    // ---------------------------------------------------------------------------------
    // Copied (with slight modifications) from:
    //   https://github.com/Leopard2A5/rust-io-test-util/blob/master/src/error_reader.rs
    //
    /// An implementation of `std::io::Read` that fails on the first call to `read` and
    /// throws an `std::io::Error` with the given ErrorKind and message.
    #[derive(Debug, PartialEq)]
    pub struct ErrReader<'a> {
        /// The ErrorKind to put into the `std::io::Error`.
        pub kind: ErrorKind,
        pub msg: &'a str,
    }

    impl<'a> ErrReader<'a> {
        /// Construct a new ErrReader.
        pub fn new(kind: ErrorKind, msg: &'a str) -> Self {
            Self { kind, msg }
        }
    }

    impl<'a> io::Read for ErrReader<'a> {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(self.kind, self.msg))
        }
    }
    // ---------------------------------------------------------------------------------

    #[test]
    fn count_single_word() {
        let mut f = Cursor::new(String::from("foobar"));
        let wordcount = count_words(&mut f).unwrap();
        assert_eq!(wordcount, 1);
    }

    #[test]
    fn count_multiple_words() {
        let mut f = Cursor::new(String::from("foo bar\nbaz"));
        let wordcount = count_words(&mut f).unwrap();
        assert_eq!(wordcount, 3);
    }

    #[test]
    fn empty_input() {
        let mut f = Cursor::new(String::from(""));
        let err = count_words(&mut f).unwrap_err();
        assert_matches!(err, WordCountError::EmptySource{..});
    }

    #[test]
    fn read_timeout() {
        let mut f = ErrReader::new(ErrorKind::TimedOut, "read: timeout");
        let err = count_words(&mut f).unwrap_err();
        assert_matches!(err, WordCountError::ReadError{..});
    }
}
