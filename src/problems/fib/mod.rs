//! When finding the n-th term of a sequence defined by a recurrence relation,
//! we can simply use the recurrence relation to generate terms for progressively larger values of n
//!
//! This problem introduces us to the computational technique of dynamic programming,
//! which successively builds up solutions by using the answers to smaller cases.
//!
//! Given: Positive integers n≤40 and k≤5
//! Return: The total number of rabbit pairs that will be present after n months,
//!     if we begin with 1 pair and in each generation, every pair of reproduction-age rabbits produces a litter of k rabbit pairs (instead of only 1 pair).

use std::{
    fmt::{Debug, Display},
    iter::Map,
};

const MAX_N: usize = 40;
const MAX_K: usize = 5;

pub fn solve(input: &str) -> String {
    let [n, k] = input
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>()[..]
    else {
        panic!("Invalid input")
    };

    next_val(n, k).to_string()
}

fn next_val(n: usize, k: usize) -> usize {
    match n {
        // F(n) = F(n-1) + k * F(n-2)
        1 | 2 => 1,
        _ => {
            let mut prev2 = 1; // F(n-2)
            let mut prev1 = 1; // F(n-1)

            for _ in 2..n {
                let current = prev1 + k * prev2;
                prev2 = prev1;
                prev1 = current;
            }

            prev1
        }
    }
}
