use std::io::{self, Read};
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let k: i32 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut events = lines
        .flat_map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let (a, b) = (nums[0], nums[1]);
            vec![(a, 1), (b + k + 1, -1)]
        })
        .collect::<Vec<_>>();

    events.sort_unstable();
    let max_overlap = events
        .iter()
        .scan(0, |state, &(_, val)| {
            *state += val;
            Some(*state)
        })
        .max()
        .unwrap();

    println!("{}", max_overlap);
}


