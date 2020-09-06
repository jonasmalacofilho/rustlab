#![cfg(test)]

mod compare {
    #[test]
    fn starts_with() {
        assert!("foo bar baz".starts_with("foo"));
    }

    #[test]
    fn case_insensitive_equality() {
        assert!("Β".to_lowercase() == "β");
    }

    #[test]
    fn ascii_insensitive_equality() {
        assert!("Foo".eq_ignore_ascii_case("foo"));
    }

    #[test]
    fn is_one_of_some_alternatives() {
        assert!(["foo", "bar", "baz"].iter().any(|&x| x == "bar"));
    }

    #[test]
    fn is_one_of_many_alternatives() {
        use std::collections::HashSet;
        let allowlist:HashSet<&str> = vec!["foo", "bar", "baz"].into_iter().collect();

        assert!(allowlist.contains("bar"));
    }

    #[test]
    fn contains() {
        assert!("foo bar baz".contains("bar"));
    }

    #[test]
    fn find() {
        assert_eq!("foo bar baz".find("bar"), Some(4));
    }
}

mod format {
    #[test]
    fn smoke_test() {
        assert_eq!(format!("foo: {}={}", "bar", 42), "foo: bar=42".to_string());
    }

    #[test]
    fn hexadecimal_integer() {
        assert_eq!(format!("{:#06x}", 0x0f00), "0x0f00".to_string());
    }

    #[test]
    fn join() {
        assert_eq!(["foo", "bar", "baz"].join(" "), "foo bar baz");
    }

    #[test]
    fn dynamic_format() {
        todo!();
    }

    #[test]
    fn alignment() {
        todo!();
    }
}
