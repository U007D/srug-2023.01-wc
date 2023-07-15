mod args;

use std::{fs::File, io::BufReader};

use clap::Parser;
use lib::{count_words, Result};

use crate::args::Args;

fn inner_main() -> Result<()> {
    let arg = Args::try_parse()?;
    let source = File::open(arg.source_file)?;
    let word_count = count_words(BufReader::new(source))?;

    println!("The file contained {word_count} words.");

    Ok(())
}

fn main() {
    if let Err(error) = inner_main() {
        eprintln!("{error}")
    }
}
