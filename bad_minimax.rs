#![allow(dead_code)]

use std::cmp;

#[derive(Debug, PartialEq, Clone, Copy)]
enum BoardCell {
    Absent,
    Ally,
    Adversary,
}

struct Move(usize, usize);

type Board = [[BoardCell; 3]; 3];

fn are_moves_left(board: &Board) -> bool {
    for y in 0..3 {
        for x in 0..3 {
            if board[y][x] == BoardCell::Absent {
                return true;
            }
        }
    }

    return false;
}

fn evaluate(board: &Board, is_maximizing: bool) -> i32 {
    for y in 0..3 {
        if board[y][0] == board[y][1] && board[y][1] == board[y][2] {
            if board[y][0] == BoardCell::Adversary {
                return if is_maximizing {
                    10
                } else {
                    -10
                };
            } else if board[y][0] == BoardCell::Ally {
                return if is_maximizing {
                    -10
                } else {
                    10
                };
            }
        }
    }

    for x in 0..3 {
        if board[0][x] == board[1][x] && board[1][x] == board[2][x] {
            if board[0][x] == BoardCell::Adversary {
                return if is_maximizing {
                    10
                } else {
                    -10
                };
            } else if board[0][x] == BoardCell::Ally {
                return if is_maximizing {
                    -10
                } else {
                    10
                };
            }
        }
    }

    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        if board[0][0] == BoardCell::Adversary {
            return if is_maximizing {
                10
            } else {
                -10
            };
        } else if board[0][0] == BoardCell::Ally {
            return if is_maximizing {
                -10
            } else {
                10
            };
        }
    }

    if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
         if board[0][2] == BoardCell::Adversary {
            return if is_maximizing {
                10
            } else {
                -10
            };
        } else if board[0][2] == BoardCell::Ally {
            return if is_maximizing {
                -10
            } else {
                10
            };
        }
    }

    return 0;
}

fn minimax(board: &mut Board, depth: i32, is_maximizing: bool) -> i32 {
    let score = evaluate(board, is_maximizing);

    if score == 10 {
        return score;
    }

    if score == -10 {
        return score;
    }

    if !are_moves_left(board) {
        return 0;
    }

    if is_maximizing {
        let mut best = 1000;

        for y in 0..3 {
            for x in 0..3 {
                if board[y][x] == BoardCell::Absent {
                    board[y][x] = BoardCell::Adversary;

                    best = cmp::max(best, minimax(board, depth + 1, !is_maximizing));

                    board[y][x] = BoardCell::Absent;
                }
            }
        }

        return best;
    } else {
        let mut best = -1000;

        for y in 0..3 {
            for x in 0..3 {
                if board[y][x] == BoardCell::Absent {
                    board[y][x] = BoardCell::Ally;

                    best = cmp::min(best, minimax(board, depth + 1, !is_maximizing));

                    board[y][x] = BoardCell::Absent;
                }
            }
        }

        return best;
    }
}

fn find_best_move(board: &mut Board, is_maximizing: bool) -> Move {
    let mut best_val = if is_maximizing {
        -1000
    } else {
        1000
    };

    let mut best_x = 3;
    let mut best_y = 3;

    for y in 0..3 {
        for x in 0..3 {
            if board[y][x] == BoardCell::Absent {
                board[y][x] = if is_maximizing {
                    BoardCell::Adversary
                } else {
                    BoardCell::Ally
                };

                let move_val = minimax(board, 0, is_maximizing);

                board[y][x] = BoardCell::Absent;

                if is_maximizing {
                    if move_val > best_val {
                        best_x = x;
                        best_y = y;
                        best_val = move_val;
                    }
                } else {
                    if move_val < best_val {
                        best_x = x;
                        best_y = y;
                        best_val = move_val;
                    }
                }
            }
        }
    }

    return Move(best_x, best_y);
}

fn rotate(board: &mut Board) -> () {
    for x in 0..3 / 2 {
        for y in 0..3 - x - 1 {
            let temp = board[x][y];

            board[x][y] = board[y][3 - 1 - x];

            board[y][3 - 1 - x] = board[3 - 1 - x][3 - 1 - y];

            board[3 - 1 - x][3 - 1 - y] = board[3 - 1 - y][x];

            board[3 - 1 - y][x] = temp;
        }
    }
}

fn check_winner(board: &mut Board) -> BoardCell {
    for c in vec![BoardCell::Ally, BoardCell::Adversary].into_iter() {
        if board[0] == [c, c, c] || board[1] == [c, c, c] || board[2] == [c, c, c] {
            return c;
        }

        rotate(board);

        if board[0] == [c, c, c] || board[1] == [c, c, c] || board[2] == [c, c, c] {
            return c;
        }

        rotate(board);
        rotate(board);
        rotate(board);

        if board[0][0] == c && board[1][1] == c && board[2][2] == c {
            return c;
        }

        if board[0][2] == c && board[1][1] == c && board[2][0] == c {
            return c;
        }
    }

    return BoardCell::Absent;
}

fn print_board(board: &Board) -> () {
    let mut out = "".to_string();

    for y in 0..3 {
        let mut row = "".to_string();

        for x in 0..3 {
            row = if row.len() == 0 {
                format!("{}", cell_to_symbol(board[y][x]))
            } else {
                format!("{} {}", row, cell_to_symbol(board[y][x]))
            }
        }

        out = if out.len() == 0 {
            format!("{}", row)
        } else {
            format!("{}\n{}", out, row)
        }
    }

    println!("{}", out);
}

fn cell_to_symbol(cell: BoardCell) -> String {
    if cell == BoardCell::Adversary {
        return "x".into();
    }
    
    if cell == BoardCell::Ally {
        return "o".into();
    } 

    return " ".into();
}

fn from_symbols(board: [[char; 3]; 3]) -> Board {
    return [
        [from_symbol(board[0][0]), from_symbol(board[0][1]), from_symbol(board[0][2])],
        [from_symbol(board[1][0]), from_symbol(board[1][1]), from_symbol(board[1][2])],
        [from_symbol(board[2][0]), from_symbol(board[2][1]), from_symbol(board[2][2])],
    ];
}

fn from_symbol(c: char) -> BoardCell {
    if c == 'x' {
        return BoardCell::Adversary;
    }

    if c == 'o' {
        return BoardCell::Ally;
    }

    return BoardCell::Absent;
}

fn main() {
    let mut board = from_symbols([
        [' ', ' ', ' '],
        [' ', ' ', ' '],
        [' ', ' ', 'x'],
    ]);

    let mut is_maximizing = false;

    loop {
        println!("-----");

        print_board(&board);

        if !are_moves_left(&board) {
            println!("");

            println!("Draw");

            break;
        }

        let Move(x, y) = find_best_move(&mut board, is_maximizing);

        board[y][x] = if is_maximizing {
            BoardCell::Adversary
        } else {
            BoardCell::Ally
        };

        is_maximizing = !is_maximizing;

        let winner = check_winner(&mut board);

        if winner != BoardCell::Absent {
            println!("=====");
    
            print_board(&board);

            println!("");
    
            println!("Winner is {:?}", winner);

            break;
        }
    }
}
