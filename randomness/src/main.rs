use structopt::StructOpt;

use rand::prelude::*;
use rand::rngs::OsRng;

#[derive(Debug, StructOpt)]
/// Generate some randomness
enum Cmd {
    /// Generate random decimal digits
    Digits { count: usize },

    /// Generate SILLY random decimal digits: no repetition or immediate neighbors
    SillyDigits { count: usize },
}

fn main() {
    let cmd = Cmd::from_args();

    match cmd {
        Cmd::Digits { count } => println!("{} digits: {:?}", count, gen_digits(count)),
        Cmd::SillyDigits { count } => {
            println!("{} SILLY digits: {:?}", count, gen_silly_digits(count))
        }
    };
}

fn gen_digits(num: usize) -> Vec<u8> {
    (0..num).map(|_| OsRng.gen_range(0..10)).collect()
}

fn gen_silly_digits(num: usize) -> Vec<u8> {
    assert!(
        num <= 9,
        "cannot reliably generate more than 9 SILLY digits"
    );

    let mut digits: Vec<u8> = Vec::with_capacity(num);

    for i in 0..num {
        loop {
            let digit = OsRng.gen_range(0..10);
            if i == 0 || (!digits.contains(&digit) && (digits[i - 1] as i8 - digit as i8).abs() > 1)
            {
                digits.push(digit);
                break;
            }
        }
    }

    digits
}
