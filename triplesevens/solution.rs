use std::io;

fn main() {

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: isize = input.trim().parse().unwrap();

    let mut good = true;

    for i in 0..3 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let d: Vec<char> = input.trim().chars().collect();

        if !d.contains(&'7') {
            good = false;
            break
        }
    }

    println!("{}", if good { "777" } else { "0" });
}
