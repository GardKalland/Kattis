use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let word = input.trim();
    let mut count = 0;
    let mut count2 = 0;
    for i in 0..word.len() {

        if is_vowel(word.chars().nth(i).unwrap()) {
            count += 1;
        }
        if is_vowel(word.chars().nth(i).unwrap()) || word.chars().nth(i).unwrap() == 'y' {
            count2 += 1;
        }


    }

    println!("{} {}", count, count2);

}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
