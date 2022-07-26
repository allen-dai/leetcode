use std::collections::HashMap;

fn main() {
    let matrix: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    dbg!(ans(matrix));
}

fn ans(board: Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            match board[i][j] {
                '.' => continue,
                c => {
                    if board[i][(j + 1)..9].contains(&c)
                        || board[(i + 1)..9].iter().any(|v| v[j] == c)
                    {
                        return false;
                    }
                }
            }
        }
    }

    let box_index: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, 3),
        (0, 6),
        (3, 0),
        (3, 3),
        (3, 6),
        (6, 0),
        (6, 3),
        (6, 6),
    ];

    let mut b: Vec<char> = Vec::new();

    for i in box_index {
        for x_d in 0..3 {
            for y_d in 0..3 {
                let x = i.0 + x_d;
                let y = i.1 + y_d;
                let c = board[x as usize][y as usize];
                if c != '.' {
                    if b.contains(&c) {
                        return false;
                    }
                    b.push(c);
                }
            }
        }
        b.clear();
    }

    return true;
}
