use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let num: usize = input.trim().parse().unwrap();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Top
    writeln!(handle, "+{}+", "-".repeat(num)).unwrap();

    // Middle
    for _ in 0..num {
        writeln!(handle, "|{}|", " ".repeat(num)).unwrap();
    }

    // Bottom
    writeln!(handle, "+{}+", "-".repeat(num)).unwrap();
}
