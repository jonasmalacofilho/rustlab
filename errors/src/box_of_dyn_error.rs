//! Box all errors.
//!
//! # Examples
//!
//! The `Err` variant holds a `Box` of types that implement `Error`, which makes
//! propagating errors ergonomic and effortless:
//!
//! ```
//! # use std::fs;
//! # use std::io;
//! # use std::num;
//! # use std::error::Error;
//! fn open_and_parse_file(file_name: &str) -> Result<i32, Box<dyn Error>> {
//!     let mut contents = fs::read_to_string(&file_name)?;
//!     let num: i32 = contents.trim().parse()?;
//!     Ok(num)
//! }
//! ```
//!
//! On the other hand, returning new errors is more involved and requires defining proper error
//! types, which is somewhat incompatible with the use of a crude error handling mechanism like
//! `Box<dyn Error>`:
//!
//! ```
//! # use std::error::Error;
//! # use std::fmt;
//! #[derive(Debug)]
//! struct SorryDave;
//!
//! impl fmt::Display for SorryDave {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         write!(f, "I'm sorry Dave.  I'm afraid I can't do that.")
//!     }
//! }
//!
//! impl Error for SorryDave {}
//!
//! fn foo() -> Result<(), Box<dyn Error>> {
//!     Err(Box::new(SorryDave {}))
//! }
//! ```

use std::error::Error;

/// A simple box of any type that implements [`Error`].
///
/// [`Error`]: https://doc.rust-lang.org/stable/std/error/trait.Error.html
pub type BoxedError = Box<dyn Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::fmt::Display;

    fn wrap<F, T, E>(f: F) -> Result<T, BoxedError>
    where
        F: Fn() -> Result<T, E>,
        E: Error + 'static,
    {
        Ok(f()?)
    }

    fn stringify<T>(res: Result<T, BoxedError>) -> String
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
