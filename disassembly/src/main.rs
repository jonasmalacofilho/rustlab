fn main() {
    noop_call();
    hello_world();
    additions();
    caught_panic();
    call_with_moving_struct();
}

fn noop_call() {}

fn hello_world() {
    use std::io::{self, Write};
    const HELLO: &str = "Hello, World!\n";

    io::stdout().write_all(HELLO.as_bytes()).unwrap();
}

fn additions() {
    fn adds_two_ints(a: i32, b: i32) -> i32 {
        a + b
    }

    adds_two_ints(1, 2);

    fn adds_many_ints(a: i32, b: i32, c :i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        (a + b) + (c + d) + (e + f) + (g + h)
    }

    adds_many_ints(1, 2, 3, 4, 5, 6, 7, 8);
}

fn caught_panic() {
    // hint: set breakpoint on `rust_panic`

    use std::panic;

    fn panics_on_42(answer: i32) {
        if answer == 42 {
            panic!("goodbye, World!");
        }
    }

    let res = panic::catch_unwind(|| panics_on_42(42));
    assert!(res.is_err());

    eprintln!("panic caught");
}

fn call_with_moving_struct() {
    struct Foo {
        a_int: i32,
        a_float: f64,
    }

    let foo = Foo { a_int: 42, a_float: std::f64::consts::PI };

    fn takes_foo(foo: Foo) -> f64 {
        // hint: print a_float on stack with (adjust rsp offset)
        // `print *(($rsp + 0x18) as *mut f64)`
        (foo.a_int * 3) as f64 + foo.a_float
    }

    takes_foo(foo);
}
