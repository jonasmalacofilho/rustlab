//! A string error that supports the question mark operator.
//!
//! # Examples
//!
//! Propagating errors is ergonomic, but requires that foreign error types to be preemptively
//! marked with a special trait.
//!
//! ```
//! # use std::fs;
//! # use std::io;
//! # use std::num;
//! # use std::error::Error;
//! # use errors::newtype_string_error::{StringError, ExpectedExternalError};
//! fn open_and_parse_file(file_name: &str) -> Result<i32, StringError> {
//!     let mut contents = fs::read_to_string(&file_name)?;
//!     let num: i32 = contents.trim().parse()?;
//!     Ok(num)
//! }
//! ```
//!
//! Returning new string errors is also **not** trivial:
//! ```
//! assert!(false); // FIXME
//! ```

use std::error::Error;
use std::fmt::{self, Display};

/// A string error that supports the question mark operator.
///
/// The newtype pattern is used to allow us to implement the [`Error`] and [`From`] traits, because
/// of the orphan rules and neither of these traits being local.
///
/// Unlike [`StringifiedError`], this does implement the `Error` trait.
///
/// [`Error`]: https://doc.rust-lang.org/stable/std/error/trait.Error.html
/// [`From`]: https://doc.rust-lang.org/stable/std/convert/trait.From.html
/// [`StringifiedError`]: ../newtype_string/struct.StringifiedError.html
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct StringError(String);

impl Error for StringError {}

impl Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<T> for StringError
where
    T: Error + ExpectedExternalError,
{
    fn from(err: T) -> Self {
        StringError(err.to_string())
    }
}

/// A marker trait used for automatic conversion of concrete errors.
///
/// Without this separate yet identical implementations of `From::from` would be required.
pub trait ExpectedExternalError {}

impl ExpectedExternalError for std::num::ParseIntError {}
impl ExpectedExternalError for std::num::TryFromIntError {}
impl ExpectedExternalError for std::io::Error {}
impl ExpectedExternalError for String {}
impl ExpectedExternalError for &str {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    fn wrap<F, T, E>(f: F) -> Result<T, StringError>
    where
        F: Fn() -> Result<T, E>,
        E: Error + ExpectedExternalError + 'static,
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
