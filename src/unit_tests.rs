#![allow(clippy::unwrap_used)]

use assert2::{assert, let_assert};
#[allow(unused_imports)]
use super::*;

#[test]
fn empty_input_returns_count_of_0() {
    /* Given */
    let expected_res = 0;
    let input = "".as_bytes();

    /* When */
    let res = count_words(input);

    /* Then */
    let_assert!(Ok(count) = res);
    assert!(count == expected_res);
}

#[test]
fn single_word_input_returns_count_of_1() {
    /* Given */
    let expected_res = 1;
    let input = "hello".as_bytes();

    /* When */
    let res = count_words(input);

    /* Then */
    let_assert!(Ok(count) = res);
    assert!(count == expected_res);
}


#[test]
fn two_word_input_returns_count_of_2() {
    /* Given */
    let expected_res = 2;
    let input = "hello, world!".as_bytes();

    /* When */
    let res = count_words(input);

    /* Then */
    let_assert!(Ok(count) = res);
    assert!(count == expected_res);
}