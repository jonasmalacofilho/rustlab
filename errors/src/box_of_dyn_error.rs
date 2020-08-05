#![allow(dead_code)]
#![allow(unused_imports)]

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;

type BoxedError = Box<dyn Error>;

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
