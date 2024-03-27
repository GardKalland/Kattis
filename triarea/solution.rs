use std::io;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let parts: Vec<f32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{}" , (parts[0] * parts[1]) / 2.0);
}
