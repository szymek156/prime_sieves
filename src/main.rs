#![feature(test)]

mod sieves;
use std::time::{Instant};

fn main() {
    let limit = 1200000000;
    
    let now = Instant::now();
    let _primes_lin = sieves::linear_sieve(limit);
    let lin_time = now.elapsed();

    let now = Instant::now();
    let _primes_atkin = sieves::atkin_bernstein_sieve(limit);
    let atkin_time = now.elapsed();

    println!(
        "linear = {}, atkin = {}",
        lin_time.as_millis(),
        atkin_time.as_millis()
    );
}