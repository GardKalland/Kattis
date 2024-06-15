use std::collections::{VecDeque, HashMap};
use std::io::{self, Read};


fn bfs(goal_state: Vec<Vec<char>>) -> HashMap<Vec<Vec<char>>, i32> {
    let mut queue: VecDeque<Vec<Vec<char>>> = VecDeque::new();
    let mut distances: HashMap<Vec<Vec<char>>, i32> = HashMap::new();

    queue.push_back(goal_state.clone());
    distances.insert(goal_state, 0);

    let moves = vec![(1, 2), (-1, 2), (1, -2), (-1, -2), (2, 1), (-2, 1), (2, -1), (-2, -1)];

    while let Some(state) = queue.pop_front() {
        let dist = *distances.get(&state).unwrap();

        if dist == 10 {
            break;
        }

        let (blank_x, blank_y) = state.iter().enumerate().flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, &c)| if c == ' ' { Some((i, j)) } else { None })
        }).next().unwrap();

        for (dx, dy) in &moves {
            let new_x = (blank_x as isize) + dx;
            let new_y = (blank_y as isize) + dy;
            let x_valid: bool = new_x >= 0 && new_x < 5;
            let y_valid: bool = new_y >= 0 && new_y < 5;

            if x_valid && y_valid {
                let mut new_state = state.clone();
                new_state[blank_x][blank_y] = new_state[new_x as usize][new_y as usize];
                new_state[new_x as usize][new_y as usize] = ' ';

                if !distances.contains_key(&new_state) {
                    distances.insert(new_state.clone(), dist + 1);
                    queue.push_back(new_state);
                }
            }
        }
    }

    distances
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();

    let goal = vec![
        vec!['1', '1', '1', '1', '1'],
        vec!['0', '1', '1', '1', '1'],
        vec!['0', '0', ' ', '1', '1'],
        vec!['0', '0', '0', '0', '1'],
        vec!['0', '0', '0', '0', '0'],
    ];

    let distances = bfs(goal);

    for _ in 0..n {
        let mut board: Vec<Vec<char>> = Vec::new();
        for _ in 0..5 {
            let row: Vec<char> = lines.next().unwrap().chars().collect();
            board.push(row);
        }

        match distances.get(&board) {
            Some(&dist) => println!("Solvable in {} move(s).", dist),
            None => println!("Unsolvable in less than 11 move(s)."),
        }
    }
}
