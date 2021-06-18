
extern crate test;
use std::time::Instant;

pub fn linear_sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];

    let mut p = 2;

    while p * p <= n {
        let mut q = p;

        while p * q <= n {
            let mut x = p * q;

            while x <= n {
                is_prime[x] = false;
                x *= p
            }

            loop {
                q += 1;
                if is_prime[q] {
                    break;
                }
            }
        }
        loop {
            p += 1;
            if is_prime[p] {
                break;
            }
        }
    }

    let mut sieve = Vec::<usize>::new();
    for (idx, el) in is_prime.iter().enumerate() {
        if *el && idx > 1 {
            sieve.push(idx);
        }
    }

    sieve
}

/// Sieve algorithm by Atkin and Berndstein
/// Based on https://github.com/fylux/SieveOfAtkin
pub fn atkin_bernstein_sieve(limit: usize) -> Vec<usize> {
    let mut res = vec![];
    if limit > 2 {
        res.push(2);
    }
    if limit > 3 {
        res.push(3);
    }

    let mut sieve = vec![false; limit];

    for x in 1..limit {
        let x2 = x * x;
        if x2 >= limit {
            break;
        }

        for y in 1..limit {
            let y2 = y * y;
            if y2 >= limit {
                break;
            }

            let n = 4 * x2 + y2;

            if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                sieve[n] ^= true;
            }

            let n = 3 * x2 + y2;
            if n <= limit && n % 12 == 7 {
                sieve[n] ^= true;
            }

            if x > y {
                let n = 3 * x2 - y2;
                if n <= limit && n % 12 == 11 {
                    sieve[n] ^= true;
                }
            }
        }
    }

    for r in 5..limit {
        let r2 = r * r;
        if r2 >= limit {
            break;
        }

        if sieve[r] {
            for i in (r2..limit).step_by(r2) {
                sieve[i] = false;
            }
        }
    }

    for a in 5..limit {
        if sieve[a] {
            res.push(a);
        }
    }

    res
}

// TODO: erastotenes
// TODO: rabin miller?
pub fn nth(n: usize) -> usize {
    let limit = 12000000;
    let primes_lin = linear_sieve(limit);

    let primes_atkin = atkin_bernstein_sieve(limit);
    println!("equal? {}", primes_lin == primes_atkin);
    primes_lin[n]
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn sieves_yields_the_same_result() {
        let limit = 12000000;
        let now = Instant::now();
        let primes_lin = linear_sieve(limit);
        let lin_time = now.elapsed();

        let now = Instant::now();
        let primes_atkin = atkin_bernstein_sieve(limit);
        let atkin_time = now.elapsed();

        println!(
            "linear = {}, atkin = {}",
            lin_time.as_millis(),
            atkin_time.as_millis()
        );

        assert_eq!(primes_atkin, primes_lin);
    }

    #[bench]
    fn bench_atkin(b: &mut Bencher) {
        b.iter(|| atkin_bernstein_sieve(120000));
    }

    #[bench]
    fn bench_linear(b: &mut Bencher) {
        b.iter(|| linear_sieve(120000));
    }
}
