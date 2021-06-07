#![cfg_attr(feature = "nightly", feature(coerce_unsized, dropck_eyepatch, unsize))]

use std::{
    fmt::{Debug, Formatter, Result},
    marker::PhantomData,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

#[cfg(feature = "nightly")]
use std::{marker::Unsize, ops::CoerceUnsized};

pub struct Ark<T: ?Sized> {
    // Use the fact that the pointer is never null for a niche; this also makes Ark covariant over
    // ArkInner<T>, but that is also accomplished by `_phantom` bellow.
    ptr: NonNull<ArkInner<T>>,

    // Tell the compiler that even though we do not access T when *we* drop, we still drop
    // ArkInner<T>, and T could still perform an access when it drops.  Because `ptr` is just a
    // pointer, it is not sufficient for the drop checker to see that we (may) own an ArkInner<T>.
    _phantom: PhantomData<ArkInner<T>>,
}

struct ArkInner<T: ?Sized> {
    strong_count: AtomicUsize,
    data: T,
}

impl<T> Ark<T> {
    pub fn new(data: T) -> Ark<T> {
        let inner = ArkInner {
            strong_count: AtomicUsize::new(1),
            data,
        };

        let ptr = Box::into_raw(Box::new(inner));

        // Safety: a pointer created with Box::into_raw() cannot be null.
        Ark {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
            _phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> Ark<T> {
    fn inner(&self) -> &ArkInner<T> {
        // Safety: as the pointer was created with Box::into_raw(), we know that it is correctly
        // aligned, dereferenceable and that the value it points to is initialized.  Additionally,
        // it is valid as long as the strong_count is greater than zero, and we know that it must
        // be at least one because of our own &self reference.  And the aliasing is always
        // respected because we never mutate or take a mutable reference from it.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> Clone for Ark<T> {
    fn clone(&self) -> Self {
        // Ordering: there are no access to synchronize in this function and the inner struct is
        // not going anywhere because the &self reference implies that strong_count >= 1.
        self.inner().strong_count.fetch_add(1, Ordering::Relaxed);

        Ark {
            ptr: self.ptr,
            _phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> Deref for Ark<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner().data
    }
}

#[cfg(feature = "nightly")]
unsafe impl<#[may_dangle] T: ?Sized> Drop for Ark<T> {
    fn drop(&mut self) {
        drop_impl(self);
    }
}
#[cfg(not(feature = "nightly"))]
impl<T: ?Sized> Drop for Ark<T> {
    fn drop(&mut self) {
        drop_impl(self);
    }
}

fn drop_impl<T: ?Sized>(this: &mut Ark<T>) {
    // Ordering: the box must only be dropped after the store to strong_count.
    if this.inner().strong_count.fetch_sub(1, Ordering::AcqRel) == 1 {
        // Safety: pointer was created with Box::into_raw(), and is valid because strong_count
        // was still one; dropping he reconstructed Box is also safe because, since we are the
        // last Ark, this.ptr will not be used again and be left dangling.
        let _ = unsafe { Box::from_raw(this.ptr.as_ptr()) };
    }
    // TODO possibly optimize for the case where drop does *not* drop the contents
}

impl<T: Debug> Debug for Ark<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let inner = unsafe { self.ptr.as_ref() };
        inner.data.fmt(f)
    }
}

// T must be Sync because Ark is used precisely to share references across threads
unsafe impl<T: Send + Sync + ?Sized> Send for Ark<T> {}

// Ark<T> can be used inside another Ark
unsafe impl<T: Send + Sync + ?Sized> Sync for Ark<T> {}

// FIXME (copied from Arc without much thought)
unsafe impl<T: Send + Sync + ?Sized> Send for ArkInner<T> {}
unsafe impl<T: Send + Sync + ?Sized> Sync for ArkInner<T> {}

#[cfg(feature = "nightly")]
impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Ark<U>> for Ark<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(not(feature = "nightly"), ignore)]
    fn can_hold_a_trait_object() {
        #[cfg(feature = "nightly")]
        {
            let _: Ark<dyn Debug> = Ark::new(3);
        }
    }

    #[test]
    fn allows_option_of_ark_to_use_niche() {
        use std::mem::size_of;

        // Because we ensure the compiler that the pointer is never null, it can use that as a
        // niche to optimize the layout of things like Option<Ark<T>>.
        assert_eq!(size_of::<Ark<&str>>(), size_of::<Option<Ark<&str>>>());
    }

    #[test]
    fn is_covariant_over_the_type_parameter<'a>() {
        let s: Ark<&'static str> = Ark::new("hi");

        // If Ark<T> had not been made covariant over T using std::ptr::NonNull, the next statement
        // would not compile.
        let _: Ark<&'a str> = s;
    }

    #[test]
    #[cfg_attr(not(feature = "nightly"), ignore)]
    fn drop_does_not_access_contents() {
        #[allow(unused_variables)]
        #[cfg(feature = "nightly")]
        {
            let mut x = String::from("hi");

            let arc = Ark::new(&mut x);
            dbg!(x);

            // If Ark<T>::drop() did not use #[may_dangle] on T the drop checker would assume that
            // when `arc` is dropped here, it could access the contents.
        }
    }

    #[test]
    #[allow(unused_mut)]
    fn drop_checker_sees_that_contents_are_dropped() {
        #[derive(Debug)]
        struct BadDrop<T: Debug>(T);

        impl<T: Debug> Drop for BadDrop<T> {
            fn drop(&mut self) {
                dbg!(&self.0);
            }
        }

        let mut x = "hi";

        // The following line must NOT compile (this requires PhantomData<ArcInner<T>>).
        // let arc = Ark::new(BadDrop(&mut x));

        // If the previous line is uncommented, `x` is still mutably borrowed (because it may/will
        // be accessed by BadDrop<T>::drop()) and cannot be used here.
        dbg!(x);
    }
}
