use std::io;



fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input = input.trim().split_whitespace();
    let n: i32 = input.next().unwrap().parse().unwrap();
    let m: i32 = input.next().unwrap().parse().unwrap();

    if n > m {
        println!("{} {}", m, n);
        return;
    } else {

        println!("{} {}", n,m);

    }

}
