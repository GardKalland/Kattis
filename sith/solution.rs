use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let result: i32 = input.trim().parse().unwrap();


    if result == a - b && result != (a - b).abs() {
        println!("JEDI");
    } else if result != a - b && result == (a - b).abs() {
        println!("SITH");
    } else {
        println!("VEIT EKKI");
    }
}
