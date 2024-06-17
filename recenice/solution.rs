use std::io::{self, BufRead};

fn digit(i: i32) -> &'static str {
    match i {
        0 => "",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => "",
    }
}

fn tens(i: i32) -> &'static str {
    match i {
        0 => "",
        1 => "ten",
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "",
    }
}

fn str_num(i: i32) -> String {
    if i >= 100 {
        return format!("{}hundred{}", digit(i / 100), str_num(i % 100));
    }

    if i >= 10 && i <= 19 {
        return match i {
            10 => "ten".to_string(),
            11 => "eleven".to_string(),
            12 => "twelve".to_string(),
            13 => "thirteen".to_string(),
            14 => "fourteen".to_string(),
            15 => "fifteen".to_string(),
            16 => "sixteen".to_string(),
            17 => "seventeen".to_string(),
            18 => "eighteen".to_string(),
            19 => "nineteen".to_string(),
            _ => "".to_string(),
        };
    }

    let mut answer = String::new();
    if i >= 10 {
        answer.push_str(tens(i / 10));
        if i % 10 != 0 {
            answer.push_str(digit(i % 10));
        }
    } else {
        answer.push_str(digit(i));
    }

    answer
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let n: usize = input.next().unwrap().unwrap().trim().parse().unwrap();
    let mut sentence = Vec::with_capacity(n);
    let mut index = 0;
    let mut characters = 0;

    for i in 0..n {
        let word = input.next().unwrap().unwrap();
        if word == "$" {
            index = i;
        } else {
            characters += word.len();
        }
        sentence.push(word);
    }

    for i in 1..=1000 {
        if str_num(i as i32).len() + characters == i {
            sentence[index] = str_num(i as i32);
            break;
        }
    }

    for word in sentence {
        print!("{} ", word);
    }
    println!();
}
