use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn get_cell(row: usize, col: usize) -> usize {
        row / 3 * 3 + col / 3
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row_contain: Vec<HashSet<u32>> = vec![HashSet::new(); 9];
        let mut col_contain: Vec<HashSet<u32>> = vec![HashSet::new(); 9];
        let mut cell_contain: Vec<HashSet<u32>> = vec![HashSet::new(); 9];
        for r in 0..9 {
            for c in 0..9 {
                if let Some(num) = board[r][c].to_digit(10) {
                    row_contain[r].insert(num);
                    col_contain[c].insert(num);
                    cell_contain[Self::get_cell(r, c)].insert(num);
                }
            }
        }

        Self::backtrack(
            board,
            &mut row_contain,
            &mut col_contain,
            &mut cell_contain,
            (0, 0),
        );

        println!(
            "\n\n{:?}\n\n{:?}\n\n{:?}\n",
            row_contain, col_contain, cell_contain
        );
    }

    pub fn backtrack(
        board: &mut Vec<Vec<char>>,
        row_contain: &mut Vec<HashSet<u32>>,
        col_contain: &mut Vec<HashSet<u32>>,
        cell_contain: &mut Vec<HashSet<u32>>,
        pos: (usize, usize),
    ) {
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
