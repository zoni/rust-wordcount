//! This crate provides the `rwc` (rust word count) binary, named after the `wc
//! -w` Unix utility.
//!
//! This is intended as an example of how I approach error handling in Rust
//! applications with a focus of different error handling strategies for
//! application and library code.

fn main() {
    if let Err(err) = wordcount::run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
