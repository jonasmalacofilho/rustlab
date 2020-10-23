pub fn noop_call() {}

pub fn hello_world() {
    use std::io::{self, Write};
    const HELLO: &str = "Hello, World!\n";

    io::stdout().write_all(HELLO.as_bytes()).unwrap();
}

pub fn adds_two_ints(a: i32, b: i32) -> i32 {
    a + b
}

pub fn adds_many_ints(a: i32, b: i32, c :i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
    (a + b) + (c + d) + (e + f) + (g + h)
}

pub fn panics_on_42(answer: i32) {
    // hint: set breakpoint on `rust_panic`
    if answer == 42 {
        panic!("goodbye, World!");
    }
}

pub struct Foo {
    pub a_int: i32,
    pub a_float: f64,
}

pub fn takes_foo(foo: Foo) -> f64 {
    // hint: print a_float on stack with (adjust rsp offset)
    // `print *(($rsp + 0x18) as *mut f64)`
    (foo.a_int * 3) as f64 + foo.a_float
}
