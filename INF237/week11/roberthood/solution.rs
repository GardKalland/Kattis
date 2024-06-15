
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn cross(o: Point, a: Point, b: Point) -> i64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

fn distance_sq(a: Point, b: Point) -> f64 {
    ((b.x - a.x).pow(2) as f64) + ((b.y - a.y).pow(2) as f64)
}

fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    points.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
    let mut lower = Vec::new();
    for p in points.iter() {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], *p) <= 0 {
            lower.pop();
        }
        lower.push(*p);
    }

    let mut upper = Vec::new();
    for p in points.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], *p) <= 0 {
            upper.pop();
        }
        upper.push(*p);
    }

    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn max_distance_on_hull(points: Vec<Point>) -> f64 {
    let hull = convex_hull(points);
    let n = hull.len();
    if n < 2 {
        return 0.0;
    }

    let mut max_dist = 0.0;
    for i in 0..n {
        let mut j = (i + 1) % n;
        loop {
            let d = distance_sq(hull[i], hull[j]);
            if d > max_dist {
                max_dist = d;
            }
            let next_j = (j + 1) % n;
            // Move j if rotating 'j' improves the distance
            if distance_sq(hull[i], hull[next_j]) > d {
                j = next_j;
            } else {
                break;
            }
        }
    }

    max_dist.sqrt()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut points = Vec::new();
    for line in lines {
        let coords: Vec<i64> = line.unwrap().split_whitespace()
                                    .map(|num| num.parse().unwrap())
                                    .collect();
        points.push(Point { x: coords[0], y: coords[1] });
    }

    let max_distance = max_distance_on_hull(points);
    println!("{}", max_distance);
}
