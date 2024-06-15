use std::collections::HashMap;
use std::io::{self, BufRead};

/// Man sjekker 2 ting, med å holde styr på pengene, så vil man bestikke den med høgast prosent sjange innen råd
/// så for 1 case så gjør: han med 70, så 60, så 50. ganger disse sammen og får 0.21
///
/// Case 2, Då er da mulig for at han med mest prosent stikk av, så det er 80 prosent for at han er med, 20 prosent på at han stikker.
/// Så då må vi bruke pengene på 2 andre. aka dei to med 20 prosent. Dei koster 400 og 500 kr.
///
/// Men! for å beregne det, så blir det 0,8 * 0,5 + 0,2 * 0,2 * 0,2
///
/// B is the amount of manney i need to birbe
/// P is the probeility of turning the henchman
/// n is the number of henchman
/// c the number i need to convert
/// m the amount of money i have



fn rec_dp(bitmask: usize, needed_hench: isize, current_bugdet: isize, tot_hench: isize, cost: &Vec<isize>, prob: &Vec<f64>, mem: &mut HashMap<(usize, isize, isize), f64>) -> f64 {
    if needed_hench == 0 {
        return 1.0;
    }

    if let Some(&val) = mem.get(&(bitmask, needed_hench, current_bugdet)) {
        return val;
    }

    if (bitmask.count_zeros() as isize + bitmask.count_ones() as isize) < needed_hench {
        mem.insert((bitmask, needed_hench, current_bugdet), 0.0);
        return 0.0;
    }

    let mut best: f64 = 0.0;

    for i in 0..tot_hench as usize {
        if (bitmask & (1 << i)) != 0 && current_bugdet >= cost[i] {
            let next = bitmask ^ (1 << i);
            let rem = current_bugdet - cost[i];
            best = best.max(
                prob[i] * rec_dp(next, needed_hench - 1, rem, tot_hench, cost, prob, mem)
                + (1.0 - prob[i]) * rec_dp(next, needed_hench, rem, tot_hench, cost, prob, mem)
            );
        }
    }

    mem.insert((bitmask, needed_hench, current_bugdet), best);
    best
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let test_cases: isize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..test_cases {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n: isize = iter.next().unwrap().parse().unwrap();
        let c: isize = iter.next().unwrap().parse().unwrap();
        let b: isize = iter.next().unwrap().parse().unwrap();

        let mut cost = Vec::new();
        let mut prob = Vec::new();

        for _ in 0..n {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let m: isize = iter.next().unwrap().parse().unwrap();
            let f: f64 = iter.next().unwrap().parse::<f64>().unwrap() / 100.0;
            cost.push(m);
            prob.push(f);
        }

        let mut mem: HashMap<(usize, isize, isize), f64> = HashMap::new();

        let res = rec_dp((1 << n) - 1, c, b, n, &cost, &prob, &mut mem);
        println!("{:.6}", res);
    }
}
