

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums = input.split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

    let num1 = nums[0];
    let num2 = nums[1];
    let num3 = nums[2];

    let dif1 = num1 - num2;
    let dif2 = num2 - num3;

    if dif1 * dif2 < 0 {
        println!("turned");
    } else {
        let abs_dif1 = dif1.abs();
        let abs_dif2 = dif2.abs();

        if abs_dif2 == abs_dif1 {
            println!("cruised");
        } else if abs_dif2 < abs_dif1 {
            println!("braked");
        } else {
            println!("accelerated");
        }
    }
}
