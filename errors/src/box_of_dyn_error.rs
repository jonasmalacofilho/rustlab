//! When getting started, boxing all errors can be a temporary solution.
//!
//! This may also be and acceptable implementation for simple CLI utilities.
//!
//! # Examples
//!
//! Strings, string slices and other types for which there already are available conversions into
//! Error can be used as erros:
//!
//! ```
//! # use std::error::Error;
//! fn foo() -> Result<(), Box<dyn Error>> {
//!     Err("I'm sorry Dave".into())
//! }
//!
//! fn bar(name: &str) -> Result<(), Box<dyn Error>> {
//!     Err(format!("I'm sorry {}", name).into())
//! }
//! ```
//!
//! It is also possible to use "proper" errors, that implement the `Error` trait:
//!
//! ```
//! # use std::error::Error;
//! # use std::fmt;
//! #[derive(Debug)]
//! enum HalError {
//!     SorryDave,
//! }
//!
//! impl fmt::Display for HalError {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         match self {
//!             HalError::SorryDave => write!(f, "I'm sorry Dave"),
//!         }
//!     }
//! }
//!
//! impl Error for HalError {}
//!
//! fn foo() -> Result<(), Box<dyn Error>> {
//!     Err(HalError::SorryDave.into())
//! }
//!
//! if let Err(err) = foo() {
//!     println!("Error: {}", err);
//! }
//! ```
//!
//! Because the `Err` variant holds a `Box` of types that implement `Error`, propagating foreign
//! errors is ergonomic and effortless:
//!
//! ```
//! # use std::error::Error;
//! # use std::fs;
//! fn open_and_parse_file(file_name: &str) -> Result<i32, Box<dyn Error>> {
//!     let mut contents = fs::read_to_string(&file_name)?;
//!     let num: i32 = contents.trim().parse()?;
//!     Ok(num)
//! }
//! ```
//!
//! Boxing prevents generic inspection of the inner error type, but it is still possible to
//! tentatively downcast to specific concrete types of interest:
//!
//! ```
//! # use std::error::Error;
//! use std::num::ParseIntError;
//!
//! fn parse_positive_integer(input: &str) -> Result<u128, Box<dyn Error>> {
//!     let num = input.parse::<u128>()?;
//!     if num == 0 {
//!         return Err("zero is not a positive integer".into());
//!     }
//!     Ok(num)
//! }
//!
//! match parse_positive_integer("f") {
//!     Err(err) if err.downcast_ref::<ParseIntError>().is_some() =>
//!         println!("Error: could not parse"),
//!     Err(err) =>
//!         println!("Error: {}", err),
//!     Ok(num) =>
//!         println!("Successfully parsed {}", num),
//! };
//! ```
