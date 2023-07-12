#![allow(clippy::unwrap_used)]

use assert2::{assert, let_assert};

#[allow(unused_imports)]
use super::*;
use clap::Parser;

#[test]
fn empty_command_line_gives_error() {
    /* Given */
    let command_line_args = Vec::<OsString>::new().into_iter();
    let expected_res = clap::error::ErrorKind::MissingRequiredArgument;

    /* When */
    let res = <Args as Parser>::try_parse_from(command_line_args);

    /* Then */
    let_assert!(Err(err) = res);
    assert!(err.kind() == expected_res)
}

#[test]
fn missing_cli_arg_gives_error() {
    /* Given */
    let command_line_args = vec!["command"].into_iter();
    let expected_res = clap::error::ErrorKind::MissingRequiredArgument;

    /* When */
    let res = <Args as Parser>::try_parse_from(command_line_args);

    /* Then */
    let_assert!(Err(err) = res);
    assert!(err.kind() == expected_res)
}

#[test]
fn two_cli_args_gives_error() {
    /* Given */
    let command_line_args = vec!["command", "arg1", "arg2"].into_iter();
    let expected_res = clap::error::ErrorKind::UnknownArgument;

    /* When */
    let res = <Args as Parser>::try_parse_from(command_line_args);

    /* Then */
    let_assert!(Err(err) = res);
    assert!(err.kind() == expected_res)
}

#[test]
fn one_cli_arg_succeeds() {
    /* Given */
    let command_line_args = vec!["command", "abc123.xyz"].into_iter();
    let expected_res = Args { source_file: "abc123.xyz".into()};

    /* When */
    let res = <Args as Parser>::try_parse_from(command_line_args);

    /* Then */
    let_assert!(Ok(arg) = res);
    assert!(arg == expected_res);
}
