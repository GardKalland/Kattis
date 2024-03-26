use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nk = first_line.split_whitespace();
    let n: usize = nk.next().unwrap().parse().unwrap();
    let k: i64 = nk.next().unwrap().parse().unwrap();


    let mut events: Vec<(i64, i32)> = Vec::with_capacity(2 * n);

    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let mut times = line.split_whitespace();
            let start: i64 = times.next().unwrap().parse().unwrap();
            let end: i64 = times.next().unwrap().parse().unwrap();
            events.push((start, 1));
            events.push(((end + k).min(604800000), -1));
        }
    }

  
    events.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));

    let mut current_friends = 0;
    let mut max_friends = 0;
    for event in events {
        current_friends += event.1;
        max_friends = max_friends.max(current_friends);
    }

    println!("{}", max_friends);
}
