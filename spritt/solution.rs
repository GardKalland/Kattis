use std::io;


fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let tot: Vec<isize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let n = tot[0];
    let x = tot[1];
    let mut count = 0;
    for _ in 1..=n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let a: isize = input.trim().parse().unwrap();
        count += a;

    }

    if count <= x {
        println!("Jebb");
    } else {
        println!("Neibb");
    }

}
