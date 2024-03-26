use std::io::{self, BufRead};

fn calculate_positions(time: f64, vehicles: &[(i32, i32)]) -> Vec<f64> {
    vehicles.iter().map(|&(x, v)| x as f64 + time * v as f64).collect()
}

fn min_spread_cover(vehicles: &[(i32, i32)]) -> f64 {
    let (mut start, mut end) = (0.0, 1e9);
    let precision = 1e-9; // Increased precision for floating-point calculations

    while end - start > precision {
        let mid = (start + end) / 2.0;
        let positions_now = calculate_positions(mid, vehicles);
        let spread_now = positions_now.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() - positions_now.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        let positions_next = calculate_positions(mid + precision, vehicles);
        let spread_next = positions_next.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() - positions_next.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        if spread_next < spread_now {
            start = mid;
        } else {
            end = mid;
        }
    }

    let final_positions = calculate_positions(start, vehicles);
    final_positions.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap() - final_positions.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}


fn condition(min_dist: f64) {
    if min_dist - min_dist.floor() < 1e-4 {
        println!("{}", min_dist.floor() as i64);
    }else {
        println!("{:.3}", min_dist);
    
    }
    
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

    let min_distance = min_spread_cover(&vehicles);
    println!("{:.0}", min_distance);
}
