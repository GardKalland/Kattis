use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let numbers: Vec<usize> = first_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = numbers[0];
    let m = numbers[1];

    let row_sums: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut col_sums: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();

    if can_form_matrix(n, m, &row_sums, &mut col_sums) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn can_form_matrix(n: usize, m: usize, row_sums: &[i32], col_sums: &mut Vec<i32>) -> bool {
    let mut heap = BinaryHeap::from(col_sums.to_vec());
    let mut row_sums_sorted = row_sums.to_vec();
    row_sums_sorted.sort_unstable_by(|a, b| b.cmp(a));

    for rsum in row_sums_sorted {
        let mut count = rsum;
        let mut tmp = vec![];

        while count > 0 {
            if let Some(max) = heap.pop() {
                if max > 0 {
                    count -= 1;
                    tmp.push(max - 1);
                }
            } else {
                return false;
            }
        }

        for item in tmp {
            heap.push(item);
        }
    }
    
    heap.into_iter().all(|x| x ==0)
}
