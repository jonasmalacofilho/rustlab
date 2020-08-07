#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn out_of_bounds_slices_panic() {
        let empty = "";
        &empty[0..1];
    }

    #[test]
    fn check_for_digits() {
        let s = "123";
        assert_eq!(s.chars().next().map(|x| x.is_digit(10)), Some(true));

        let s = "123foo";
        assert_eq!(
            s.chars().filter(|x| x.is_digit(10)).collect::<Vec<_>>(),
            ['1', '2', '3']
        );
    }
}
