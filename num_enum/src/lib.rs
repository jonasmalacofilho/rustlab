#[cfg(test)]
mod tests {
    #[test]
    fn try_from_works() {
        use num_enum::TryFromPrimitive;
        use std::convert::TryFrom;

        #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
        #[repr(u8)]
        enum Number {
            Zero,
            One,
        }

        assert_eq!(Number::try_from_primitive(1), Ok(Number::One));
        assert_eq!(Number::try_from(1), Ok(Number::One));
        assert!(Number::try_from(42).is_err());
    }

    #[test]
    fn from_works() {
        use num_enum::FromPrimitive;
        use std::convert::From;

        #[derive(Debug, Eq, PartialEq, FromPrimitive)]
        #[repr(u8)]
        enum Number {
            Zero,
            One,
            #[num_enum(default)]
            NaN,
        }

        assert_eq!(Number::from_primitive(1), Number::One);
        assert_eq!(Number::from(1), Number::One);
        assert_eq!(Number::from(42), Number::NaN);
    }
}
