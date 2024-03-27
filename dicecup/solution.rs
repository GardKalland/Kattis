use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let dice: Vec<usize> = input.split_whitespace()
                                .map(|s| s.parse().unwrap())
                                .collect();

    let (n, m) = (dice[0], dice[1]);

    let mut sums = HashMap::new();
    let mut max_count = 0;

    for i in 1..=n {
        for j in 1..=m {
            let sum = i + j;
            let count = sums.entry(sum).or_insert(0);
            *count += 1;

            if *count > max_count {
                max_count = *count;
            }
        }
    }

    let mut results = Vec::new();
    for (sum, &count) in &sums {
        if count == max_count {
            results.push(sum);
        }
    }

    results.sort_unstable();
    for result in results {
        println!("{}", result);
    }
}
