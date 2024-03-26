use std::io;


fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_parts: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut h = input_parts[0];
    let mut m = input_parts[1];


    let mut tot_minutes = h * 60 + m;

    tot_minutes -= 45;

    if tot_minutes < 0 { tot_minutes += 24 * 60; }

    h = tot_minutes / 60;
    m = tot_minutes % 60;

    println!("{} {}", h, m);


}