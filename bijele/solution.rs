use std::io;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input.trim().split_whitespace().map(|part| part.parse().unwrap()).collect();

    let corr: Vec<i32> = vec![1, 1, 2, 2, 2, 8];

    let differences: Vec<i32> = corr.iter().zip(parts.iter()).map(|(&correct, &found)| correct - found).collect();

    println!("{}", differences.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));



}
