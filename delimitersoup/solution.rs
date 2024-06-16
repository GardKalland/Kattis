
use std::collections::VecDeque;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut buffer = String::new();
    stdin_lock.read_line(&mut buffer).unwrap();
    let l: usize = buffer.trim().parse().unwrap();

    buffer.clear();
    stdin_lock.read_line(&mut buffer).unwrap();
    let input_array: Vec<char> = buffer.trim().chars().collect();

    let mut stack = VecDeque::new();
    let mut ok = true;

    for (i, &next_char) in input_array.iter().enumerate() {
        if next_char == ' ' {
            continue;
        } else {
            match next_char {
                ']' | ')' | '}' => {
                    if stack.is_empty() {
                        writeln!(stdout_lock, "{} {}", next_char, i).unwrap();
                        ok = false;
                        break;
                    } else {
                        let from_stack = stack.pop_back().unwrap();
                        if !((from_stack == '[' && next_char == ']')
                            || (from_stack == '(' && next_char == ')')
                            || (from_stack == '{' && next_char == '}'))
                        {
                            writeln!(stdout_lock, "{} {}", next_char, i).unwrap();
                            ok = false;
                            break;
                        }
                    }
                }
                _ => {
                    stack.push_back(next_char);
                }
            }
        }
    }
    if ok {
        writeln!(stdout_lock, "ok so far").unwrap();
    }
}
