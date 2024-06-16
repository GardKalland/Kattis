use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    let mut buffer = String::new();
    stdin_lock.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();

    let (mut tb, mut lr) = (0, 0);

    for _ in 0..n {
        buffer.clear();
        stdin_lock.read_line(&mut buffer).unwrap();
        let input = buffer.trim();
        let (a, b) = count_zeros(input);
        tb += a;
        lr += b;
    }

    let swords = tb.min(lr) / 2;
    println!("{} {} {}", swords, tb - swords * 2, lr - swords * 2);
}

fn count_zeros(s: &str) -> (i32, i32) {
    let (tb_count, lr_count) = s.chars().enumerate().fold((0, 0), |(tb, lr), (i, c)| {
        if c == '0' {
            if i < 2 {
                (tb + 1, lr)
            } else {
                (tb, lr + 1)
            }
        } else {
            (tb, lr)
        }
    });
    (tb_count, lr_count)
}
