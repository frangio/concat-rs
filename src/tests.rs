#![cfg(test)]

use super::concat;

use std::io::{Read, Cursor};
use std::iter::IntoIterator;

#[test]
fn it_concats() {
    let cursors = vec!["foo", "bar"].into_iter().map(str::as_bytes).map(Cursor::new);
    let mut concatd = String::new();

    concat(cursors).read_to_string(&mut concatd).unwrap();

    assert_eq!("foobar", concatd);
}
