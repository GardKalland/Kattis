use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let b: i64 = parts[0].parse().unwrap();
            let d: i64 = parts[1].parse().unwrap();
            let starter: &str = parts[2];



            let winner = determine_winner(b, d, starter);
            println!("{}", winner);
        }
    }
}

fn determine_winner(b: i64, d: i64, starter: &str) -> String {


    if starter == "Harry"  {
        if b.ilog2() < d.ilog2() {
            format!("Harry can win")

        } else {
            format!("Harry cannot win")
        }
    } else if starter == "Vicky" {
        if  d.ilog2() < b.ilog2() {
            format!("Vicky can win")
        } else {
            format!("Vicky cannot win")
        }
    } else {

        format!("{} cannot win", starter)
    }
}
