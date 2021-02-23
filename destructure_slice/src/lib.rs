use std::mem::{self, MaybeUninit};

/// Return an array of references from a slice, for irrefutable destructing with `let`.
///
/// # Please do not use in the real world
///
/// This is just an experiment with `min_const_generics`, and the safety arguments are very
/// superficial.  Additionally, this bypasses the Rust design decision of differentiating between
/// refutable and irrefutable patterns.
///
/// # Examples
///
/// ```
/// use destructure_slice;
///
/// let data = vec![42, 3];
///
/// let [&first, &second] = destructure_slice::destructure(&data);
///
/// assert_eq!(first, 42);
/// assert_eq!(second, 3);
/// ```
///
/// # Panics
///
/// Panics if the slice does not have `N` length.
///
/// ```should_panic
/// use destructure_slice;
///
/// let [&first, &second, &third] = destructure_slice::destructure(&vec![42, 3]);  // panic!
/// ```
pub fn destructure<T, const N: usize>(slice: &[T]) -> [&T; N] {
    if slice.len() != N {
        panic!("incorrect number of elements in slice")
    }

    let mut ret = [MaybeUninit::<&T>::uninit(); N];

    for i in 0..N {
        ret[i] = MaybeUninit::new(&slice[i]);
    }

    assert_eq!(
        mem::size_of::<[MaybeUninit<&T>; N]>(),
        mem::size_of::<[&T; N]>()
    );

    // SAFETY:
    // - [T; N] has the same size as size_of::<T> * N and the same alignment as T
    // - both MaybeUninit<T> and T have the same layout (size, alignment and ABI)
    // - all elements have been initialized to valid values
    unsafe { mem::transmute_copy::<[MaybeUninit<&T>; N], [&T; N]>(&ret) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vanilla_rust_match_works() {
        let some_data = vec![1, 2, 3];

        let res = match &some_data[..] {
            &[foo, bar, baz] => Ok((foo, bar, baz)),
            _ => Err("wrong slice length"),
        };

        assert_eq!(res, Ok((1, 2, 3)));
    }

    #[test]
    fn vanilla_rust_match_catches_errors() {
        let some_data = vec![1, 2, 3];

        let fail = match &some_data[..] {
            [foo, bar] => Ok((foo, bar)),
            _ => Err("wrong slice length"),
        };
        assert_eq!(fail, Err("wrong slice length"));
    }

    #[test]
    fn it_works() {
        let some_data = vec![1, 2, 3];

        let [&foo, &bar, &baz] = destructure(&some_data[..]);
        assert_eq!((foo, bar, baz), (1, 2, 3));
    }

    #[test]
    fn catches_errors() {
        let some_data = vec![1, 2, 3];

        let fail = std::panic::catch_unwind(|| {
            let [&_foo, &_bar] = destructure(&some_data[..]);
        });
        assert!(fail.is_err());

        let fail = std::panic::catch_unwind(|| {
            let [&_foo, &_bar, &_baz, &_extra] = destructure(&some_data[..]);
        });
        assert!(fail.is_err());
    }

    #[test]
    fn handles_non_copy_types() {
        let some_data = vec![String::from("foo"), String::from("bar")];

        let [foo, bar] = destructure(&some_data[..]);
        assert_eq!((foo.as_str(), bar.as_str()), ("foo", "bar"));

        let fail = std::panic::catch_unwind(|| {
            let [_foo] = destructure(&some_data[..]);
        });
        assert!(fail.is_err());
    }
}
