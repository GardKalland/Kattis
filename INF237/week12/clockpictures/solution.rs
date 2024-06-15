use std::io::{self, BufRead};

fn compute_gaps(angles: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut gaps = Vec::with_capacity(n);
    for i in 1..n {
        gaps.push(angles[i] - angles[i - 1]);
    }
    let wrap_around_gap = 360_000 - angles[n - 1] + angles[0];
    gaps.push(wrap_around_gap);
    gaps
}

fn build_prefix_array(pattern: &Vec<i32>) -> Vec<usize> {
    let m = pattern.len();
    let mut prefix = vec![0; m];
    let mut j = 0;
    for i in 1..m {
        while j > 0 && pattern[i] != pattern[j] {
            j = prefix[j - 1];
        }
        if pattern[i] == pattern[j] {
            j += 1;
        }
        prefix[i] = j;
    }
    prefix
}

fn kmp(text: &Vec<i32>, pattern: &Vec<i32>, n: usize) -> bool {
    let m = pattern.len();
    let prefix = build_prefix_array(pattern);
    let mut j = 0;
    for i in 0..2 * n {
        while j > 0 && text[i % n] != pattern[j] {
            j = prefix[j - 1];
        }
        if text[i % n] == pattern[j] {
            j += 1;
        }
        if j == m {
            return true;
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let num_hands: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut image1_angles: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
        
    let mut image2_angles: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
        
    if num_hands == 1 {
        println!("possible");
        return;
    }
    
    image1_angles.sort();
    image2_angles.sort();
    
    let gaps1 = compute_gaps(&image1_angles, num_hands);
    let gaps2 = compute_gaps(&image2_angles, num_hands);
    
    if kmp(&gaps1, &gaps2, num_hands) {
        println!("possible");
    } else {
        println!("impossible");
    }
}
