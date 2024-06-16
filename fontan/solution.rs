
use std::collections::VecDeque;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let mut dimensions = input.trim().split_whitespace();
    let r: usize = dimensions.next().unwrap().parse().unwrap();
    let c: usize = dimensions.next().unwrap().parse().unwrap();

    let mut grid = vec![vec![' '; c]; r];
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; c]; r];

    for i in 0..r {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let line = input.trim();
        for (j, ch) in line.chars().enumerate() {
            grid[i][j] = ch;
            if ch == 'V' {
                queue.push_back((i, j));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        if !in_range(x, y, r, c) || visited[x][y] {
            continue;
        }
        visited[x][y] = true;
        if grid[x][y] != '#' {
            grid[x][y] = 'V';
        }
        if x + 1 < r && grid[x + 1][y] != '#' {
            queue.push_back((x + 1, y));
        } else if x + 1 < r && grid[x + 1][y] == '#' {
            if y > 0 && grid[x][y - 1] == '.' {
                queue.push_back((x, y - 1));
            }
            if y + 1 < c && grid[x][y + 1] == '.' {
                queue.push_back((x, y + 1));
            }
        }
    }

    for i in 0..r {
        for j in 0..c {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}

fn in_range(x: usize, y: usize, r: usize, c: usize) -> bool {
    x < r && y < c
}
