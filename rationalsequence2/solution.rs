use std::io::{self, BufRead};

fn move_recursive(l: i32, r: i32) -> i32 {
    if l == r {
        return 1;
    }

    if l > r {
        return 2 * move_recursive(l - r, r) + 1;
    }

    2 * move_recursive(l, r - l)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let p: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..p {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let i: i32 = parts[0].parse().unwrap();
            let fraction: Vec<&str> = parts[1].split('/').collect();
            let l: i32 = fraction[0].parse().unwrap();
            let r: i32 = fraction[1].parse().unwrap();

            println!("{} {}", i, move_recursive(l, r));
        }
    }
}
