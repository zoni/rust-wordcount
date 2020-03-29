use std::io::prelude::*;
use std::io::BufReader;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WordCountError {
    #[error("Wordcount not implement yet")]
    NotImplemented,

    #[error("Read error")]
    ReadError { source: std::io::Error },

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub fn count_words<R: Read>(input: &mut R) -> Result<u32, WordCountError> {
    let reader = BufReader::new(input);
    let mut wordcount: u32 = 0;
    for line in reader.lines() {
        for _word in line
            .map_err(|source| WordCountError::ReadError { source })?
            .split_whitespace()
        {
            wordcount += 1;
        }
    }
    Ok(wordcount)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::{self, ErrorKind};

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
    fn count_words_in_file() {
        let mut f = File::open("words.txt").unwrap();
        let wordcount = count_words(&mut f).unwrap();
        assert_eq!(wordcount, 50);
    }

    #[test]
    fn returns_wordcounterror_on_pipeerror() {
        let mut f = ErrReader::new(ErrorKind::BrokenPipe, "read: broken pipe");
        let err = count_words(&mut f).unwrap_err();
        assert_matches!(err, WordCountError::ReadError{..});
    }

    #[test]
    fn returns_wordcounterror_on_other_error() {
        let mut f = ErrReader::new(ErrorKind::TimedOut, "read: timeout");
        let err = count_words(&mut f).unwrap_err();
        assert_matches!(err, WordCountError::ReadError{..});
    }
}
