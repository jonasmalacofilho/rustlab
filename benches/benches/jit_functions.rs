//! Benchmark Haxe's "JIT functions" approach to a JIT interpreter.
//!
//! This approach is used by Haxe in the `eval` macro interpreter, and was proved to be
//! substantially faster in that context.[1][2]
//!
//! The idea is that instead of a single pass interpreter loop, two passes are performed:
//!
//! - first, the AST is traversed to create a tree of closures functions
//! - second, the tree of functions is executed
//!
//! Now let's see if it is beneficial (or even necessary) in a system's programming language like
//! Rust.
//!
//! TODO try a more complex AST with some variables and loops, because for simple arithmetic the
//! JIT functions are actually a lot slower (probably due to the increased number of allocations).
//!
//! [1] https://www.youtube.com/watch?v=pOClR-jJoBM
//! [2] https://haxe.org/blog/eval/

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Clone)]
enum Binop {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Clone)]
enum Expr {
    Literal(i64),
    Binop(Binop, Box<Expr>, Box<Expr>),
}

fn interp(e: &Expr) -> i64 {
    use crate::Binop::*;
    use Expr::*;

    match e {
        Literal(val) => *val,
        Binop(op, a, b) => {
            let a = interp(a);
            let b = interp(b);
            match op {
                Add => a + b,
                Sub => a - b,
                Mult => a * b,
                Div => a / b,
            }
        }
    }
}

fn jit_interp(e: &Expr) -> i64 {
    fn jit(e: &Expr) -> Box<dyn Fn() -> i64> {
        use crate::Binop::*;
        use Expr::*;

        match e {
            Literal(val) => {
                let val = *val;
                Box::new(move || val)
            }
            Binop(op, a, b) => {
                let a = jit(a);
                let b = jit(b);
                match op {
                    Add => Box::new(move || a() + b()),
                    Sub => Box::new(move || a() - b()),
                    Mult => Box::new(move || a() * b()),
                    Div => Box::new(move || a() / b()),
                }
            }
        }
    }

    jit(e)()
}

fn criterion_benchmark(c: &mut Criterion) {
    use crate::Binop::*;
    use Expr::*;

    // ((11 * 41) + ((-3) / 17)) - 19
    let data = Binop(
        Sub,
        Box::new(Binop(
            Add,
            Box::new(Binop(Mult, Box::new(Literal(11)), Box::new(Literal(41)))),
            Box::new(Binop(Div, Box::new(Literal(-3)), Box::new(Literal(17)))),
        )),
        Box::new(Literal(19)),
    );

    fn native(a: i64, b: i64, c: i64, d: i64, e: i64) -> i64 {
        ((a * b) + (c / d)) - e
    }

    assert_eq!(native(11, 41, -3, 17, 19), 432);
    assert_eq!(interp(&data), 432);
    assert_eq!(jit_interp(&data), 432);

    c.bench_function("native code (baseline)", |b| {
        b.iter(|| {
            native(
                black_box(11),
                black_box(41),
                black_box(-3),
                black_box(17),
                black_box(19),
            )
        })
    });

    c.bench_function("interpreter loop", |b| b.iter(|| interp(black_box(&data))));

    c.bench_function("jit interpreter", |b| {
        b.iter(|| jit_interp(black_box(&data)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
