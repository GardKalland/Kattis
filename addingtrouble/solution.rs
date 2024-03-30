use std::io;


fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let parts: Vec<isize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    if parts[0] + parts[1] == parts[2] {
        println!("correct!");
    } else {
        println!("wrong!");
    }
}
