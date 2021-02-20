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
/// use desctruct_slice::des;
///
/// let data = vec![42, 3];
///
/// let [&first, &second] = des(&data);
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
/// use desctruct_slice::des;
///
/// let [&first, &second, &third] = des(&vec![42, 3]);  // panics
/// ```
/// let [&first, &second] = des(&data[..]);
pub fn des<T, const N: usize>(slice: &[T]) -> [&T; N] {
    if slice.len() != N {
        panic!("incorrect number of elements in slice")
    }

    // SAFETY: we are initializing a bunch of MaybeUninits, which do not require initialization
    // (see documentation of MaybeUninit)
    let mut ret: [MaybeUninit<&T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    for i in 0..N {
        ret[i] = MaybeUninit::new(&slice[i]);
    }

    // double check that the sizes are the same since we cannot use transmute::<_, [&T; N]>(ret)
    // because it claims that the types do not have a fixed size (possible issue with
    // min_const_generics?)
    assert_eq!(
        mem::size_of::<[MaybeUninit<&T>; N]>(),
        mem::size_of::<[&T; N]>()
    );

    // SAFETY: everything has been initialized by now, and both src and dst have the same size
    unsafe { mem::transmute_copy::<_, [&T; N]>(&ret) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vanilla_works() {
        let some_data = vec![1, 2, 3];

        let res = match &some_data[..] {
            &[foo, bar, baz] => Ok((foo, bar, baz)),
            _ => Err("wrong slice length"),
        };

        assert_eq!(res, Ok((1, 2, 3)));
    }

    #[test]
    fn vanilla_catches_errors() {
        let some_data = vec![1, 2, 3];

        let fail = match &some_data[..] {
            [foo, bar] => Ok((foo, bar)),
            _ => Err("wrong slice length"),
        };
        assert_eq!(fail, Err("wrong slice length"));
    }

    #[test]
    fn helper_works() {
        let some_data = vec![1, 2, 3];

        let [&foo, &bar, &baz] = des(&some_data[..]);
        assert_eq!((foo, bar, baz), (1, 2, 3));
    }

    #[test]
    fn helper_catches_errors() {
        let some_data = vec![1, 2, 3];

        let fail = std::panic::catch_unwind(|| {
            let [&_foo, &_bar] = des(&some_data[..]);
        });
        assert!(fail.is_err());

        let fail = std::panic::catch_unwind(|| {
            let [&_foo, &_bar, &_baz, &_extra] = des(&some_data[..]);
        });
        assert!(fail.is_err());
    }
}
