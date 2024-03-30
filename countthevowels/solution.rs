use std::io;



fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut count = 0;

    for i in 0..input.len() {
        if vowel(input.to_lowercase().chars().nth(i).unwrap()) {
            count += 1;
        }
    }
    println!("{}", count);

}

fn vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
