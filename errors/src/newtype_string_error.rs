#![allow(dead_code)]
#![allow(unused_imports)]

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display};

/// A string error that supports the question mark operator.
///
/// The newtype pattern is used to allow us to implement the Error and From traits, because of the
/// orphan rules and neither of these traits being local.
///
/// Unlike `newtype_string::StringifiedError`, this does implement the Error trait.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct StringError(String);

impl Error for StringError {}

impl Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<T> for StringError
where
    T: ExpectedExternalError,
{
    fn from(err: T) -> Self {
        StringError(err.to_string())
    }
}

/// A marker trait used for automatic conversion of concrete errors.
///
/// Without this separate yet identical implementations of `From::from` would be required.
trait ExpectedExternalError: Error {}

impl ExpectedExternalError for std::num::ParseIntError {}
impl ExpectedExternalError for std::num::TryFromIntError {}

fn wrap<F, T, E>(f: F) -> Result<T, StringError>
where
    F: Fn() -> Result<T, E>,
    E: ExpectedExternalError + 'static,
{
    Ok(f()?)
}

fn stringify<T>(res: Result<T, StringError>) -> String
where
    T: Display,
{
    match res {
        Ok(val) => format!("Success: {}", val),
        Err(err) => format!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_works() {
        assert!(wrap(|| "42".parse::<i32>()).is_ok());

        assert!(wrap(|| "fx".parse::<i32>()).is_err());
        assert!(wrap(|| u8::try_from(-42)).is_err());
    }

    #[test]
    fn stringifying_works() {
        assert_eq!("Success: 42", stringify(wrap(|| "42".parse::<i32>())));

        assert_eq!(
            "Error: invalid digit found in string",
            stringify(wrap(|| "fx".parse::<i32>()))
        );
        assert_eq!(
            "Error: out of range integral type conversion attempted",
            stringify(wrap(|| u8::try_from(-42)))
        );
    }
}
