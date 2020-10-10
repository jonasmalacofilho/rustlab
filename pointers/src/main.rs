#[derive(Clone, Debug)]
struct Foo {
    an_int: i32,
    another_int: i32,
    some_string: String,
    an_float: f64,
}

fn borrows(foo: &Foo) {
    dbg!(&foo);

    // see mut_borrows
}

fn mut_borrows(foo: &mut Foo) {
    dbg!(&foo);

    // &mut Foo is a mutable reference; references are one of rust's pointer types

    // explicit dereferencing
    //
    // > The * [...] operator is also a unary prefix operator. When applied to a pointer it denotes
    // > the pointed-to location.
    // —The dereference operator, The Rust Reference
    (*foo).an_int = 1;

    // automatic dereferencing
    //
    // > [...] if the type of the expression to the left of the dot is a pointer, it is
    // > automatically dereferenced as many times as necessary to make the field access possible.
    // —Field access expressions, The Rust Reference
    foo.an_int = 2;
}

fn takes(mut foo: Foo) {
    dbg!(&foo);

    // normal field access
    foo.an_int = 1;

    // Foo is not a pointer, so trying to dereference it makes no sense
    //
    //     (*foo).an_int = -1; // type `Foo` cannot be dereferenced
    //
    // > On non-pointer types *x is equivalent to *std::ops::Deref::deref(&x) in an immutable place
    // > expression context and *std::ops::DerefMut::deref_mut(&mut x) in a mutable place
    // > expression context.
    // —The dereference operator, The Rust Reference
    //
    // This means that besides the pointer types that are intrinsic to rust, deref (only) works on
    // types that implement Deref/DerefMut; these are smart pointers, and you can define your own.
}

fn heap_takes(mut foo: Box<Foo>) {
    dbg!(&foo);

    // Box does implement Deref/Deref mut, which is why both are possible:

    // - explicit dereferencing
    (*foo).an_int = 1;

    // - automatic dereferencing
    foo.an_int = 2;

    // and it's also possible to change the value stored in the box
    //
    // > If the expression is of type &mut T and *mut T, and [sic] is either a local variable, a
    // > (nested) field of a local variable or is a **mutable place expression,** then the
    // > resulting memory location **can be assigned to.**
    // —The dereference operator, The Rust Reference
    *foo = Foo {
        an_int: 3,
        another_int: 12,
        some_string: String::from("Bye"),
        an_float: 1.67,
    };

    // note: left-hand-side use of *foo is particularly useful with MutexGuards and in for loops
    // over &mut v where v is some Vec<_>
}

fn main() {
    let mut foo = Foo {
        an_int: 0,
        another_int: 42,
        some_string: String::from("Hello"),
        an_float: 3.14,
    };

    let foo_clone = foo.clone();
    let foo_box = Box::new(foo.clone());

    borrows(&foo);
    mut_borrows(&mut foo);
    takes(foo_clone);
    heap_takes(foo_box);
}
