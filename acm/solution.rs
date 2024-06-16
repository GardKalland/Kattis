use std::collections::HashMap;
use std::io::{self, BufRead};

struct Question {
    ac: bool,
    time: i32,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut m: HashMap<char, Question> = HashMap::new();

    while let Some(Ok(line)) = lines.next() {
        let mut iter = line.split_whitespace();
        let t: i32 = iter.next().unwrap().parse().unwrap();
        if t == -1 {
            break;
        }
        let q: char = iter.next().unwrap().chars().next().unwrap();
        let s: &str = iter.next().unwrap();

        let entry = m.entry(q).or_insert(Question { ac: false, time: 0 });
        if s == "right" {
            entry.ac = true;
            entry.time += t;
        } else {
            entry.time += 20;
        }
    }

    let mut rights = 0;
    let mut total = 0;
    for question in m.values() {
        if question.ac {
            rights += 1;
            total += question.time;
        }
    }

    println!("{} {}", rights, total);
}
