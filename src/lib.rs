#[cfg(test)]
mod unit_tests;

mod error;

use std::io::BufRead;

pub use error::{Error, Result};

pub fn count_words(source: impl BufRead) -> Result<usize> {
    source.lines().try_fold(0_usize, |total, line| {
        line?.split_whitespace().try_fold(total, |sub_total, _| {
            sub_total.checked_add(1).ok_or(Error::CountOverflow)
        })
    })
}
