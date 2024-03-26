use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Calculates shortest paths from all nodes to the target (Jimmy's house) using Dijkstra's algorithm.
fn dijkstra(graph: &HashMap<usize, Vec<(usize, usize)>>, target: usize) -> HashMap<usize, usize> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(target, 0);
    heap.push(State { cost: 0, position: target });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > *dist.get(&position).unwrap_or(&usize::MAX) {
            continue; // Ignore longer paths, inherently addressing cycles
        }

        if let Some(edges) = graph.get(&position) {
            for &(next, weight) in edges {
                let next_cost = cost + weight;
                if next_cost < *dist.get(&next).unwrap_or(&usize::MAX) {
                    heap.push(State { cost: next_cost, position: next });
                    dist.insert(next, next_cost);
                }
            }
        }
    }

    dist
}

// Counts valid paths from the start node to the target node, using shortest paths to ensure progress.
fn count_paths(
    graph: &HashMap<usize, Vec<(usize, usize)>>,
    shortest_paths: &HashMap<usize, usize>,
    start: usize,
    end: usize,
) -> usize {
    let mut queue = VecDeque::new();
    let mut paths = HashMap::new();

    paths.insert(start, 1);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&node) {
            for &(neighbor, _) in neighbors {
                // Ensure the neighbor is closer to the target based on shortest path distances.
                if shortest_paths.get(&neighbor).unwrap_or(&usize::MAX) < shortest_paths.get(&node).unwrap_or(&usize::MAX) {
                    let node_paths = *paths.get(&node).unwrap_or(&0);
                    *paths.entry(neighbor).or_insert(0) += node_paths;
                    // No need to check 'visited' here as Dijkstra's ensures we move closer to target
                }
            }
        }
    }

    *paths.get(&end).unwrap_or(&0)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(Ok(line)) = lines.next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "0" {
            break;
        }

        let m: usize = parts[1].parse().unwrap();

        let mut graph = HashMap::new();

        for _ in 0..m {
            if let Some(Ok(edge_line)) = lines.next() {
                let parts: Vec<usize> = edge_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
                let (a, b, d) = (parts[0], parts[1], parts[2]);
                graph.entry(a).or_insert_with(Vec::new).push((b, d));
                graph.entry(b).or_insert_with(Vec::new).push((a, d)); // Graph is bidirectional
            }
        }

        let shortest_paths = dijkstra(&graph, 2); // Calculate shortest paths from every node to Jimmy's house (2)

        let paths_count = count_paths(&graph, &shortest_paths, 1, 2); // Count valid paths from the office (1) to the house (2)
        println!("{}", paths_count);
    }
}
