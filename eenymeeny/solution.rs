use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let interval = input.split_whitespace().count() - 1;

    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut names = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        names.push(input.trim().to_string());
    }

    let mut team_a = Vec::new();
    let mut team_b = Vec::new();

    let mut prev_index = 0;
    let mut current_n = n;
    for i in 0..n {
        let index = (interval + prev_index) % current_n;
        if i % 2 == 0 {
            team_a.push(names.remove(index));
        } else {
            team_b.push(names.remove(index));
        }
        current_n -= 1;
        prev_index = index;
    }

    println!("{}", team_a.len());
    for name in team_a {
        println!("{}", name);
    }

    println!("{}", team_b.len());
    for name in team_b {
        println!("{}", name);
    }
}
