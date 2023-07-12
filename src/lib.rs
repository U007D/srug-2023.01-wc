#[cfg(test)]
mod unit_tests;

mod error;

use std::io::BufRead;

pub use error::{Error, Result};

pub fn count_words(source: impl BufRead) -> Result<usize> {
    source.lines().try_fold(0_usize, |total, line| {
        let line_count = line?
            .split_whitespace()
            .try_fold(0_usize, |line_total, _| {
                line_total.checked_add(1).ok_or(Error::CountOverflow)
            })?;
        total.checked_add(line_count).ok_or(Error::CountOverflow)
    })
}
