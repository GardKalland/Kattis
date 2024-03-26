use std::io;

fn main() {

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let input_parts: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let X = input_parts[0];
    let Y = input_parts[1];
    let N = input_parts[2];


    for i in 1..=N {

        if i % X == 0 && i % Y == 0 {
            println!("FizzBuzz");
        }
        else if i % X == 0 {
            println!("Fizz");
        }
        else if i % Y == 0 {
            println!("Buzz");
        }
        else {
            println!("{}",i);
        }
    }
}