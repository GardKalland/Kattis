
use std::f64::consts::PI;
use std::io::{self, BufRead};

fn update(p: &mut [f64; 3], cmd: &str, val: f64) {
    match cmd {
        "turn" => p[2] += val,
        _ => {
            p[0] += val * (p[2] * PI / 180.0).cos();
            p[1] += val * (p[2] * PI / 180.0).sin();
        }
    }
}

fn dist_squared(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    (p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)
}

fn test(t: usize, lines: &mut impl Iterator<Item = String>) {
    let mut pos_coll = Vec::new();
    let mut x_sum = 0.0;
    let mut y_sum = 0.0;

    for _ in 0..t {
        let s: Vec<String> = lines.next().unwrap().split_whitespace().map(String::from).collect();
        let mut pos = [
            s[0].parse::<f64>().unwrap(),
            s[1].parse::<f64>().unwrap(),
            s[3].parse::<f64>().unwrap(),
        ];
        let commands = &s[4..];
        let mut it = commands.iter();

        while let Some(cmd) = it.next() {
            let val = it.next().unwrap().parse::<f64>().unwrap();
            update(&mut pos, cmd, val);
        }

        pos_coll.push((pos[0], pos[1]));
        x_sum += pos[0];
        y_sum += pos[1];
    }

    let average = (x_sum / pos_coll.len() as f64, y_sum / pos_coll.len() as f64);
    let mut dist = 0.0;
    for &p in &pos_coll {
        let d = dist_squared(p, average);
        if d > dist {
            dist = d;
        }
    }
    println!("{} {} {}", average.0, average.1, dist.sqrt());
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let mut lines = input.into_iter();

    while let Some(line) = lines.next() {
        let n: usize = line.trim().parse().unwrap();
        if n == 0 {
            break;
        }
        test(n, &mut lines);
    }
}
