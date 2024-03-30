use std::io;


fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let a: f64 = input.trim().parse().unwrap();

    let b = 100.0 - a;


    println!("{:.3}", 1.0 / (a / 100.0));
    println!("{:.3}", 1.0 / (b / 100.0));

}
