use std::io;



fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();


    for i in 1..=t  {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<isize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        let n = parts[1];
        let c = (n*(n+1))/2 + n;
        println!("{} {}", i, c);

    }

}
