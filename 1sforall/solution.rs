use std::io::{self, Read};
use std::cmp;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut dp = vec![10_usize.pow(9); n + 1];
    dp[1] = 1;

    for k in 2..=n {
        for l in 1..=k / 2 {
            dp[k] = cmp::min(dp[k], dp[k - l] + dp[l]);
        }
        for l in 2..=((k as f64).sqrt() as usize + 1) {
            if k % l == 0 {
                dp[k] = cmp::min(dp[k], dp[k / l] + dp[l]);
            }
        }
        let s = k.to_string();
        for i in 0..s.len() - 1 {
            if s.as_bytes()[i + 1] != b'0' {
                let (left, right) = s.split_at(i + 1);
                let left_val: usize = left.parse().unwrap();
                let right_val: usize = right.parse().unwrap();
                dp[k] = cmp::min(dp[k], dp[left_val] + dp[right_val]);
            }
        }
    }

    println!("{}", dp[n]);
}
