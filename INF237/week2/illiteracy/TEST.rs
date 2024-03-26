use std::collections::{HashMap, VecDeque};
use std::io;

fn next_char(mut word: Vec<char>, index: usize) -> Vec<char> {
    if word[index] == 'F' {
        word[index] = 'A';
    } else {
        word[index] = ((word[index] as u8) + 1) as char;
    }
    word
}

fn main() {
    let mut start_word = String::new();
    io::stdin().read_line(&mut start_word).unwrap();
    let start_word = start_word.trim().chars().collect::<Vec<char>>();

    let mut end_word = String::new();
    io::stdin().read_line(&mut end_word).unwrap();
    let end_word = end_word.trim().chars().collect::<Vec<char>>();

    if start_word == end_word {
        println!("0");
        return;
    }

    let mut que = VecDeque::new();
    let mut searched = HashMap::new();
    let mut distance = HashMap::new();

    searched.insert(start_word.clone(), true);
    distance.insert(start_word.clone(), 0);
    que.push_back(start_word.clone());

    while let Some(word) = que.pop_front() {
        for i in 0..start_word.len() {
            let mut new_word = word.clone();
            match word[i] {
                'A' => {
                    if i == 0 {
                        new_word = next_char(new_word, i + 1);
                    } else if i == start_word.len() - 1 {
                        new_word = next_char(new_word, i - 1);
                    } else {
                        let hold = next_char(new_word.clone(), i - 1);
                        new_word = next_char(hold, i + 1);
                    }
                }
                'B' => {
                    if i != 0 && i != start_word.len() - 1 {
                        new_word[i + 1] = new_word[i - 1];
                    }
                }
                'C' => {
                    new_word = next_char(new_word, start_word.len() - (i + 1));
                }
                'D' => {
                    if i != 0 && i != start_word.len() - 1 {
                        if i < 4 {
                            for j in 0..i {
                                new_word = next_char(new_word, j);
                            }
                        } else {
                            for j in (i + 1..start_word.len()).rev() {
                                new_word = next_char(new_word, j);
                            }
                        }
                    }
                }
                'E' => {
                    if i != 0 && i != start_word.len() - 1 {
                        if i < 4 {
                            new_word = next_char(new_word, 0);
                            new_word = next_char(new_word, 2 * i);
                        } else {
                            new_word = next_char(new_word, start_word.len() - 1);
                            new_word = next_char(new_word, i - (start_word.len() - 1 - i));
                        }
                    }
                }
                'F' => {
                    if i % 2 == 0 {
                        new_word = next_char(new_word, 4 + i / 2);
                    } else {
                        new_word = next_char(new_word, i / 2);
                    }
                }
                _ => {}
            }
            if *searched.get(&new_word).unwrap_or(&false) {
                continue;
            }
            if new_word == end_word {
                println!("{}", distance[&word] + 1);
                return;
            }
            searched.insert(new_word.clone(), true);
            distance.insert(new_word.clone(), distance[&word] + 1);
            que.push_back(new_word);
        }
    }
}

