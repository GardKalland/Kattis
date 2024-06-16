
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let code = input.trim();

    let mut nops = 0;
    let mut count = 0;

    for letter in code.chars() {
        if letter.is_ascii_uppercase() {
            if count % 4 == 0 {
                count = 0;
            } else {
                let num = 4 - (count % 4);
                nops += num;
                count = 0;
            }
        }
        count += 1;
    }

    println!("{}", nops);
}
