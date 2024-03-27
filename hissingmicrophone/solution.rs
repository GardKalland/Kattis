use std::io;



fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let line = input.trim();

    if line.contains("ss") {
        println!("hiss");
    } else {
        println!("no hiss");
    }

}
