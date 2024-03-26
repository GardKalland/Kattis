use std::io::{self, Read};

fn swipe(mut v: Vec<i32>) -> Vec<i32> {
    let mut v2 = v.into_iter().filter(|&x| x != 0).collect::<Vec<_>>();
    v2.push(0);

    for i in 0..v2.len() - 1 {
        if v2[i] == v2[i + 1] {
            v2[i] *= 2;
            v2[i + 1] = 0;
        }
    }

    let v3 = v2.into_iter().filter(|&x| x != 0).collect::<Vec<_>>();
    let blanks = 4 - v3.len();
    let mut result = v3;
    result.extend(vec![0; blanks]);
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let mut board: Vec<Vec<i32>> = (0..4)
        .map(|_| (0..4).map(|_| iter.next().unwrap().parse().unwrap()).collect())
        .collect();

    let dir: i32 = iter.next().unwrap().parse().unwrap();

    match dir {
        0 => {
            for i in 0..4 {
                board[i] = swipe(board[i].clone());
            }
        },
        1 => {
            for i in 0..4 {
                let column: Vec<i32> = (0..4).map(|j| board[j][i]).collect();
                let swiped = swipe(column);
                for j in 0..4 {
                    board[j][i] = swiped[j];
                }
            }
        },
        2 => {
            for i in 0..4 {
                board[i].reverse();
                board[i] = swipe(board[i].clone());
                board[i].reverse();
            }
        },
        3 => {
            for i in 0..4 {
                let mut column: Vec<i32> = (0..4).map(|j| board[j][i]).collect();
                column.reverse();
                let swiped = swipe(column);
                for j in (0..4).rev() {
                    board[j][i] = swiped[3 - j];
                }
            }
        },
        _ => {}
    }

    for row in board {
        for (i, val) in row.iter().enumerate() {
            print!("{}{}", val, if i < 3 { " " } else { "\n" });
        }
    }
}
