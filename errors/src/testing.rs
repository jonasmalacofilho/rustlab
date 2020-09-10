#![cfg(test)]
#![allow(dead_code)]
#![allow(unused_attributes)]

use std::io::{Error, ErrorKind};

fn fallible(arg: i32) -> Result<i32, Error> {
    if arg == 42 {
        Ok(42)
    } else {
        Err(Error::new(ErrorKind::Other, "oh no!"))
    }
}

#[test]
fn smoke_test() {
    assert_eq!(fallible(42).unwrap(), 42);
}

#[test]
#[cfg(feature = "fail-error-tests")]
fn error_with_is_ok() {
    assert!(fallible(3).is_ok());

    // not very useful:
    // thread 'testing::error_with_is_ok' panicked at 'assertion failed: fallible(3).is_ok()',
    // src/testing.rs:23:5
}

#[test]
#[cfg(feature = "fail-error-tests")]
fn error_with_unwrap() {
    assert_eq!(fallible(3).unwrap(), 42);

    // very useful, but a bit noisy:
    // thread 'testing::error_with_unwrap' panicked at 'called `Result::unwrap()` on an `Err`
    // value: Custom { kind: Other, error: "oh no!" }', src/testing.rs:33:28
}

#[test]
#[ignore]
#[cfg(feature = "fail-error-tests")]
fn error_with_partial_eq() {
    unimplemented!();

    // does not work for errors that don't impl PartialEq (like io::Error)
    // assert_eq!(fallible(3), Ok(42));
}

#[test]
#[cfg(feature = "fail-error-tests")]
fn error_as_test_result() -> Result<(), Error> {
    assert_eq!(fallible(3)?, 42);
    Ok(())

    // straight to the point, but doesn't immediately show the offending expression:
    // Error: Custom { kind: Other, error: "oh no!" }
}
