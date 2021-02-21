use std::str::pattern::Pattern;

/// Extracts tokens from a string, according to `delim` pattern.
///
/// Mostly an exercise on lifetime variance, inspired by Jon Gjengset's
/// [Crust of Rust: Subtyping and Variance].
///
/// Still, this mimics the behavior of POSIX `strtok`, which additional pattern alternatives as
/// supported in `str::matches` and `str::find`.
///
/// [Crust of Rust: Subtyping and Variance]: https://www.youtube.com/watch?v=iVYWDIW71jk&t=1548s
pub fn strtok<'a, P>(str: &mut &'a str, delim: P) -> Option<&'a str>
where
    P: Pattern<'a>,
{
    let mut split = str.split(delim);

    while let Some(token) = split.next() {
        if token.len() > 0 {
            *str = split.as_str();
            return Some(token);
        }
    }
    None
}

/// The original issue started with more rudimentary lifetime bounds.
///
/// Both the original `&str` and the reference to it are bound to `'u`.  But, `&'x mut T` is
/// invariant in `T`, so if supplied with some `&'i str`, then `'u` must be equal to `'i`.
///
/// In other words, the return will need to leave at least as much as the input &str, making it
/// impractical to use (more than once).
#[allow(dead_code)]
#[allow(unused_variables)]
fn unusable_strtok<'u, P>(str: &'u mut &'u str, delim: P) -> Option<&'u str>
where
    P: Pattern<'u>,
{
    *str = str;
    None
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn would_not_compile() {
    let string = String::from("Hello World");

    let mut input = string.as_str();

    {
        let token = unusable_strtok(&mut input, ' ');
        // - input: &'would_not_compile mut &'would_not_compile str
        // - token: Option<&'would_not_compile str>
        // - token must live as long as string ('would_not_compile)
        // - as long as token lives, it mutably borrows the input
    }

    // Thus, the following would not compile:

    // let x = input; // cannot use `input` because it was mutably borrowed

    // even though we wrapped the previous borrow in a block
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_char_delim() {
        let mut input = "Hello World";
        let delim = " ";

        assert_eq!(strtok(&mut input, delim), Some("Hello"));
        assert_eq!(strtok(&mut input, delim), Some("World"));
        assert_eq!(strtok(&mut input, delim), None);

        assert_eq!(input, "");
    }

    #[test]
    fn works_with_char_slice() {
        let mut input = "Hello, World!";
        let delim = [' ', ',', '!'].as_ref();

        assert_eq!(strtok(&mut input, delim), Some("Hello"));
        assert_eq!(strtok(&mut input, delim), Some("World"));
        assert_eq!(strtok(&mut input, delim), None);

        assert_eq!(input, "");
    }
}
