use std::io::{self, BufRead};


fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut left = n;
    let mut right = n;

    while left % 100 != 99 && right % 100 != 99 {
        if left > 1 { left -= 1; } right += 1;}


    if left % 100 == 99 && right % 100 == 99 {
        println!("{}", right);
    } else if left % 100 == 99 {
        println!("{}", left);
    } else  {
        println!("{}", right);
    }
}
