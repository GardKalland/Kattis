use std::io;


fn main() {


    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: Vec<f64> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (x, y, x1,y1, x2, y2 ) = (n[0], n[1], n[2], n[3], n[4], n[5]);
    let mut z:f64 = 0.0;


    if x > x1 && x < x2 && y > y2 {
        z = y - y2;
    } else if x > x2 && y > y2 {
        z = (x - x2).hypot(y - y2);
    } else if x > x2 && y > y1 && y < y2 {
        z = x - x2;
    } else if x > x2 && y < y1 {
        z = (x-x2).hypot(y1-y);
    } else if x > x1 && x < x2 && y < y1 {
        z = y1 - y;
    } else if x < x1 && y < y1 {
        z = (x1 - x).hypot(y1 - y);
    } else if x < x1 && y > y1 && y < y2 {
        z = x1 - x;
    } else if x < x1 && y > y2 {
        z = (x1 - x).hypot(y - y2);
    }
    println!("{}", z);
}
