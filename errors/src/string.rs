//! Just use plain strings for errors.
//!
//! # Examples
//!
//! The `Err` variant simply holds a `String`:
//!
//! ```
//! fn foo() -> Result<(), String> {
//!     Err("I'm sorry, Dave.  I'm afraid I can't do that.".to_string())
//! }
//! ```
//!
//! Because `String` does not implement `From<Error>`, the question mark operator cannot be used to
//! propagate foreign errors without first converting them explicitly:
//!
//! ```
//! # use std::fs;
//! # use std::io;
//! # use std::num;
//! fn open_and_parse_file(file_name: &str) -> Result<i32, String> {
//!     let mut contents = fs::read_to_string(&file_name).map_err(|e| e.to_string())?;
//!     let num = contents.trim().parse::<i32>().map_err(|e| e.to_string())?;
//!     Ok(num)
//! }
//! ```

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::error::Error;
    use std::fmt::Display;

    fn wrap<F, T, E>(f: F) -> Result<T, String>
    where
        F: Fn() -> Result<T, E>,
        E: Error + 'static,
    {
        Ok(f().map_err(|e| e.to_string())?) // annoying
    }

    fn stringify<T>(res: Result<T, String>) -> String
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
