use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let g: usize = input.trim().parse().unwrap();

    for game in 1..=g {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let m: usize = input.trim().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let tiles: Vec<u32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let (n, t) = (parts[0], parts[1]);

        let result = odds(&tiles, n, t);
        println!("Game {} -- {} : {}", game, result.0, result.1);
    }
}

fn odds(tiles: &Vec<u32>, n: usize, t: usize) -> (usize, usize) {
    let mut dp = vec![vec![0; t+1]; n+1];
    dp[0][0] = 1;

    for &tile in tiles {
        for i in (1..=n).rev() {
            for j in (0..=t).rev() {
                if j >= tile as usize {
                    dp[i][j] += dp[i-1][j-tile as usize];
                }
            }
        }
    }

    let win = dp[n][t];
    let tot_comb = bino_coeff(tiles.len(), n);
    let lose = tot_comb - win;

    (win, lose)
}

fn bino_coeff(n: usize, k: usize) -> usize {
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}
