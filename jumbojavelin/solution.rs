use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: usize = input.trim().parse().unwrap();

    let mut rods = Vec::new();
    while rods.len() < n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        rods.extend(input.split_whitespace().map(|x| x.parse::<i32>().unwrap()));
    }
    let mut sum = 0;
    for i in 0..n{
        sum += rods[i];
    }


    println!("{}", sum - (n as i32 - 1));
}
