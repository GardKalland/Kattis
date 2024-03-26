use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut vars = HashMap::<String, i64>::new();
    let mut back_vars = HashMap::<i64, String>::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "def" => {
                let var = parts[1].to_string();
                let val: i64 = parts[2].parse().unwrap();
                if let Some(&old_val) = vars.get(&var) {
                    back_vars.remove(&old_val);
                }
                vars.insert(var.clone(), val);
                back_vars.insert(val, var);
            },
            "calc" => {
                let mut total = 0i64;
                let mut valid = true;
                let mut it = parts[1..].iter().peekable();
                let mut op = "+";
                let mut expression = Vec::new();

                while let Some(&next) = it.next() {
                    if next == "=" {
                        break;
                    }
                    expression.push(next);
                    let sign = it.peek().map_or("=", |&&s| s);
                    expression.push(sign);

                    let val = if let Some(&v) = vars.get(next) {
                        v
                    } else {
                        valid = false;
                        0
                    };

                    total = match op {
                        "+" => total.saturating_add(val),
                        "-" => total.saturating_sub(val),
                        _ => total,
                    };

                    if sign == "=" {
                        break;
                    } else {
                        op = sign;
                        it.next();
                    }
                }

                let result = if valid {
                    back_vars.get(&total).map_or_else(|| "unknown".to_string(), ToString::to_string)
                } else {
                    "unknown".to_string()
                };

                println!("{} {}", expression.join(" "), result);
            },
            "clear" => {
                vars.clear();
                back_vars.clear();
            },
            _ => (),
        }
    }
}
