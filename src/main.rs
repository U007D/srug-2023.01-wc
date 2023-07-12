mod args;

use std::fs::File;
use std::io::BufReader;

use crate::args::Args;
use clap::Parser;

use lib::{count_words, Result};

fn main() -> Result<()> {
    let arg = Args::try_parse()?;
    let source = File::open(arg.source_file)?;
    let word_count = count_words(BufReader::new(source))?;

    println!("The file contained {word_count} words.");

    Ok(())
}
