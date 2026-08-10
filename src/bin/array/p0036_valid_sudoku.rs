/*
Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
1. Each row must contain the digits 1-9 without duplicates.
2. Each column must contain the digits 1-9 without duplicates.
3. Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without duplicates.

Note:
A Sudoku board (partially filled) could be valid but is not necessarily solvable.
Only the filled cells need to be validated according to the mentioned rules.

Constraints:
board.length == 9
board[i].length == 9
board[i][j] is a digit 1-9 or '.'.
*/

use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];

    // If you use the 0-8 math formula we talked about, you can use a Vec for squares too:
    let mut squares = vec![HashSet::new(); 9];
    for i in 0..81 {
        let r = i / 9; // Integer division gives you the row (0 to 8)
        let c = i % 9; // Modulo gives you the column (0 to 8)
        let val = board[r as usize][c as usize];

        if val == '.' {
            continue;
        }

        // .insert() returns false if the value was already present
        if !rows[r].insert(val)
            || !cols[c].insert(val)
            || !squares[(r / 3) * 3 + (c / 3)].insert(val)
        {
            return false;
        }
    }
    true
}

fn main() {
    let test_cases = vec![
        (
            vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            true,
        ),
        (
            vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            false, // two 8s in the first column
        ),
        (
            vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ],
            true, // entirely empty board is valid
        ),
    ];

    for (i, (board, expected)) in test_cases.into_iter().enumerate() {
        let result = is_valid_sudoku(board.clone());
        if result == expected {
            println!("Test {} PASSED", i + 1);
        } else {
            println!(
                "Test {} FAILED: Expected {}, got {}",
                i + 1,
                expected,
                result
            );
        }
    }
}
