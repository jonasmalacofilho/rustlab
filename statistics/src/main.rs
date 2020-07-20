use std::io::{self, BufRead};

mod stupid_stats {
    pub fn average<'a>(nums: impl Iterator<Item = &'a i32>) -> f64 {
        let mut sum = 0.;
        let mut cnt = 0;
        for &n in nums {
            sum += n as f64;
            cnt += 1;
        }
        sum / (cnt as f64)
    }

    pub fn median<'a>(nums: impl Iterator<Item = &'a i32>) -> &'a i32 {
        let mut tmp = nums.map(|n| n).collect::<Vec<_>>();
        tmp.sort();
        tmp[(tmp.len() - 1)/ 2]
    }

    pub fn mode<'a>(nums: impl Iterator<Item = &'a i32>) -> &'a i32 {
        let mut cnts = std::collections::HashMap::new();
        for n in nums {
            let cnt = cnts.entry(n).or_insert(0);
            *cnt += 1;
        }
        cnts.iter().max_by_key(|(_, &cnt)| cnt).unwrap().0
    }
}

fn main() {
    println!("Type in the desired numbers, one in each line.");
    println!("Once you done, terminate with Ctrl+D.");

    let nums = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!(
        "Results: mean={}, median={}, mode={}",
        stupid_stats::average(nums.iter()),
        stupid_stats::median(nums.iter()),
        stupid_stats::mode(nums.iter())
    );
}
