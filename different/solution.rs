use std::io::{self, BufRead};

fn main() {


    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i64> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();

        if numbers.len() == 2 {
            let difference = (numbers[0] - numbers[1]).abs();
            println!("{}", difference);
        }
    }
}
