use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();
    let d: i32 = iter.next().unwrap().parse().unwrap();
    let l: i32 = iter.next().unwrap().parse().unwrap();

    let mut found = false;

    for _b in 0..=l / b {
        for _c in 0..=l / c {
            let x1 = _b * b + _c * c;
            if x1 > l {
                break;
            }
            for _d in 0..=l / d {
                let x2 = x1 + _d * d;
                if x2 > l {
                    break;
                }
                if x2 == l {
                    println!("{} {} {}", _b, _c, _d);
                    found = true;
                }
            }
        }
    }

    if !found {
        println!("impossible");
    }
}
