use std::io::{self, BufRead};

fn main() {
    let mut input = String::new();

    io::stdin().lock().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();

    println!("{}", a + b);

}
