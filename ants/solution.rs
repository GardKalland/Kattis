use std::io;

fn main() {
    let mut input = String::new();


    io::stdin().read_line(&mut input).unwrap();
    let num_rounds: u32 = input.trim().parse().unwrap();

    for _ in 0..num_rounds {
        input.clear();

        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let l: i32 = parts.next().unwrap().parse().unwrap();
        let n: usize = parts.next().unwrap().parse().unwrap();

        let mut ants = Vec::new();
        while ants.len() < n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            ants.extend(input.split_whitespace().map(|x| x.parse::<i32>().unwrap()));
        }

        let mid = (l as f32 / 2.0).round() as i32;
        let mut worst = 0;
        let mut best = l;
        let mut best_ant: Option<i32> = None;
        for &ant in &ants {
            if (ant - mid).abs() < best {
                best = (ant - mid).abs();
                best_ant = Some(ant);
            }
            let longer = std::cmp::max((l - ant).abs(), ant);
            if longer > worst {
                worst = longer;
            }
        }

        if let Some(best_ant) = best_ant {
            best = std::cmp::min((l - best_ant).abs(), best_ant);
            println!("{} {}", best, worst);
        }
    }
}
