use std::collections::HashMap;
use std::f64;
use std::io;

const MAXN: usize = 200;

fn reset_graph(dis: &mut Vec<Vec<f64>>, n: usize) {
    for i in 0..n {
        for j in 0..n {
            dis[i][j] = if i == j { 1.0 } else { f64::MIN };
        }
    }
}

fn floyd_warshall(dis: &mut Vec<Vec<f64>>, n: usize) -> bool {
    let mut has_arbitrage = false;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dis[i][k] * dis[k][j] > dis[i][j] {
                    dis[i][j] = dis[i][k] * dis[k][j];
                    if i == j && dis[i][j] > 1.0 {
                        has_arbitrage = true;
                        return has_arbitrage;
                    }
                }
            }
        }
    }
    has_arbitrage
}

fn main() {
    let mut input = String::new();
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let n: usize = input.trim().parse().unwrap();
        if n == 0 {
            break;
        }
        let mut currency_index = HashMap::new();
        let mut dis = vec![vec![f64::MIN; MAXN]; MAXN];
        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let currency = input.trim().to_string();
            currency_index.insert(currency, i);
        }
        reset_graph(&mut dis, n);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let m: usize = input.trim().parse().unwrap();
        for _ in 0..m {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.split_whitespace().collect();
            let from = parts[0];
            let to = parts[1];
            let rates: Vec<f64> = parts[2].split(':').map(|r| r.parse().unwrap()).collect();
            let idx_from = *currency_index.get(from).unwrap();
            let idx_to = *currency_index.get(to).unwrap();
            dis[idx_from][idx_to] = dis[idx_from][idx_to].max(rates[1] / rates[0]);
        }
        let has_arbitrage = floyd_warshall(&mut dis, n);
        println!("{}", if has_arbitrage { "Arbitrage" } else { "Ok" });
        input.clear();
    }
}
