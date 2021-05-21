#[test]
fn cast_const_to_mut_ptr() {
    let foo = 42i32;
    let ptr = &foo as *const i32;

    // Miri says this is fine, even with -Zmiri-track-raw-pointers;
    // is it because we own foo anyway?
    let ptr = ptr as *mut i32;
    dbg!(unsafe { *ptr });
}
