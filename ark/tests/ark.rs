use ark::Ark;
use std::thread;

#[test]
fn smoke_test() {
    let some_data = 42;

    let a = Ark::new(some_data);
    let b = Ark::clone(&a);

    let handle = thread::spawn(move || {
        assert_eq!(*b, 42);
    });

    assert_eq!(*a, 42);

    handle.join().unwrap();
}

#[test]
fn smoke_test_where_ark_is_sync() {
    let some_data = 42;

    let a = Ark::new(some_data);
    let b = Ark::new(Ark::clone(&a));

    let handle = thread::spawn(move || {
        assert_eq!(**b, 42);
    });

    assert_eq!(*a, 42);

    handle.join().unwrap();
}
