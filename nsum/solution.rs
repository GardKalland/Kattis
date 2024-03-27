use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();


    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let sum: i32 = input.split_whitespace().map(|num| num.parse::<i32>().unwrap()).take(n).sum();

    println!("{}", sum);
}
