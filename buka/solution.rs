use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: i32 = input.trim().parse().expect("Please type a number!");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let op = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: i32 = input.trim().parse().expect("Please type a number!");

    let result = if op == "*" {
        a * b
    } else if op == "+" {
        a + b
    } else {
        eprintln!("Unknown operator");
        return;
    };

    println!("{}", result);
}
