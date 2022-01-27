#![allow(dead_code)]

const GRID_SIZE: i32 = 9;

fn is_in_row(board: Vec<Vec<i32>>, number: i32, row: i32) -> bool {
    for i in 0..GRID_SIZE {
        if board[row as usize][i as usize] == number {
            return true;
        }
    }

    return false;
}

fn is_in_col(board: Vec<Vec<i32>>, number: i32, col: i32) -> bool {
    for i in 0..GRID_SIZE {
        if board[i as usize][col as usize] == number {
            return true;
        }
    }

    return false;
}

fn is_in_box(board: Vec<Vec<i32>>, number: i32, row: i32, col: i32) -> bool {
    let localrow = row - row % 3;
    let localcol = col - col % 3;

    for i in localrow..localrow + 3 {
        for j in localcol..localcol + 3 {
            if board[i as usize][j as usize] == number {
                return true;
            }
        }
    }

    return false;
}

fn is_valid_placement(board: Vec<Vec<i32>>, number: i32, row: i32, col: i32) -> bool {
    return !is_in_row(board.clone(), number, row) && !is_in_col(board.clone(), number, col) && !is_in_box(board.clone(), number, row, col);
}

fn can_be_solved(mut board: Vec<Vec<i32>>) -> bool {
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if board[row as usize][col as usize] == 0 {
                for attempt in 1..GRID_SIZE + 1 {
                    if is_valid_placement(board.clone(), attempt, row, col) {
                        board[row as usize][col as usize] = attempt;

                        if can_be_solved(board.clone()) {
                            return true;
                        } else {
                            board[row as usize][col as usize] = 0;
                        }
                    }
                }

                return false;
            }
        }
    }

    return true;
}

fn solve(board: &mut Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>> {
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            if board[row as usize][col as usize] == 0 {
                for attempt in 1..GRID_SIZE + 1 {
                    if is_valid_placement(board.clone(), attempt, row, col) {
                        board[row as usize][col as usize] = attempt;

                        if !solve(board).is_none() {
                            return Some(board.to_vec());
                        } else {
                            board[row as usize][col as usize] = 0;
                        }
                    }
                }

                return None;
            }
        }
    }

    return Some(board.to_vec());
}

fn main() {
    let mut board: Vec<Vec<i32>> = vec![
        vec![7, 0, 2, 0, 5, 0, 6, 0, 0],
        vec![0, 0, 0, 0, 0, 3, 0, 0, 0],
        vec![1, 0, 0, 0, 0, 9, 5, 0, 0],
        vec![8, 0, 0, 0, 0, 0, 0, 9, 0],
        vec![0, 4, 3, 0, 0, 0, 7, 5, 0],
        vec![0, 9, 0, 0, 0, 0, 0, 0, 8],
        vec![0, 0, 9, 7, 0, 0, 0, 0, 5],
        vec![0, 0, 0, 2, 0, 0, 0, 0, 0],
        vec![0, 0, 7, 0, 4, 0, 2, 0, 3],
    ];

    println!("{:?}", can_be_solved(board.clone()));
    println!("{:?}", solve(&mut board));
    println!("{:?}", board);
}
