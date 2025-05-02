use std::env;
use std::time::Instant;
use crate::small_board::SmallBoard;
use crate::big_board::BigBoard;

pub mod small_board;
pub mod big_board;

// Board size # of pegs
//     5      15
//     6      21
//     7      28
//     8      36 (does no longer fit in u32)
//     9      45
//
//       0 <- column  row
//      x 1           0      0
//     x x 2          1     1 2
//    x o x 3         2    3 4 5
//   x x x x 4        3   6 7 8 9
//  x x x x x         4  0 1 2 3 4
//

fn main() {
    let args: Vec<String> = env::args().collect();

    let size: i32 = if args.len() < 2 {
        7
    } else {
        match args[1].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("{} ist keine gültige Zahl.", args[1]);
                return;
            }
        }
    };

    let start = Instant::now();

    let game = Game::new(size);
    let cache_entries: usize = 1 << game.positions;
    let mut cache_vec: Vec<i64> = Vec::with_capacity(cache_entries);
    for i in 0..cache_entries {
        cache_vec.push(-1);
    }
    let cache = cache_vec.as_mut_slice();
    
    let board = game.start_board();
    let count = count_solutions(cache, board);

    let elapsed = start.elapsed();
    println!("Time: {} ms", elapsed.as_millis());
    println!("Number of solutions: {}", count);
}


fn count_solutions(mut cache: &mut[i64], board: Box<dyn Board>) -> i64 {
    let index = board.as_usize();
    if cache[index] != -1 {
        cache[index]
    } else if board.is_solution() {
        cache[index] = 1;
        1
    } else {
        let mut count: i64 = 0;
        let next_boards = board.next_boards();
        for board in next_boards {
            count += count_solutions(cache, board)
        }
        cache[index] = count;
        count
    }
}

trait Board {

    // Is field occupied by a peg? Row and column start at 0.
    fn is_occupied(&self, row: i32, column: i32) -> bool;

    // Is this a solution (means: Exactly one peg on the board).
    fn is_solution(&self) -> bool;

    // Represention with pegs as usize.
    fn as_usize(&self) -> usize;

    // Length of the edges / number of rows
    fn size(&self) -> i32;

    fn set_peg(&mut self, row: i32, col: i32);

    fn remove_peg(&mut self, row: i32, col: i32);

    fn copy(&self) -> Box<dyn Board>;

    fn position(&self, row: i32, col: i32) -> i32 {
        row * (row + 1) / 2 + col
    }

    fn start_position(&mut self) {
        for row in 0..self.size() {
            for col in 0..row + 1 {
                self.set_peg(row, col)
            }
        }
        self.remove_peg(2, 1)
    }

    fn next_boards(&self) -> Vec<Box<dyn Board>> {
        let mut boards: Vec<Box<dyn Board>> = Vec::new();
        for row in 0..self.size() {
            for col in 0..row + 1 {
                if self.is_occupied(row, col) {
                    self.add_move(&mut boards, row, col, 0, 1); // right
                    self.add_move(&mut boards, row, col, 0, -1); // left
                    self.add_move(&mut boards, row, col, -1, 0); // up + right
                    self.add_move(&mut boards, row, col, -1, -1); // up + left
                    self.add_move(&mut boards, row, col, 1, 1); // down + right
                    self.add_move(&mut boards, row, col, 1, 0); // down + left
                }
            }
        }
        boards
    }

    fn add_move(&self, boards: &mut Vec<Box<dyn Board>>, row: i32, col: i32, y: i32, x: i32) {
        // The "jump to field" must be on the board
        if (self.is_valid(row + 2 * y, col + 2 * x)) {
            // There must be a peg on the "jump over field"
            if (self.is_occupied(row + y, col + x)) {
                // The "jump to field" must be empty
                if (!self.is_occupied(row + 2 * y, col + 2 * x)) {

                    let mut next = self.copy();
                    next.remove_peg(row, col);
                    next.remove_peg(row + y, col + x);
                    next.set_peg(row + 2 * y, col + 2 * x);
                    boards.push(next);
                }
            }
        }        
    }

    fn is_valid(&self, row: i32, col: i32) -> bool {
        let s: i32 = self.size().try_into().unwrap();
        if row < 0 || row >= s {
            false
        } else if col < 0 || col > row {
            false
        } else {
            true
        }
    }

    fn to_string(&self) -> String {
        let mut buffer = String::new();
        for row in 0..self.size() {
            (0..self.size() - row - 1).for_each(|_| {
                buffer.push(' ');
            });
            for col in 0..row + 1 {
                buffer.push(
                    if self.is_occupied(row, col) {
                        'x'
                    } else {
                        'o'
                    }
                );
                if col < row {
                    buffer.push(' ');
                }
            }
            buffer.push('\n');
        }
        buffer
    }
}


struct Game {
    size: i32,
    positions: i32,
}

impl Game {

    fn new(size: i32) -> Game {
        Game { size, positions: size * (size + 1) / 2 }
    }

    fn start_board(self) -> Box<dyn Board> {
        if self.positions < 32 {
            Box::new(SmallBoard::new(self.size))
        } else if self.positions < 32 {
            Box::new(BigBoard::new(self.size))
        } else {
            panic!("too big")
        }
     }
}