use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

/// Minimum value used in the game
const MIN: u8 = 1;
/// Maximum value used in the game
const MAX: u8 = 100;

/// Logs and returns the value of a given expression for simple debugging.
///
/// Follows [`std::dbg!`] but the expression is logged—with [`debug!`] from the [`log`] crate—
/// instead of directly printed to `stderr`.
///
/// [`debug!`]: https://docs.rs/log/*/log/macro.debug.html
/// [`log`]: https://crates.io/crates/log/
/// [`std::dbg!`]: https://doc.rust-lang.org/std/macro.dbg.html
#[macro_export]
macro_rules! debug_expr {
    // Adapted from std::dgb! to use log::debug! instead of std::eprintln!
    () => {
        log::debug!("[{}:{}]", std::file!(), std::line!());
    };
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                log::debug!("[{}:{}] {} = {:#?}",
                    std::file!(), std::line!(), std::stringify!($val), &tmp);
                tmp
            }
        }
    };
    // Trailing comma with single argument is ignored
    ($val:expr,) => { debug_expr!($val) };
    ($($val:expr),+ $(,)?) => {
        ($(debug_expr!($val)),+,)
    };
}

/// A guessing game.
///
/// How it works:
///
///  - generate a random integer between MIN and MAX;
///  - prompt the player to enter a guess
///  - after the guess is entered, indicated whether it's too low or too high
///  - if the guess is correct, congratulate and exit
fn main() {
    env_logger::init();

    println!("Guess the number!");

    let mut rng = thread_rng();
    let num = rng.gen_range(MIN, MAX + 1);
    debug_expr!(num);

    loop {
        println!("Your guess?");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess = match guess.trim().parse::<u8>() {
            Ok(val) if val >= MIN && val <= MAX => val,
            _ => {
                println!("Guess must be an integer between {} and {}", MIN, MAX);
                continue;
            }
        };

        print!("You guessed {}", guess);
        match guess.cmp(&num) {
            Ordering::Less => println!(" but that was too LOW."),
            Ordering::Greater => println!(" but that was too HIGH."),
            Ordering::Equal => {
                println!("... and that was just right!  Congratulations!");
                break;
            }
        }
    }
}
