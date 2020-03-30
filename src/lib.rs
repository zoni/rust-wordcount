#[cfg(test)]
#[macro_use]
extern crate assert_matches;

pub mod wordcounter;

use std::env;
use std::fs::File;

use crate::wordcounter::count_words;
use anyhow::Context;

pub fn run() -> anyhow::Result<()> {
    for filename in env::args().skip(1).collect::<Vec<String>>() {
        let mut reader = File::open(&filename).context(format!("unable to open '{}'", filename))?;
        let wordcount =
            count_words(&mut reader).context(format!("unable to count words in '{}'", filename))?;
        println!("{} {}", wordcount, filename);
    }
    Ok(())
}
