#![allow(dead_code)]
#![allow(unused_imports)]

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display};

/// A string that supports the question mark operator.
///
/// The newtype pattern is used to allow us to implement the From trait, because of the orphan
/// rules and this trait not being local.
///
/// Unlike `newtype_string_error::StringError`, this does not implement Error.  This has the
/// advantage or not requiring the expected errors to be marked.  On the other hand, it is even
/// less suitable for library modules.  That said, `StringError` was not very well suited for that
/// task either.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct StringifiedError(String);

impl Display for StringifiedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<T> for StringifiedError
where
    T: Error,
{
    fn from(err: T) -> Self {
        StringifiedError(err.to_string())
    }
}

fn wrap<F, T, E>(f: F) -> Result<T, StringifiedError>
where
    F: Fn() -> Result<T, E>,
    E: Error + 'static,
{
    Ok(f()?)
}

fn stringify<T>(res: Result<T, StringifiedError>) -> String
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
