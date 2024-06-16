use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let num: usize = input.trim().parse().unwrap();

    let width = num * 2 + 3;
    let mut grid = vec![vec![' '; width]; width];

    grid[0][num + 1] = 'x';
    grid[num + 1][0] = 'x';
    grid[num + 1][width - 1] = 'x';
    grid[width - 1][num + 1] = 'x';

    let mut col = num;
    for row in 1..=num {
        grid[row][col] = '/';
        if col > 0 {
            col -= 1;
        }
    }

    col = num + 2;
    for row in 1..=num {
        grid[row][col] = '\\';
        col += 1;
    }

    col = 1;
    for row in (num + 2)..(width - 1) {
        grid[row][col] = '\\';
        col += 1;
    }

    col = width - 2;
    for row in (num + 2)..(width - 1) {
        grid[row][col] = '/';
        if col > 0 {
            col -= 1;
        }
    }

    let mut top = String::new();
    for _ in 0..=num {
        top.push(' ');
    }
    top.push('x');

    let mut sb = String::new();
    sb.push_str(&top);
    sb.push('\n');

    for i in 1..(width - 1) {
        let mut count = 0;
        for j in 0..width {
            let ch = grid[i][j];
            if ch == ' ' {
                sb.push(' ');
            } else {
                sb.push(ch);
                count += 1;
            }
            if count == 2 {
                break;
            }
        }
        sb.push('\n');
    }

    sb.push_str(&top);

    // Output
    println!("{}", sb);
}
