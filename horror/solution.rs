
use std::collections::{VecDeque, HashSet};
use std::io::{self, Read};

struct Graph {
    adj: Vec<Vec<usize>>,
    not_visited: HashSet<usize>,
    queue: VecDeque<(i32, usize)>,
}

impl Graph {
    fn construct_from_input() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        let mut lines = input.lines();
        let first_line: Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let v = first_line[0];
        let e = first_line[2];

        let mut graph = Graph::new(v);

        for s in lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()) {
            graph.add_src(s);
        }

        for line in lines.take(e) {
            let edge: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            graph.add_edge(edge[0], edge[1]);
        }

        graph
    }

    fn new(vertices: usize) -> Self {
        Graph {
            adj: vec![vec![]; vertices],
            not_visited: (0..vertices).collect(),
            queue: VecDeque::new(),
        }
    }

    fn add_edge(&mut self, v: usize, u: usize) {
        self.adj[v].push(u);
        self.adj[u].push(v);
    }

    fn add_src(&mut self, v: usize) {
        self.not_visited.remove(&v);
        self.queue.push_back((0, v));
    }

    fn bfs(&mut self) -> usize {
        let mut c = -1;
        let mut i = usize::MAX;

        while let Some((cost, v)) = self.queue.pop_front() {
            if cost > c || (cost == c && v < i) {
                c = cost;
                i = v;
            }
            for &u in &self.adj[v] {
                if self.not_visited.remove(&u) {
                    self.queue.push_back((cost + 1, u));
                }
            }
        }

        if self.not_visited.is_empty() {
            i
        } else {
            *self.not_visited.iter().min().unwrap()
        }
    }
}

fn main() {
    let mut graph = Graph::construct_from_input();
    println!("{}", graph.bfs());
}
