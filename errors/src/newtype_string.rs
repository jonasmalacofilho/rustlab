//! A string wrapper that supports the question mark operator.
//!
//! # Examples
//!
//! Propagating errors is ergonomic and requires no effort.
//!
//! ```
//! # use std::fs;
//! # use std::io;
//! # use std::num;
//! # use std::error::Error;
//! # use errors::newtype_string::StringifiedError;
//! fn open_and_parse_file(file_name: &str) -> Result<i32, StringifiedError> {
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

/// A string that supports the question mark operator.
///
/// The newtype pattern is used to allow us to implement the [`From`] trait, since it is not local
/// and because of the orphan rules.
///
/// Unlike [`StringError`], this does not implement [`Error`].  The advantage is not requiring the
/// expected errors to be marked; on the other hand, it is even less suitable for library modules.
/// That said, `StringError` was not very well suited for that task either.
///
/// [`Error`]: https://doc.rust-lang.org/stable/std/error/trait.Error.html
/// [`From`]: https://doc.rust-lang.org/stable/std/convert/trait.From.html
/// [`StringError`]: ../newtype_string_error/struct.StringError.html
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct StringifiedError(String);

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

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
