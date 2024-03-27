use std::io;

fn main() {
    let mut input = String::new();
    let mut n = String::new();

    io::stdin().read_line(&mut input).unwrap();
    io::stdin().read_line(&mut n).unwrap();

    let n: usize = n.trim().parse().unwrap();
    println!("{}", input.trim().repeat(n));
}
