use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();


    let first_line: Vec<usize> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let num_players = first_line[0] + 1;
    let pairs = first_line[1];

    let mut graph = vec![Vec::new(); num_players];

    for _ in 0..pairs {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a_b: Vec<usize> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

        graph[a_b[0]].push(a_b[1]);
        graph[a_b[1]].push(a_b[0]);
    }

    let mut player_assignments: HashMap<usize, usize> = HashMap::new();

    for player_index in 1..num_players {
        let mut player_hits = vec![false; num_players];
        for &connected_player in &graph[player_index] {
            if eval_player(&mut player_assignments, &graph, &mut player_hits, connected_player) {
                player_assignments.insert(connected_player, player_index);
                break;
            }
        }
    }

    if player_assignments.len() != num_players - 1 {
        println!("Impossible");
    } else {
        for player in 1..num_players {
            println!("{}", player_assignments.get(&player).unwrap_or(&0));
        }
    }
}

fn eval_player(
    player_assignments: &mut HashMap<usize, usize>,
    social_graph: &[Vec<usize>],
    player_hits: &mut Vec<bool>,
    target_player: usize,
) -> bool {
    if let Some(&current_player) = player_assignments.get(&target_player) {
        if player_hits[target_player] {
            return false;
        }

        player_hits[target_player] = true;
        for &next_target in &social_graph[current_player] {
            if !player_hits[next_target] && eval_player(player_assignments, social_graph, player_hits, next_target) {
                player_assignments.insert(next_target, current_player);
                return true;
            }
        }
        false
    } else {
        true
    }
}
