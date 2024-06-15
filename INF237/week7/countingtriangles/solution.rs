use std::io;
use std::io::prelude::*;

#[derive(Clone, Copy)]
struct Line {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    iterator.next().unwrap().unwrap()
}

fn parse_line_segment(line: &str) -> Line {
    let parts: Vec<f64> = line.split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect();
    Line {
        x1: parts[0],
        y1: parts[1],
        x2: parts[2],
        y2: parts[3],
    }
}

fn intersect(l1: Line, l2: Line) -> (f64, f64) {
    let a1 = l1.y2 - l1.y1;
    let b1 = l1.x1 - l1.x2;
    let c1 = a1 * l1.x1 + b1 * l1.y1;

    let a2 = l2.y2 - l2.y1;
    let b2 = l2.x1 - l2.x2;
    let c2 = a2 * l2.x1 + b2 * l2.y1;

    let determinant = a1 * b2 - a2 * b1;

    let x = (b2 * c1 - b1 * c2) / determinant;
    let y = (a1 * c2 - a2 * c1) / determinant;

    (x, y)
}

fn can_intersect(l1: Line, l2: Line) -> bool {
    let (x, y) = intersect(l1, l2);

    x.is_finite() && y.is_finite() &&
    x >= l1.x1.min(l1.x2) && x <= l1.x1.max(l1.x2) &&
    x >= l2.x1.min(l2.x2) && x <= l2.x1.max(l2.x2) &&
    y >= l1.y1.min(l1.y2) && y <= l1.y1.max(l1.y2) &&
    y >= l2.y1.min(l2.y2) && y <= l2.y1.max(l2.y2)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        if n == 0 { break; }

        let mut segments = Vec::new();
        for _ in 0..n {
            if let Some(Ok(line)) = lines.next() {
                let parts: Vec<f64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
                let seg = Line {
                    x1: parts[0],
                    y1: parts[1],
                    x2: parts[2],
                    y2: parts[3],
                };
                segments.push(seg);
            }

        }

        let mut count = 0;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    if can_intersect(segments[i], segments[j]) &&
                       can_intersect(segments[j], segments[k]) &&
                       can_intersect(segments[i], segments[k]) {
                        count += 1;
                    }
                }
            }
        }

        println!("{}", count);
    }
}
