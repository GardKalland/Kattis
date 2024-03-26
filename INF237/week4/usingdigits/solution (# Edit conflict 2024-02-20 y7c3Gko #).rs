use std::io::{self, prelude, BufRead};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
    hops: Vec<usize>,  // Keep track of remaining hops
}

// Define ordering for the State struct to make the binary heap a min-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let x: usize = iter.next().unwrap().parse().unwrap();
    let y: usize = iter.next().unwrap().parse().unwrap();
    let code_key = lines.next().unwrap().unwrap();

    let grid: Vec<Vec<u32>> = (0..y)
        .map(|_| {
            lines.next().unwrap().unwrap().chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let smallest_sum = find_smallest_sum(x, y, &grid, &code_key);
    println!("{}", smallest_sum);
}

fn find_smallest_sum(x: usize, y: usize, grid: &Vec<Vec<u32>>, code_key: &str) -> u32 {
    let mut hops: Vec<usize> = code_key.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut dp = vec![vec![u32::MAX; x]; y];
    dp[0][0] = grid[0][0];
    let mut heap = BinaryHeap::new();

    heap.push(State { cost: grid[0][0], position: (0, 0), hops: hops.clone() });

    while let Some(State { cost, position: (i, j), hops }) = heap.pop() {
        if dp[i][j] < cost {
            continue;
        }


        if i == y - 1 && j == x - 1 {
            return cost;
        }
        println!("{}")
        
        // Expand to the right
        if j + 1 < x {
            let next_cost = cost + grid[i][j + 1];
            if next_cost < dp[i][j + 1] {
                heap.push(State { cost: next_cost, position: (i, j + 1), hops: hops.clone() });
                dp[i][j + 1] = next_cost;
            }
        }
        // Expand downwards
        if i + 1 < y {
            let next_cost = cost + grid[i + 1][j];
            if next_cost < dp[i + 1][j] {
                heap.push(State { cost: next_cost, position: (i + 1, j), hops: hops.clone() });
                dp[i + 1][j] = next_cost;
            }
        }
        // Use a hop if available
        if let Some(&hop) = hops.first() {
            // Hop to the right
            if j + hop < x {
                let next_cost = cost + grid[i][j + hop];
                if next_cost < dp[i][j + hop] {
                    let mut new_hops = hops.clone();
                    new_hops.remove(0);
                    heap.push(State { cost: next_cost, position: (i, j + hop), hops: new_hops });
                    dp[i][j + hop] = next_cost;
                }
            }
            // Hop downwards
            if i + hop < y {
                let next_cost = cost + grid[i + hop][j];
                if next_cost < dp[i + hop][j] {
                    let mut new_hops = hops.clone();
                    new_hops.remove(0);

                    heap.push(State { cost: next_cost, position: (i + hop, j), hops: new_hops });
                    dp[i + hop][j] = next_cost;
                }
            }
        }
    }

    dp[y - 1][x - 1]
}
