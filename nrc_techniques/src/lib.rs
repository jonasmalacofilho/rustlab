#[cfg(test)]
mod test {
    #[test]
    fn options_rewrite_foo() {
        fn foo(input: Option<i32>) -> Option<i32> {
            input.filter(|&x| x >= 0)
        }

        assert_eq!(foo(None), None);
        assert_eq!(foo(Some(-1)), None);
        assert_eq!(foo(Some(42)), Some(42));
    }

    #[test]
    fn options_rewrite_bar() {
        type ErrNegative = ();
        fn bar(input: Option<i32>) -> Result<i32, ErrNegative> {
            input.ok_or(())
        }

        assert_eq!(bar(Some(42)), Ok(42));
        assert_eq!(bar(None), Err(()));
    }

    #[test]
    fn primitive_for_impl() {
        let mut out = vec![];

        let vec = vec![0, 1, 2, 3];
        let mut iter = (&vec).into_iter();
        loop {
            match iter.next() {
                Some(v) => out.push(format!("{}", v)),
                None => break,
            }
        }

        assert_eq!(out, vec!["0", "1", "2", "3"]);
    }

    #[test]
    fn empty_type_errors() {
        fn wrap_parse(text: &str) -> Result<u32, String> {
            let num = text.parse::<u32>().map_err(|e| e.to_string())?;
            Ok(num)
        }

        assert_eq!(wrap_parse("42"), Ok(42));
    }
}
