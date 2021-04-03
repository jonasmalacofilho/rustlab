//! Measure how many sieves can be executed in a given period.

use prime_sieve::Sieve;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let period = 5;
    let upper_limit: usize = 1_000_000;

    let prime_counts = [
        (10, 4),
        (100, 25),
        (1_000, 168),
        (10_000, 1_229),
        (100_000, 9_592),
        (1_000_000, 78_498),
        (10_000_000, 664_579),
        (100_000_000, 5_761_455),
    ].iter().map(|x| *x).collect::<HashMap<usize, usize>>();

    let start = Instant::now();
    let mut duration = 0;
    let mut passes = 0;

    while duration < period {
        let sieve = Sieve::build(upper_limit);
        assert_eq!(sieve.count_primes(), prime_counts[&upper_limit]);
        passes += 1;
        duration = Instant::now().duration_since(start).as_secs();
    }

    println!(
        "{} passes in {} seconds; each pass took {} seconds in average",
        passes,
        duration,
        duration as f32 / passes as f32
    );
}
