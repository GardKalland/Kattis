use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let num: usize = input.trim().parse().unwrap();

    for _ in 0..num {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        let adv = input.trim();
        let mut stack = vec!['0'];
        let mut check = true;

        for item in adv.chars() {
            match item {
                '$' | '|' | '*' => stack.push(item),
                'b' if stack.last() == Some(&'$') => { stack.pop(); },
                't' if stack.last() == Some(&'|') => { stack.pop(); },
                'j' if stack.last() == Some(&'*') => { stack.pop(); },
                '.' => {},
                _ => {
                    check = false;
                    break;
                }
            }
        }

        if stack.len() > 1 {
            check = false;
        }

        if check {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
