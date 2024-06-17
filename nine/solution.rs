use std::io::{self, BufRead};

const EULER_TOTIENT: i64 = 1000000006;
const PRIME: i64 = 1000000007;

const PRE_CALC: [i64; 41] = [
    1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969, 14348907, 43046721, 129140163,
    387420489, 162261460, 486784380, 460353133, 381059392, 143178169, 429534507, 288603514, 865810542, 597431612, 792294829,
    376884473, 130653412, 391960236, 175880701, 527642103, 582926302, 748778899, 246336683, 739010049, 217030133, 651090399,
    953271190,
];

fn remain(d: i64) -> i64 {
    if d <= 40 {
        return PRE_CALC[d as usize];
    }
    let rem = remain(d >> 1);
    ((if d & 1 == 1 { 3 } else { 1 }) * rem % PRIME * rem % PRIME) % PRIME
}

fn experiment(d: i64) -> i64 {
    let mut d = (d - 1) << 1;
    if d >= EULER_TOTIENT {
        d -= (d / EULER_TOTIENT) * EULER_TOTIENT;
    }
    (8 * remain(d)) % PRIME
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let mut lines = input.iter();

    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..n {
        let d: i64 = lines.next().unwrap().trim().parse().unwrap();
        println!("{}", experiment(d));
    }
}
