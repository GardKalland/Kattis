use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

fn read_line() -> String {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    iterator.next().unwrap().unwrap()
}

fn main() {
    let input = read_line();
    let mut parts = input.trim().split_whitespace();
    let l = parts.next().unwrap().chars().next().unwrap();
    let n: usize = parts.next().unwrap().parse().unwrap();

    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let l_index = alphabet.find(l).unwrap();
    let alphabet_range = &alphabet[..=l_index];
    if l_index + 1 > n {
        println!("AMBIGUOUS");
        return;
    }

    let lines = (0..n).map(|_| read_line()).collect::<Vec<_>>();

    if let Some(result) = process_lines(&lines, alphabet_range) {
        println!("{}", result);
    } else {
        println!("IMPOSSIBLE");
    }
}

fn process_lines(lines: &[String], alphabet_range: &str) -> Option<String> {
    let mut comparisons = HashSet::new();
    for window in lines.windows(2) {
        let (prev, next) = (&window[0], &window[1]);
        let min_len = std::cmp::min(prev.len(), next.len());
        if prev.len() > next.len() && prev[..min_len] == next[..min_len] {
            return None;
        }
        for (c_prev, c_next) in prev.chars().zip(next.chars()).take(min_len) {
            if c_prev != c_next {
                if comparisons.contains(&(c_next, c_prev)) {
                    return None;
                }
                comparisons.insert((c_prev, c_next));
                break;
            }
        }
    }

    let mut graph: HashMap<char, Vec<char>> = HashMap::new();
    let mut in_counts: HashMap<char, usize> = alphabet_range.chars().map(|c| (c, 0)).collect();
    for (x, y) in comparisons {
        graph.entry(x).or_default().push(y);
        *in_counts.entry(y).or_default() += 1;
    }

    let mut queue: VecDeque<char> = in_counts.iter().filter_map(|(&c, &count)| if count == 0 { Some(c) } else { None }).collect();
    let mut order = Vec::new();
    while let Some(c) = queue.pop_front() {
        order.push(c);
        if let Some(next_chars) = graph.get(&c) {
            for &next_char in next_chars {
                if let Some(count) = in_counts.get_mut(&next_char) {
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(next_char);
                    }
                }
            }
        }
    }

    if order.len() != alphabet_range.len() {
        None
    } else {
        Some(order.into_iter().collect())
    }
}
 