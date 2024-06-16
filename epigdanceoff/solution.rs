use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    let mut buffer = String::new();
    stdin_lock.read_line(&mut buffer).unwrap();
    let mut dimensions = buffer.trim().split_whitespace();
    let rows: usize = dimensions.next().unwrap().parse().unwrap();
    let cols: usize = dimensions.next().unwrap().parse().unwrap();

    let mut grid = Vec::with_capacity(rows);
    for _ in 0..rows {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).unwrap();
        let line: Vec<char> = buffer.trim().chars().collect();
        grid.push(line);
    }

    let mut count = 0;
    for i in 0..cols {
        let mut c = 0;
        for j in 0..rows {
            if grid[j][i] == '_' {
                c += 1;
            }
        }
        if c == rows {
            count += 1;
        }
    }

    println!("{}", count + 1);
}
