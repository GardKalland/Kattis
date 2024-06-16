use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut v: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    v.sort();

    let s = lines.next().unwrap().unwrap();

    match s.as_str() {
        "ABC" => println!("{} {} {}", v[0], v[1], v[2]),
        "ACB" => println!("{} {} {}", v[0], v[2], v[1]),
        "BAC" => println!("{} {} {}", v[1], v[0], v[2]),
        "BCA" => println!("{} {} {}", v[1], v[2], v[0]),
        "CAB" => println!("{} {} {}", v[2], v[0], v[1]),
        "CBA" => println!("{} {} {}", v[2], v[1], v[0]),
        _ => {}
    }
}
