use std::io;


fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Some(x) = input.find('a') {
        println!("{}", &input[x..].trim());
    }


}
