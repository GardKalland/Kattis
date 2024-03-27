use std::io;


fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let line = input.trim(); // use this to take away \n
    println!("Kvedja,\n{}", line);


}
