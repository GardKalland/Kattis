use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let mut nums: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let height1 = nums[6];
    let height2 = nums[7];
    let mut boxes = nums[0..6].to_vec();
    boxes.sort();

    let mut found = false;
    let mut stack1 = vec![0; 3];
    let mut stack2 = vec![0; 3];

    while !found {
        if boxes[0] + boxes[1] + boxes[2] == height1 && boxes[3] + boxes[4] + boxes[5] == height2 {
            found = true;
            stack1.copy_from_slice(&boxes[0..3]);
            stack2.copy_from_slice(&boxes[3..6]);
        }
        if !boxes.next_permutation() {
            break;
        }
    }

    stack1.sort_by(|a, b| b.cmp(a));
    stack2.sort_by(|a, b| b.cmp(a));

    for i in 0..3 {
        print!("{} ", stack1[i]);
    }
    for i in 0..3 {
        print!("{} ", stack2[i]);
    }
    println!();
}

trait NextPermutation {
    fn next_permutation(&mut self) -> bool;
}

impl NextPermutation for Vec<i64> {
    fn next_permutation(&mut self) -> bool {
        let n = self.len();
        if n < 2 {
            return false;
        }

        let mut i = n - 2;
        while i != usize::MAX && self[i] >= self[i + 1] {
            i -= 1;
        }

        if i == usize::MAX {
            self.reverse();
            return false;
        }

        let mut j = n - 1;
        while self[j] <= self[i] {
            j -= 1;
        }

        self.swap(i, j);
        self[i + 1..].reverse();

        true
    }
}
