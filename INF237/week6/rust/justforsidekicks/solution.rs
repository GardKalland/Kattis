use std::io::{self, BufRead};

struct FenwickTree {
    bit: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self { bit: vec![0; n + 1] }
    }

    fn update(&mut self, mut idx: usize, value: i64) {
        idx += 1; // Adjust index to 1-based for Fenwick tree operations
        while idx < self.bit.len() {
            self.bit[idx] += value;
            idx += idx & (!idx).wrapping_add(1);
        }
    }

    fn sum(&self, mut idx: usize) -> i64 {
        let mut result = 0;
        while idx > 0 {
            result += self.bit[idx];
            idx -= idx & (!idx).wrapping_add(1);
        }
        result
    }

    fn range_sum(&self, start: usize, end: usize) -> i64 {
        self.sum(end) - self.sum(start - 1)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let first_line = iterator.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = parts[0];
    let q = parts[1];

    let second_line = iterator.next().unwrap().unwrap();
    let mut values: Vec<i64> = second_line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let third_line = iterator.next().unwrap().unwrap();
    let mut gem_types: Vec<usize> = third_line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    // Create a Fenwick tree for each gem type
    let mut fenwicks: Vec<FenwickTree> = (0..6).map(|_| FenwickTree::new(n)).collect();

    // Initialize the Fenwick trees with gem occurrences
    for (i, &gem_type) in gem_types.iter().enumerate() {
        fenwicks[gem_type - 1].update(i, 1);
    }

    for _ in 0..q {
        let line = iterator.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let query_type: i32 = parts[0].parse().unwrap();

        match query_type {
            1 => {
                let pos: usize = parts[1].parse().unwrap();
                let new_type: usize = parts[2].parse().unwrap();
                let old_type = gem_types[pos - 1];

                // Update Fenwick trees to reflect the change in gem type
                fenwicks[old_type - 1].update(pos - 1, -1);
                fenwicks[new_type - 1].update(pos - 1, 1);


                gem_types[pos - 1] = new_type;
            },
            2 => {
                let gem_type: usize = parts[1].parse().unwrap();
                let new_value: i64 = parts[2].parse().unwrap();


                values[gem_type - 1] = new_value;
            },
            3 => {
                let start: usize = parts[1].parse().unwrap();
                let end: usize = parts[2].parse().unwrap();


                let mut sum = 0;
                for (j, fenwick) in fenwicks.iter().enumerate() {
                    sum += fenwick.range_sum(start, end) * values[j];
                }
                println!("{}", sum);
            },
            _ => unreachable!(),
        }
    }
}
