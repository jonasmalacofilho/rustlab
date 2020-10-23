use disassembly::*;

fn main() {
    noop_call();
    hello_world();
    additions();
    caught_panic();
    call_with_moving_struct();
}

fn additions() {
    adds_two_ints(1, 2);

    adds_many_ints(1, 2, 3, 4, 5, 6, 7, 8);
}

fn caught_panic() {
    use std::panic;

    let res = panic::catch_unwind(|| panics_on_42(42));
    assert!(res.is_err());

    eprintln!("panic caught");
}

fn call_with_moving_struct() {
    let foo = Foo { a_int: 42, a_float: std::f64::consts::PI };

    takes_foo(foo);
}
