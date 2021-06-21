// #![feature(test)]

mod sieves;
use std::{time::Instant};

fn main() {
    let limit = 120000000;

    println!("Calculating primes up to {}...", limit);

    println!("Linear...");
    let now = Instant::now();
    let primes_lin = sieves::linear_sieve(limit);
    let lin_time = now.elapsed();

    println!("Atkin...");
    let now = Instant::now();
    let primes_atkin = sieves::atkin_bernstein_sieve(limit);
    let atkin_time = now.elapsed();

    println!(
        "linear = {}, atkin = {}",
        lin_time.as_millis(),
        atkin_time.as_millis()
    );

    println!("They match? {}", primes_atkin == primes_lin);
}
