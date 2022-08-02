use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn get_cell(row: usize, col: usize) -> usize {
            (row / 3) * 3 + col / 3
        }

        fn get_next_row(row: usize, col: usize) -> usize {
            row + (col + 1) / 9
        }

        fn get_next_col(col: usize) -> usize {
            (col + 1) % 9
        }

        pub fn get_next_pos(
            board: &Vec<Vec<char>>,
            mut row: usize,
            mut col: usize,
        ) -> (usize, usize) {
            while row != 9 {
                if board[row][col] == '.' {
                    return (row, col);
                }
                row = get_next_row(row, col);
                col = get_next_col(col);
            }
            return (9, 0);
        }

        fn backtrack(
            board: &mut Vec<Vec<char>>,
            row_contain: &mut HashSet<(usize, u32)>,
            col_contain: &mut HashSet<(usize, u32)>,
            cell_contain: &mut HashSet<(usize, u32)>,
            r: usize,
            c: usize,
        ) -> bool {
            let (R, C) = get_next_pos(&board, r, c);

            if R == 9 {
                return true;
            }
            let CL = get_cell(R, C);

            let mut used = HashSet::new();
            for n in 1..10 {
                let x = n as u32;
                let rr = (R, x);
                let cc = (C, x);
                let ccl = (CL, x);
                if row_contain.contains(&rr) {
                    used.insert(n);
                }
                if col_contain.contains(&cc) {
                    used.insert(n);
                }
                if cell_contain.contains(&ccl) {
                    used.insert(n);
                }
            }
            if used.len() == 9 {
                return false;
            }

            for idx in 1..10 {
                if !used.contains(&idx) {
                    board[R][C] = ((idx + 48) as u8) as char;

                    row_contain.insert((R, idx));
                    col_contain.insert((C, idx));
                    cell_contain.insert((CL, idx));
                    if backtrack(board, row_contain, col_contain, cell_contain, R, C) {
                        return true;
                    }

                    row_contain.remove(&(R, idx));
                    col_contain.remove(&(C, idx));
                    cell_contain.remove(&(CL, idx));
                }
            }

            board[R][C] = '.';
            return false;
        }

        let mut row_contain: HashSet<(usize, u32)> = HashSet::with_capacity(9 * 9);
        let mut col_contain: HashSet<(usize, u32)> = HashSet::with_capacity(9 * 9);
        let mut cell_contain: HashSet<(usize, u32)> = HashSet::with_capacity(9 * 9);
        for r in 0..9 {
            for c in 0..9 {
                if let Some(num) = board[r][c].to_digit(10) {
                    row_contain.insert((r, num));
                    col_contain.insert((c, num));
                    cell_contain.insert((get_cell(r, c), num));
                }
            }
        }

        backtrack(
            board,
            &mut row_contain,
            &mut col_contain,
            &mut cell_contain,
            0,
            0,
        );
    }

    pub fn test() {
        let input = [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let output = [
            ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            ['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        let mut board = Vec::new();
        let mut ans = Vec::new();
        for r in input.iter() {
            board.push(r.to_vec());
        }
        for r in output.iter() {
            ans.push(r.to_vec());
        }

        Self::solve_sudoku(&mut board);
        println!("Output:");
        for r in board.iter() {
            println!("{:?}", r);
        }
        println!("Expected:");
        for r in ans.iter() {
            println!("{:?}", r);
        }
        println!();

        //assert_eq!(board, ans);
    }
}

fn main() {
    Solution::test();
}
