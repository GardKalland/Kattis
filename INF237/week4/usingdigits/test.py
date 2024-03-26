use std::io::{self, BufRead};

const MAXN: usize = 101;
const INF: i32 = 1805;

fn solve(r: usize, c: usize, p: usize, dp: &mut Vec<Vec<Vec<i32>>>, grid: &Vec<Vec<i32>>, code: &Vec<usize>, x: usize, y: usize) -> i32 {
    if r >= y || c >= x {
        return INF;
    }
    if r == y - 1 && c == x - 1 {
        return grid[r][c];
    }
    if dp[r][c][p] < INF {
        return dp[r][c][p];
    }

    let mut opt = INF;
    if p < code.len() {
        let key = code[p];
        if r + key + 1 < y {
            opt = opt.min(grid[r][c] + solve(r + key + 1, c, p + 1, dp, grid, code, x, y));
        }
        if c + key + 1 < x {
            opt = opt.min(grid[r][c] + solve(r, c + key + 1, p + 1, dp, grid, code, x, y));
        }
    }

    if r + 1 < y {
        opt = opt.min(grid[r][c] + solve(r + 1, c, p, dp, grid, code, x, y));
    }
    if c + 1 < x {
        opt = opt.min(grid[r][c] + solve(r, c + 1, p, dp, grid, code, x, y));
    }

    dp[r][c][p] = opt;
    opt
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (x, y) = (parts[0], parts[1]);
    let code_str = lines.next().unwrap().unwrap();
    let code: Vec<usize> = code_str.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    let mut grid = vec![vec![0; x]; y];
    for i in 0..y {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            grid[y - i - 1][j] = c.to_digit(10).unwrap() as i32;
        }
    }

    let mut dp = vec![vec![vec![INF; code.len() + 1]; x]; y];

    let ans = solve(0, 0, 0, &mut dp, &grid, &code, x, y);
    println!("{}", ans);
}
