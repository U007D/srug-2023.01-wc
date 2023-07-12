#[cfg(test)]
mod unit_tests;

use std::ffi::OsString;

use clap::clap_derive::Parser;

#[derive(Clone, Debug, Parser, PartialEq)]
pub struct Args {
    pub source_file: OsString,
}

