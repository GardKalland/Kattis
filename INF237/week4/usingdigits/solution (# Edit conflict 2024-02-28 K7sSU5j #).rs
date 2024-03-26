use std::io::{self, BufRead};


const MAX_GRID_SIZE: usize = 101;
const INF: i32 = 1805;


fn solve(row: usize, col: usize, code_index: usize, dp: &mut Vec<Vec<Vec<i32>>>, grid: &Vec<Vec<i32>>, code: &Vec<usize>, max_col: usize, max_row: usize) -> i32 {
    
    if row >= max_row || col >= max_col {
        return INF;
    }
    
    if row == max_row - 1 && col == max_col - 1 {
        return grid[row][col];
    }
    
    if dp[row][col][code_index] < INF {
        return dp[row][col][code_index];
    }

    
    let mut min_path_sum = INF;
    
    if code_index < code.len() {
        let jump_distance = code[code_index];
        
        if row + jump_distance + 1 < max_row {
            min_path_sum = min_path_sum.min(grid[row][col] + solve(row + jump_distance + 1, col, code_index + 1, dp, grid, code, max_col, max_row));
        }
        
        if col + jump_distance + 1 < max_col {
            min_path_sum = min_path_sum.min(grid[row][col] + solve(row, col + jump_distance + 1, code_index + 1, dp, grid, code, max_col, max_row));
        }
    }

   
    if row + 1 < max_row {
        min_path_sum = min_path_sum.min(grid[row][col] + solve(row + 1, col, code_index, dp, grid, code, max_col, max_row));
    }
    
    if col + 1 < max_col {
        min_path_sum = min_path_sum.min(grid[row][col] + solve(row, col + 1, code_index, dp, grid, code, max_col, max_row));
    }

   
    dp[row][col][code_index] = min_path_sum;
    min_path_sum
}

fn main() {
   
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (max_col, max_row) = (dimensions[0], dimensions[1]);

    let code_str = lines.next().unwrap().unwrap();
    let code: Vec<usize> = code_str.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    let mut grid = vec![vec![0; max_col]; max_row];
    for i in 0..max_row {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            grid[max_row - i - 1][j] = c.to_digit(10).unwrap() as i32;
        }
    }

    let mut dp = vec![vec![vec![INF; code.len() + 1]; max_col]; max_row];
    
    let ans = solve(0, 0, 0, &mut dp, &grid, &code, max_col, max_row);
    println!("{}", ans);
}
