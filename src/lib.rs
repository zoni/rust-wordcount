#[cfg(test)]
#[macro_use]
extern crate assert_matches;

pub mod wordcounter;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use crate::wordcounter::count_words;
use anyhow::Context;

pub fn run() -> anyhow::Result<()> {
    let filename = env::args().nth(1).unwrap_or(String::from("-"));
    let mut f = match filename.as_str() {
        // https://stackoverflow.com/questions/26378842/how-do-i-overcome-match-arms-with-incompatible-types-for-structs-implementing-sa
        "-" => Box::new(std::io::stdin()) as Box<dyn Read>,
        _ => Box::new(File::open(&filename).context(format!("unable to open '{}'", filename))?)
            as Box<dyn Read>,
    };

    let wordcount = count_words(&mut f).context("unable to count words")?;
    println!("{} {}", wordcount, filename);
    Ok(())
}
