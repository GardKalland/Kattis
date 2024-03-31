use std::io;


fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    for i in (0..n) {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if i % 2 == 0 {
            println!("{}", input.trim());
        }
    }
}
