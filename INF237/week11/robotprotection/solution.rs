
use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn cross(o: Point, a: Point, b: Point) -> i64 {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    }

    fn distance(&self, other: &Point) -> i64 {
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2)
    }
}

fn polar_angle_sort(points: &mut Vec<Point>) {
    let pivot = points.iter().min_by_key(|p| (p.y, p.x)).unwrap().clone();
    points.sort_by(|a, b| {
        let cross_product = Point::cross(pivot, *a, *b);
        if cross_product == 0 {
            pivot.distance(a).cmp(&pivot.distance(b))
        } else if cross_product > 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
}

fn graham_scan(mut points: Vec<Point>) -> Vec<Point> {
    if points.len() < 3 {
        return points;
    }

    polar_angle_sort(&mut points);

    let mut hull = Vec::new();
    for point in points {
        while hull.len() >= 2 && Point::cross(hull[hull.len() - 2], *hull.last().unwrap(), point) <= 0 {
            hull.pop();
        }
        hull.push(point);
    }
    hull
}

fn area_of_polygon(points: &[Point]) -> f64 {
    let mut area = 0.0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        area += (points[i].x * points[j].y - points[j].x * points[i].y) as f64;
    }
    area.abs() / 2.0
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(|line| line.ok());

    while let Some(line) = lines.next() {
        let n = line.parse::<usize>().unwrap();
        if n == 0 {
            break;
        }

        let points: Vec<Point> = (0..n)
            .map(|_| {
                let coords = lines.next().unwrap();
                let mut iter = coords.split_whitespace();
                let x = iter.next().unwrap().parse::<i64>().unwrap();
                let y = iter.next().unwrap().parse::<i64>().unwrap();
                Point { x, y }
            })
            .collect();

        let hull = graham_scan(points);
        let area = area_of_polygon(&hull);
        println!("{:.1}", area);
    }
}
