use std::io::{self, BufRead};

fn test_case(kids: usize, candies: Vec<i64>) {
    let mut sum: i64 = 0;
    for &candy in &candies {
        sum += candy;
        if sum > kids as i64 {
            sum %= kids as i64;
        }
    }
    if sum == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().filter_map(Result::ok).collect::<Vec<_>>();
    let mut iter = input.into_iter().filter(|line| !line.trim().is_empty());

    let t: usize = iter.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let kids: usize = iter.next().unwrap().trim().parse().unwrap();
        let candies = iter.by_ref().take(kids).map(|line| line.trim().parse().unwrap()).collect();
        test_case(kids, candies);
    }
}
