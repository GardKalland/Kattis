use std::io;


fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x:i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut tot = x;


    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut temp: i32 = input.trim().parse().unwrap();
        tot += x - temp;
    }
    println!("{}", tot);



}
