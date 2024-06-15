use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let values: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let length = values[0];

        println!("length {}", length);

        line.clear();
        
        io::stdin().read_line(&mut line).unwrap();
        let ants: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        println!("ants {:?}", ants);
        
        let mut min_time = 0;
        let mut max_time = 0;

        for &ant in &ants {
            
            let dist_near_end = ant.min(length - ant);
            let dist_far_end = ant.max(length - ant);
    
            min_time = min_time.max(dist_near_end);
            max_time = max_time.max(dist_far_end);
        }

        println!("{} {}", min_time, max_time);
    }
}
