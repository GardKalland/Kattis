use std::io;


fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();

    if a > b {
        println!("{}",1);
    } else if a < b {
        println!("{}",0);
    } else {
        println!("{}",0);
    }
}
