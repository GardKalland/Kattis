use std::io;
use std::io::prelude::*;

fn length(platforms: &[(i32, i32, i32)]) -> i32 {
    let mut s = 0;
    for (i, &(y, x1, mut x2)) in platforms.iter().enumerate() {
        let mut l = -1;
        let mut r = -1;
        x2 -= 1;
        for &(y2, x3, mut x4) in platforms[..i].iter().rev() {
            x4 -= 1;
            if l == -1 && x3 <= x1 && x1 <= x4 {
                l = y - y2;
            }
            if r == -1 && x3 <= x2 && x2 <= x4 {
                r = y - y2;
            }
            if l != -1 && r != -1 {
                break;
            }
        }
        s += if l != -1 { l } else { y } + if r != -1 { r } else { y };
    }
    s
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut platforms: Vec<(i32, i32, i32)> = Vec::with_capacity(n);

    for line in lines {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        platforms.push((parts[0], parts[1], parts[2]));
    }

    platforms.sort();

    let result = length(&platforms);
    println!("{}", result);
}
