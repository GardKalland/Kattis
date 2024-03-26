use std::io::{self, BufRead};

fn calculate_positions(time: f64, vehicles: &[(i32, i32)]) -> Vec<f64> {
    vehicles.iter().map(|&(x, v)| x as f64 + v as f64 * time).collect()
}

fn find_min_distance(n: usize, vehicles: &[(i32, i32)]) -> i64 {
    let (mut left, mut right, eps) = (0.0, 1e9, 1e-6);
    while right - left > eps {
        let mid = (left + right) / 2.0;
        let positions = calculate_positions(mid, vehicles);
        let spread_now = positions.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() - positions.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let positions_plus_eps = calculate_positions(mid + eps, vehicles);
        let spread_next = positions_plus_eps.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() - positions_plus_eps.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        
        if spread_next > spread_now {
            right = mid;
        } else {
            left = mid + eps;
        }
    }
    let min_distance_positions = calculate_positions(left, vehicles);
    let min_distance = min_distance_positions.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() - min_distance_positions.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    min_distance.round() as i64
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();


    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();


    let mut vehicles: Vec<(i32, i32)> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        vehicles.push((parts[0], parts[1]));
    }


    let min_distance: i64 = find_min_distance(n, &vehicles);
    println!("{:.3}", min_distance);
}
