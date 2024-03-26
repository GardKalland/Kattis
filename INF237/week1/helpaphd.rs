use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let N: i32 = input
        .trim()
        .parse()
        .unwrap();

    
    for line in stdin.lock().lines() {
        let task = line.unwrap();
        
        if task == "P=NP" { println!("skipped"); }
        else {
            let value: Vec<&str> = task.split('+').collect();
            let a: i32 = value[0].parse().unwrap();
            let b: i32 = value[1].parse().unwrap();
            println!("{}", a + b);
        }
    }
}
