
use std::io;


fn main() {
    let mut input = String::new();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input).unwrap();
    io::stdin().read_line(&mut input2).unwrap();

    let n: i32 = input.trim().parse().unwrap();
    let m: i32 = input2.trim().parse().unwrap();
    println!("{}", n - m)

}
