use std::io;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim();
    if &n[..3] == "555" { // checking the first 3 elements
        println!("1");
    } else {
        println!("0");
    }

}
