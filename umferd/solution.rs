use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let m: usize = lines.next().unwrap().trim().parse().unwrap();
    let n: usize = lines.next().unwrap().trim().parse().unwrap();

    let mut empty_cells = 0;

    for _ in 0..n {
        let lane: String = lines.next().unwrap().trim().to_string();
        empty_cells += lane.matches('.').count();
    }

    let total_cells = m * n;
    let proportion = empty_cells as f64 / total_cells as f64;

    println!("{}", proportion);
    Ok(())
}
