use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

fn increment_char(chars: &mut [char], pos: usize) {
    chars[pos] = match chars[pos] {
        'F' => 'A',
        _ => ((chars[pos] as u8) + 1) as char,
    };
}

fn process_input() -> Vec<char> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().chars().collect()
}

fn main() {
    let start_sequence = process_input();
    let target_sequence = process_input();

    if start_sequence == target_sequence {
        println!("0");
        return;
    }

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let mut distances = HashMap::new();

    visited.insert(start_sequence.clone(), true);
    distances.insert(start_sequence.clone(), 0);
    queue.push_back(start_sequence.clone());

    while let Some(current_sequence) = queue.pop_front() {
        for index in 0..current_sequence.len() {
            let mut next_sequence = current_sequence.clone();
            let mut modify_indices: Vec<usize> = Vec::new();
            match current_sequence[index] {
                'A' => {
                    if index == 0 && current_sequence.len() > 1 {
                        modify_indices.push(1);
                    } else if index == current_sequence.len() - 1 && index > 0 {
                        modify_indices.push(index - 1);
                    } else {
                        modify_indices.push(index + 1);
                    }
                },
                'B' => if index > 0 && index < current_sequence.len() - 1 {
                    next_sequence[index + 1] = next_sequence[index - 1];
                },
                'C' => modify_indices.push(current_sequence.len() - (index + 1)),
                'D' => {
                    if index > 0 && index < current_sequence.len() - 1 {
                        if index < 4 {
                            modify_indices.extend(0..=index);
                        } else {
                            modify_indices.extend((index + 1..current_sequence.len()).rev());
                        }
                    }
                },
                'E' => {
                    if index > 0 && index < current_sequence.len() - 1 {
                        if index < 4 {
                            modify_indices.push(0);
                            modify_indices.push(2 * index);
                        } else {
                            modify_indices.push(current_sequence.len() - 1);
                            modify_indices.push(index - (current_sequence.len() - 1 - index));
                        }
                    }
                },
                'F' => {
                    modify_indices.push(if index % 2 == 0 { 4 + index / 2 } else { index / 2 });
                },
                _ => (),
            }
            for i in modify_indices {
                increment_char(&mut next_sequence, i);
            }

            if !visited.insert(next_sequence.clone(), true).unwrap_or(false) {
                continue;
            }

            if next_sequence == target_sequence {
                println!("{}", distances[&current_sequence] + 1);
                return;
            }

            distances.insert(next_sequence.clone(), distances[&current_sequence] + 1);
            queue.push_back(next_sequence);
        }
    }
}
