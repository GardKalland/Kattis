use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let test_cases: i32 = input.trim().parse().unwrap();




    for _ in 0..test_cases {

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let counts: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();


        let mut imported_turtle = 0;
        let mut prev = 1;

        for &count in counts.iter() {
            if count == 0 { break; }

            if count > 2 * prev {
                imported_turtle += count - 2 * prev;
            }
            prev = count;
        }
        println!("{}", imported_turtle);
    }
    
}