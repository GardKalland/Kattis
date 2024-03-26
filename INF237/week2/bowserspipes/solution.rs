use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse input
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let f: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let _ = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let d: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();

    // Precompute destinations and warps
    let mut destinations = vec![-1; n]; // Coin room destination for each pipe
    let mut min_warps = vec![usize::MAX; n]; // Minimum warps to reach a coin room for each pipe
    let mut pipe_to_coin_room = HashMap::new(); // Map from coin room to pipe with minimum warps

    for i in 0..n {
        if destinations[i] == -1 {
            dfs(i, &f, &mut destinations, &mut min_warps, 0);
        }
    }

    // Determine the optimal pipe for each of Luigi's checks
    for &pipe in d.iter() {
        let coin_room = destinations[pipe];
        if !pipe_to_coin_room.contains_key(&coin_room) || min_warps[pipe] < *pipe_to_coin_room.get(&coin_room).unwrap() {
            pipe_to_coin_room.insert(coin_room, min_warps[pipe]);
        }
    }

    // Output the optimal pipes for Mario
    for &pipe in &d {
        let coin_room = destinations[pipe];
        for (k, &v) in pipe_to_coin_room.iter() {
            if *k == coin_room && v == min_warps[pipe] {
                println!("{}", pipe);
                break;
            }
        }
    }
}

fn dfs(node: usize, f: &Vec<i32>, destinations: &mut Vec<i32>, min_warps: &mut Vec<usize>, warps: usize) {
    if f[node] == -1 {
        destinations[node] = node as i32; // This is a coin room
        min_warps[node] = warps;
    } else {
        let next = f[node] as usize;
        if destinations[next] == -1 {
            dfs(next, f, destinations, min_warps, warps + 1);
        }
        destinations[node] = destinations[next];
        min_warps[node] = min_warps[next] + 1;
    }
}
