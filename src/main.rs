#![feature(iter_map_while)]
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    for filename in env::args().skip(1).collect::<Vec<String>>() {
        let file = File::open(&filename).context(format!("unable to open '{}'", filename))?;
        let reader = BufReader::new(file);

        let mut err = None;
        let mut iter = reader
            .lines()
            .map_while(|line| line.map_err(|e| err = Some(e)).ok());
        let wordcount = wordcount::count_words(&mut iter);
        if let Some(err) = err {
            return Err(
                anyhow::Error::new(err).context(format!("unable to count words in '{}'", filename))
            );
        }
        println!("{} {}", wordcount, filename);
    }
    Ok(())
}
