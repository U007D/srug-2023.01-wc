mod args;

use std::process::ExitCode;
use std::{fs::File, io::BufReader};

use clap::Parser;
use lib::{count_words, Result};

use crate::args::Args;

fn inner_main() -> Result<()> {
    let arg = Args::try_parse()?;
    let source = File::open(&arg.source_file)?;
    let word_count = count_words(BufReader::new(source))?;

    let filename = arg.source_file.to_string_lossy();
    println!("The file `{filename}` contains {word_count} words.");

    Ok(())
}

fn main() -> ExitCode {
    match inner_main() {
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}
