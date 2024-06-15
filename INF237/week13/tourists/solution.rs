
use std::io::{self, BufRead};
use std::collections::VecDeque;

struct Node {
    depth: usize,
    parent: isize,
    parents: [isize; 18],
    con: Vec<usize>,
}

impl Node {
    fn new() -> Self {
        Self {
            depth: 0,
            parent: -1,
            parents: [-1; 18],
            con: Vec::new(),
        }
    }
}

fn read_input() -> (usize, Vec<Node>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut nodes: Vec<Node> = (0..n).map(|_| Node::new()).collect();

    for _ in 0..(n - 1) {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let a = parts[0] - 1;
        let b = parts[1] - 1;
        nodes[a].con.push(b);
        nodes[b].con.push(a);
    }

    (n, nodes)
}

fn dfs(v: usize, p: isize, nodes: &mut Vec<Node>) {
    let mut stack = VecDeque::new();
    stack.push_back((v, p));

    while let Some((v, p)) = stack.pop_back() {
        if p != -1 {
            nodes[v].parent = p;
            nodes[v].depth = nodes[p as usize].depth + 1;
        }

        for &x in &nodes[v].con {
            if x as isize != p {
                stack.push_back((x, v as isize));
            }
        }
    }
}

fn lca(mut a: usize, mut b: usize, nodes: &Vec<Node>) -> usize {
    if nodes[a].depth < nodes[b].depth {
        std::mem::swap(&mut a, &mut b);
    }

    for i in (0..18).rev() {
        if nodes[a].depth >= nodes[b].depth + (1 << i) {
            a = nodes[a].parents[i] as usize;
        }
    }

    if a == b {
        return a;
    }

    for i in (0..18).rev() {
        if nodes[a].parents[i] != nodes[b].parents[i] {
            a = nodes[a].parents[i] as usize;
            b = nodes[b].parents[i] as usize;
        }
    }

    nodes[a].parents[0] as usize
}

fn between(a: usize, b: usize, nodes: &Vec<Node>) -> usize {
    let lca_node = lca(a, b, nodes);
    nodes[a].depth + nodes[b].depth - 2 * nodes[lca_node].depth + 1
}

fn main() {
    let (n, mut nodes) = read_input();

    nodes[0].depth = 0;
    nodes[0].parent = -1;
    dfs(0, -1, &mut nodes);

    for node in &mut nodes {
        node.parents[0] = node.parent;
    }

    for i in 1..18 {
        for j in 0..n {
            let parent_index = nodes[j].parents[i - 1];
            let grandparent_index = if parent_index == -1 {
                -1
            } else {
                nodes[parent_index as usize].parents[i - 1]
            };
            nodes[j].parents[i] = grandparent_index;
        }
    }

    let mut sum: u64 = 0;

    for i in 1..=n {
        for j in (i * 2..=n).step_by(i) {
            sum += between(i - 1, j - 1, &nodes) as u64;
        }
    }

    println!("{}", sum);
}
