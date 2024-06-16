 use std::io::{self, BufRead};


 #[derive(Debug)]
 struct Fraction {
     n: u32,
     d: u32,
 }

 impl Fraction {
     fn new() -> Self {
         Fraction {
             n: 1,
             d: 1,
         }
     }
 }


 fn find_frac(node: u32, frac: &mut Fraction) {
     let mut bin = 0;
     let mut start = 0;
     let mut node = node;

     while node != 1 {
         if node & 1 != 0 {
             bin |= 1 << start;
         }
         start += 1;
         node >>= 1;

     }


     for i in (0..start).rev() {
         if bin & (1 << i) != 0 {
             frac.n += frac.d;
         } else {
             frac.d += frac.n;
         }

     }
 }


 fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let c: u32 = iter.next().unwrap().parse().unwrap();
        let node: u32 = iter.next().unwrap().parse().unwrap();

        let mut frac = Fraction::new();
        find_frac(node, &mut frac);

        println!("{} {}/{}", c, frac.n, frac.d);
    }
 }
