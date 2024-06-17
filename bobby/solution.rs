use std::io::{self, BufRead};

const MAX_N: usize = 10;
static mut BINOMIAL: [[i32; 11]; 11] = [[0; 11]; 11];

fn binomial(n: usize, k: isize) -> i32 {
    if k < 0 || k > n as isize {
        0
    } else {
        unsafe { BINOMIAL[n][k as usize] }
    }
}

fn set_binomial() {
    unsafe {
        BINOMIAL[0][0] = 1;
        for n in 1..=MAX_N {
            for k in 0..=n {
                BINOMIAL[n][k] = binomial(n - 1, k as isize - 1) + binomial(n - 1, k as isize);
            }
        }
    }
}

fn test_case(line: &str) {
    let parts: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (r, s, x, y, w) = (parts[0], parts[1], parts[2], parts[3], parts[4]);

    let above_or_eq = (s - r + 1) as f64 / s as f64;
    let below = 1.0 - above_or_eq;
    let mut p_win = 0.0;

    for i in x..=y {
        p_win += binomial(y as usize, i as isize) as f64 * above_or_eq.powf(i as f64) * below.powf((y - i) as f64);
    }

    let p_lose = 1.0 - p_win;
    if p_win * (w as f64 - 1.0) + p_lose * -1.0 > 0.0 {
        println!("yes");
    } else {
        println!("no");
    }
}

fn main() {
    set_binomial();
    let stdin = io::stdin();
    let input = stdin.lock().lines().collect::<Vec<_>>();
    let n: usize = input[0].as_ref().unwrap().trim().parse().unwrap();
    for i in 1..=n {
        test_case(&input[i].as_ref().unwrap());
    }
}
