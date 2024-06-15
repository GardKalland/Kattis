
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let contains: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut tree = vec![vec![]; n + 1]; 
    for (i, &parent) in contains.iter().enumerate() {
        if parent != 0 {
            tree[parent].push(i + 1);
        }
    }

    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..q {
        let query_line = lines.next().unwrap().unwrap();
        let mut query_iter = query_line.split_whitespace();
        let m: usize = query_iter.next().unwrap().parse().unwrap();

        let mut query_boxes = vec![];
        for _ in 0..m {
            query_boxes.push(query_iter.next().unwrap().parse::<usize>().unwrap());
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        for &box_id in &query_boxes {
            if !visited.contains(&box_id) {
                queue.push_back(box_id);
                visited.insert(box_id);
            }
        }

        while let Some(current) = queue.pop_front() {
            for &child in &tree[current] {
                if !visited.contains(&child) {
                    queue.push_back(child);
                    visited.insert(child);
                }
            }
        }

        println!("{}", visited.len());
    }
}
