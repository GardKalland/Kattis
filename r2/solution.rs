use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<isize> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    println!("{}", (parts[1] * 2) - parts[0]);
}
