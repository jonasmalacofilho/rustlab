#[cfg(feature = "bool-based")]
type C = bool;

// Best chunk sizes appear to u8 and, in a close second place, usize.  But only tested on
// Intel/x86-64 (Skylake).
#[cfg(feature = "bit-based")]
type C = u8;

#[cfg(feature = "bit-based")]
const CBITS: usize = C::count_ones(C::MAX) as usize;

pub struct Sieve {
    upper_limit: usize,
    map: Vec<C>,
}

impl Sieve {
    #[cfg_attr(feature = "disass", inline(never))]
    pub fn build(upper_limit: usize) -> Self {
        let mut sieve = Sieve::new(upper_limit);
        sieve.clear_non_primes();
        sieve
    }

    #[cfg_attr(feature = "disass", inline(never))]
    pub fn is_prime(&self, number: usize) -> bool {
        if number < self.upper_limit && number % 2 == 1 {
            // SAFETY: just checked that number is odd and less than upper_limit
            unsafe { self.is_prime_unchecked(number) }
        } else {
            number == 2
        }
    }

    #[cfg_attr(feature = "disass", inline(never))]
    pub fn not_prime(&self, number: usize) -> bool {
        !self.is_prime(number)
    }

    #[cfg(feature = "for-loops")]
    fn clear_non_primes(&mut self) {
        // SAFETY: 1 is explicitly allowed
        unsafe { self.clear_prime_unchecked(1) };

        let enough = (self.upper_limit as f64).sqrt() as usize;

        for factor in (3..=enough).step_by(2) {
            // SAFETY: factor is odd (3, 5, 7, ...) and less than upper_limit
            // (enough < upper_limit)
            if unsafe { !self.is_prime_unchecked(factor) } {
                continue;
            }

            for mult in ((factor * factor)..self.upper_limit).step_by(factor * 2) {
                // SAFETY: mult is odd (prime * 3, 5, 7, ...) and less than upper_limit
                unsafe { self.clear_prime_unchecked(mult) };
            }
        }
    }

    #[cfg(feature = "for_each")]
    fn clear_non_primes(&mut self) {
        // SAFETY: 1 is explicitly allowed
        unsafe { self.clear_prime_unchecked(1) };

        let enough = (self.upper_limit as f64).sqrt() as usize;

        (3..=enough).step_by(2).for_each(|factor| {
            // SAFETY: factor is odd (3, 5, 7, ...) and less than upper_limit
            // (enough < upper_limit)
            if unsafe { !self.is_prime_unchecked(factor) } {
                return;
            }

            ((factor * factor)..self.upper_limit)
                .step_by(factor * 2)
                .for_each(|mult| {
                    // SAFETY: mult is odd (prime * 3, 5, 7, ...) and less than upper_limit
                    unsafe { self.clear_prime_unchecked(mult) };
                });
        });
    }

    // As of Rust 1.51, while loops are ~60% faster than using the for statement, and ~20% faster
    // than using for_each().  This may be partially caused by .step_by() and RangeInclusive.
    #[cfg(feature = "while-loops")]
    fn clear_non_primes(&mut self) {
        // SAFETY: 1 is explicitly allowed
        unsafe { self.clear_prime_unchecked(1) };

        let enough = (self.upper_limit as f64).sqrt() as usize;

        let mut factor = 3;
        while factor <= enough {
            // SAFETY: factor is odd (3, 5, 7, ...) and less than upper_limit
            // (enough < upper_limit)
            if unsafe { self.is_prime_unchecked(factor) } {
                let mut mult = factor * factor;

                while mult < self.upper_limit {
                    // SAFETY: mult is odd (prime * 3, 5, 7, ...) and less than upper_limit
                    unsafe { self.clear_prime_unchecked(mult) };
                    mult += factor * 2;
                }
            }
            factor += 2;
        }
    }
}

#[cfg(feature = "bool-based")]
impl Sieve {
    #[cfg_attr(feature = "disass", inline(never))]
    pub fn count_primes(&self) -> usize {
        // Assumes that extra bits at the end are zeroed/cleared.
        self.map.iter().filter(|&x| *x).count() as usize + 1
    }

    #[cfg(feature = "bool-based")]
    #[cfg_attr(feature = "disass", inline(never))]
    fn new(upper_limit: usize) -> Self {
        Sieve {
            upper_limit,
            map: vec![true; upper_limit / 2],
        }
    }

    /// # Safety
    ///
    /// The `number` argument must be one or odd and less than `self.upper_limit`.
    #[cfg(feature = "bool-based")]
    #[cfg_attr(feature = "disass", inline(never))]
    unsafe fn clear_prime_unchecked(&mut self, number: usize) {
        debug_assert!(number % 2 != 0);
        debug_assert!(number < self.upper_limit || number == 1);
        debug_assert!(number / 2 < self.map.len());

        *self.map.get_unchecked_mut(number / 2) = false;
    }

    /// # Safety
    ///
    /// The `number` argument must be odd and less `self.upper_limit`.
    #[cfg(feature = "bool-based")]
    #[cfg_attr(feature = "disass", inline(never))]
    unsafe fn is_prime_unchecked(&self, number: usize) -> bool {
        debug_assert!(number % 2 != 0);
        debug_assert!(number < self.upper_limit);
        debug_assert!(number / 2 < self.map.len());

        *self.map.get_unchecked(number / 2)
    }
}

#[cfg(feature = "bit-based")]
impl Sieve {
    #[cfg_attr(feature = "disass", inline(never))]
    pub fn count_primes(&self) -> usize {
        // Assumes that extra bits at the end are zeroed/cleared.
        self.map.iter().map(|x| x.count_ones()).sum::<u32>() as usize + 1
    }

    #[cfg(feature = "bit-based")]
    #[cfg_attr(feature = "disass", inline(never))]
    fn new(upper_limit: usize) -> Self {
        let mut map = vec![C::MAX as C; upper_limit / 2 / CBITS + 1];

        // Zero extra bits at the end, so count_primes can use count_ones.
        let keep = upper_limit / 2 % CBITS;
        let keep_mask: C = (1 << keep) - 1;

        let last = map.len() - 1;

        // SAFETY: just computed last from map.len()
        unsafe { *map.get_unchecked_mut(last) &= keep_mask };

        Sieve { upper_limit, map }
    }

    /// # Safety
    ///
    /// The `number` argument must be one or odd and less than `self.upper_limit`.
    #[cfg(feature = "bit-based")]
    #[cfg_attr(feature = "disass", inline(never))]
    unsafe fn clear_prime_unchecked(&mut self, number: usize) {
        debug_assert!(number % 2 != 0);
        debug_assert!(number < self.upper_limit || number == 1);

        let number = number / 2;
        let word = number / CBITS;
        let bit = number % CBITS;

        debug_assert!(word < self.map.len());
        *self.map.get_unchecked_mut(word) &= !(1 << bit)
    }

    ///
    /// # Safety
    ///
    /// The `number` argument must be odd and less `self.upper_limit`.
    #[cfg(feature = "bit-based")]
    #[cfg_attr(feature = "disass", inline(never))]
    unsafe fn is_prime_unchecked(&self, number: usize) -> bool {
        debug_assert!(number % 2 != 0);
        debug_assert!(number < self.upper_limit);

        let number = number / 2;
        let word = number / CBITS;
        let bit = number % CBITS;

        debug_assert!(word < self.map.len());
        self.map.get_unchecked(word) & (1 << bit) != 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_all_primes_bellow_25() {
        let sieve = Sieve::build(25);

        assert!(sieve.is_prime(2));
        assert!(sieve.is_prime(3));
        assert!(sieve.is_prime(5));
        assert!(sieve.is_prime(7));
        assert!(sieve.is_prime(11));
        assert!(sieve.is_prime(13));
        assert!(sieve.is_prime(17));
        assert!(sieve.is_prime(19));
        assert!(sieve.is_prime(23));

        assert!(sieve.not_prime(0));
        assert!(sieve.not_prime(1));
        assert!(sieve.not_prime(4));
        assert!(sieve.not_prime(6));
        assert!(sieve.not_prime(8));
        assert!(sieve.not_prime(9));
        assert!(sieve.not_prime(10));
        assert!(sieve.not_prime(12));
        assert!(sieve.not_prime(14));
        assert!(sieve.not_prime(15));
        assert!(sieve.not_prime(16));
        assert!(sieve.not_prime(18));
        assert!(sieve.not_prime(20));
        assert!(sieve.not_prime(21));
        assert!(sieve.not_prime(22));
        assert!(sieve.not_prime(24));

        assert_eq!(sieve.count_primes(), 9);
    }
}
