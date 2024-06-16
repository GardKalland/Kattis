use std::io;

fn main () {
    let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    let n: f64 = input.trim().parse().unwrap();
    let result = (n.sqrt()) * 4.0;
    println!("{}", result);
}
