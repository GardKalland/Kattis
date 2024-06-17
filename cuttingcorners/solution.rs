use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Corner {
    x: i32,
    y: i32,
}

impl Corner {
    fn new(x: i32, y: i32) -> Corner {
        Corner { x, y }
    }
}

fn angle_vector(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 * x2 + y1 * y2) / (x1.hypot(y1) * x2.hypot(y2))).acos()
}

fn angle_points(a: &Corner, b: &Corner, c: &Corner) -> f64 {
    let x1 = a.x as f64 - b.x as f64;
    let y1 = a.y as f64 - b.y as f64;
    let x2 = c.x as f64 - b.x as f64;
    let y2 = c.y as f64 - b.y as f64;
    angle_vector(x1, y1, x2, y2)
}

fn update(corners: &mut Vec<Corner>, angles: &mut Vec<f64>) {
    angles.clear();
    let n = corners.len();
    angles.push(angle_points(&corners[n - 1], &corners[0], &corners[1]));
    for i in 1..n {
        angles.push(angle_points(&corners[i - 1], &corners[i], &corners[(i + 1) % n]));
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    loop {
        let n: usize = match iter.next() {
            Some(n_str) => n_str.parse().unwrap(),
            None => break,
        };
        if n == 0 {
            break;
        }

        let mut corners = Vec::with_capacity(n);
        for _ in 0..n {
            let x: i32 = iter.next().unwrap().parse().unwrap();
            let y: i32 = iter.next().unwrap().parse().unwrap();
            corners.push(Corner::new(x, y));
        }

        let mut angles = Vec::with_capacity(n);
        update(&mut corners, &mut angles);

        let mut uni_min = 0.0;
        let mut last_removed = Corner::new(0, 0);
        let mut last_min_index = 0;

        loop {
            let mut min = f64::MAX;
            let mut min_index = 0;

            for (i, &angle) in angles.iter().enumerate() {
                if angle < min {
                    min = angle;
                    min_index = i;
                }
            }

            if min < uni_min {
                corners.insert(last_min_index, last_removed.clone());
                break;
            }

            if angles.len() == 3 {
                break;
            }

            last_removed = corners.remove(min_index);
            uni_min = min;
            last_min_index = min_index;

            update(&mut corners, &mut angles);
        }

        print!("{} ", corners.len());
        for (i, corner) in corners.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{} {}", corner.x, corner.y);
        }
        println!();
    }
}
