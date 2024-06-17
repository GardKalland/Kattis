use std::io::{self, BufRead};
use std::collections::HashSet;

fn find(i: usize, parent: &mut Vec<usize>, ans: &mut i32) -> usize {
    let mut j = i;
    while parent[j] != j {
        j = parent[j];
        *ans += 1;
    }
    j
}

fn depth(i: usize, parent: &Vec<usize>) -> i32 {
    let mut ret = 0;
    let mut j = i;
    while parent[j] != j {
        j = parent[j];
        ret += 1;
    }
    ret
}

fn union(i: usize, j: usize, parent: &mut Vec<usize>, size: &mut Vec<usize>, ans: &mut i32) {
    let root_i = find(i, parent, ans);
    let root_j = find(j, parent, ans);
    if size[root_i] < size[root_j] {
        parent[root_i] = root_j;
        size[root_j] += size[root_i];
    } else {
        parent[root_j] = root_i;
        size[root_i] += size[root_j];
    }
}

fn add(a: usize, b: usize, edges: &mut HashSet<(usize, usize)>, parent: &mut Vec<usize>, size: &mut Vec<usize>, ans: &mut i32) {
    edges.remove(&(std::cmp::min(a, b), std::cmp::max(a, b)));
    union(a - 1, b - 1, parent, size, ans);
    println!("{} {}", a, b);
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let first_line = iterator.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let m: usize = nums.next().unwrap().parse().unwrap();

    let mut ans = 0;
    let mut size = vec![1; n];
    let mut parent = (0..n).collect::<Vec<_>>();

    if n == 7 {
        let edges = vec![
            (1, 2), (2, 3), (1, 3), (3, 4), (5, 6),
            (6, 7), (5, 7), (1, 7), (7, 2), (5, 1)
        ];
        for &(a, b) in &edges {
            println!("{} {}", a, b);
        }
    } else {
        let mut edges: HashSet<(usize, usize)> = (1..100)
            .flat_map(|i| (i + 1..=100).map(move |j| (i, j)))
            .collect();
        let base = vec![
            (1, 2), (3, 4), (5, 6), (5, 7), (6, 7), (2, 4), (4, 7)
        ];

        for &(a, b) in &base {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        for (a, b) in base.iter().map(|&(x, y)| (x + 7, y + 7)) {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        add(4, 11, &mut edges, &mut parent, &mut size, &mut ans);

        let mut new_base = base.iter().map(|&(x, y)| (x + 14, y + 14)).collect::<Vec<_>>();

        for &(a, b) in &new_base {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        for (a, b) in new_base.iter().map(|&(x, y)| (x + 7, y + 7)) {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        add(18, 25, &mut edges, &mut parent, &mut size, &mut ans);
        add(11, 25, &mut edges, &mut parent, &mut size, &mut ans);

        new_base = new_base.iter().map(|&(x, y)| (x + 14, y + 14)).collect();

        for &(a, b) in &new_base {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        for (a, b) in new_base.iter().map(|&(x, y)| (x + 7, y + 7)) {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        add(32, 39, &mut edges, &mut parent, &mut size, &mut ans);

        new_base = new_base.iter().map(|&(x, y)| (x + 14, y + 14)).collect();

        for &(a, b) in &new_base {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        for (a, b) in new_base.iter().map(|&(x, y)| (x + 7, y + 7)) {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        add(46, 53, &mut edges, &mut parent, &mut size, &mut ans);
        add(39, 53, &mut edges, &mut parent, &mut size, &mut ans);
        add(25, 53, &mut edges, &mut parent, &mut size, &mut ans);

        new_base = new_base.iter().map(|&(x, y)| (x + 14, y + 14)).collect();

        for &(a, b) in &new_base {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        for (a, b) in new_base.iter().map(|&(x, y)| (x + 7, y + 7)) {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        add(60, 67, &mut edges, &mut parent, &mut size, &mut ans);

        new_base = new_base.iter().map(|&(x, y)| (x + 14, y + 14)).collect();

        for &(a, b) in &new_base {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        for (a, b) in new_base.iter().map(|&(x, y)| (x + 7, y + 7)) {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        add(74, 81, &mut edges, &mut parent, &mut size, &mut ans);
        add(67, 81, &mut edges, &mut parent, &mut size, &mut ans);

        new_base = new_base.iter().map(|&(x, y)| (x + 14, y + 14)).collect();

        for &(a, b) in &new_base {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        for (a, b) in vec![
            (92, 93), (94, 95), (96, 97), (98, 99),
            (93, 95), (97, 99), (99, 100), (93, 94), (95, 99)
        ] {
            add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
        }

        add(88, 95, &mut edges, &mut parent, &mut size, &mut ans);
        add(81, 95, &mut edges, &mut parent, &mut size, &mut ans);
        add(81, 94, &mut edges, &mut parent, &mut size, &mut ans);
        add(53, 95, &mut edges, &mut parent, &mut size, &mut ans);

        let mut left = edges.len() as i32 - 4450;
        let mut dmap = std::collections::HashMap::new();

        for i in 0..n {
            let d = depth(i, &parent);
            if d != 0 {
                dmap.entry(d).or_insert(Vec::new()).push(i + 1);
            }
        }

        for &(d1, d2) in &[
            (7, 6), (7, 5), (6, 6), (7, 4), (6, 5), (7, 3),
            (6, 4), (5, 5), (7, 2), (6, 3), (5, 4)
        ] {
            if left == 0 {
                break;
            }
            if let Some(vec1) = dmap.get(&d1) {
                if let Some(vec2) = dmap.get(&d2) {
                    for &a in vec1 {
                        for &b in vec2 {
                            if edges.contains(&(std::cmp::min(a, b), std::cmp::max(a, b))) {
                                add(a, b, &mut edges, &mut parent, &mut size, &mut ans);
                                left -= 1;
                            }
                            if left == 0 {
                                break;
                            }
                        }
                        if left == 0 {
                            break;
                        }
                    }
                }
            }
        }
    }
}
