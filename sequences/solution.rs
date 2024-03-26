use std::io::{self, BufRead};



fn main() {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut lines = lock.lines();

    let s = lines.next().unwrap().unwrap();

    let modu: i64 = 1_000_000_007;
    let mut seenq = 0;
    let mut seen1 = 0;
    let mut total = 0;
    let mut prev = 1;
    let mut pow2 = 1;

    for c in s.chars() {
        match c {
            '?' => {
                total = (total * 2 % modu) + (seen1 * pow2 % modu);
                total %= modu;

                total += (seenq * prev) % modu;
                total %= modu;

                seenq += 1;

                prev = pow2;
                pow2 = pow2 * 2 % modu;
            }
            '1' => {
                seen1 += 1;
            }
            '0' => {
                if seenq > 0 {
                    total += (seen1 * pow2 % modu) + (prev * seenq % modu);
                    total %= modu;
                } else {
                    total += seen1;
                    total %= modu;
                }
            }
            _ => {}
        }
    }

    println!("{}", total % modu);
}
