//! Using plain strings for errors is possible, but not ergonomic.
//!
//! # Examples
//!
//! The `Err` variant simply holds a `String`:
//!
//! ```
//! fn foo() -> Result<(), String> {
//!     Err("I'm sorry, Dave".to_string())
//! }
//!
//! if let Err(err) = foo() {
//!     println!("Error: {}", err);
//! }
//! ```
//!
//! Because `String` does not implement `From<Error>`, the question mark operator cannot be used to
//! propagate foreign errors without first converting them explicitly:
//!
//! ```
//! # use std::fs;
//! fn open_and_parse_file(file_name: &str) -> Result<i32, String> {
//!     let mut contents = fs::read_to_string(&file_name).map_err(|e| e.to_string())?;
//!     let num = contents.trim().parse::<i32>().map_err(|e| e.to_string())?;
//!     Ok(num)
//! }
//! ```
//!
//! Additionally, after the conversion errors can no longer be differentiated in a typed manner,
//! leaving only string operations for this task:
//!
//! ```
//! fn parse_positive_integer(input: &str) -> Result<u128, String> {
//!     let num = input.parse::<u128>().map_err(|e| e.to_string())?;
//!     if num == 0 {
//!         return Err("zero is not a positive integer".into());
//!     }
//!     Ok(num)
//! }
//!
//! assert!(matches!(parse_positive_integer("f"), Err(e) if e.starts_with("invalid digit")));
//! assert!(matches!(parse_positive_integer("0"), Err(e) if e.starts_with("zero is not a")));
//! ```
