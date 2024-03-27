use std::io;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    if n == 2 {
        println!("blandad best");
    } else {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        println!("{}", line.trim());
    }
}
