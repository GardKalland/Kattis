use std::io;
use std::io::prelude::*;

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    } else if n == 2 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as i64 + 1;
    for i in (3..limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn factorize(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let limit = (n as f64).sqrt() as i64 + 1;
    for i in 2..limit {
        if n % i == 0 && is_prime(i) {
            factors.push(i);
        }
    }
    factors
}

fn count_prime_factors(mut n: i64) -> i32 {
    let factors = factorize(n);
    let mut num_factors = 0;

    for &factor in &factors {
        while n > 1 && n % factor == 0 {
            num_factors += 1;
            n /= factor;
        }
    }

    if n > 1 {
        if is_prime(n) {
            return num_factors + 1;
        }

        for i in ((n / 2 + 1)..=2).rev() {
            while n > 1 {
                if n % i == 0 && is_prime(i) {
                    n /= i;
                    num_factors += 1;
                }
            }
        }
    }

    num_factors
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let input: i64 = input.trim().parse().unwrap();

    let num_factors = count_prime_factors(input);
    println!("{}", num_factors);
}
