use std::env;
use std::time::Instant;

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
    for _i in 0..cache_entries {
        cache_vec.push(-1);
    }
    let cache = cache_vec.as_mut_slice();
    
    let board = game.start_board();
    let count = count_solutions(cache, board, game.size);
    let elapsed = start.elapsed();
    println!("Time: {} ms", elapsed.as_millis());
    println!("Number of solutions: {}", count);
}

fn count_solutions(cache: &mut[i64], board: u32, size: i32) -> i64 {
    let index = board as usize;
    if cache[index] != -1 {
        cache[index]
    } else if is_solution(board) {
        cache[index] = 1;
        1
    } else {
        let mut count: i64 = 0;
        let mut next_boards: [u32; 32] = [0; 32];
        let next_count = compute_next_boards(&mut next_boards, board, size);
        for i in 0..next_count {
            count += count_solutions(cache, next_boards[i], size);
        }
        cache[index] = count;
        count
    }
}

// Board size # of pegs
//     5      15
//     6      21
//     7      28
//     8      36 (does no longer fit in u32)
//     9      45
//
// Implementation using an <code>int</code> for the bits. A board of size 5 has 15 pegs:
//
//       0 <- column  row
//      x 1           0      0
//     x x 2          1     1 2
//    x o x 3         2    3 4 5
//   x x x x 4        3   6 7 8 9
//  x x x x x         4  0 1 2 3 4
//

pub fn position(row: i32, col: i32) -> i32 {
    row * (row + 1) / 2 + col
}


pub fn compute_next_boards(mut boards: &mut [u32], pegs: u32, size: i32) -> usize {
    let mut index: usize = 0;
    for row in 0..size as i32 {
        for col in 0..row + 1 {
            if is_occupied(pegs, row, col) {
                add_move(&mut boards, &mut index, pegs, size, row, col, 0, 1); // right
                add_move(&mut boards, &mut index, pegs, size, row, col, 0, -1); // left
                add_move(&mut boards, &mut index, pegs, size, row, col, -1, 0); // up + right
                add_move(&mut boards, &mut index, pegs, size, row, col, -1, -1); // up + left
                add_move(&mut boards, &mut index, pegs, size, row, col, 1, 1); // down + right
                add_move(&mut boards, &mut index, pegs, size, row, col, 1, 0); // down + left
            }
        }
    }
    index
}

pub fn add_move(boards: &mut [u32], index: &mut usize, pegs: u32, size: i32, row: i32, col: i32, y: i32, x: i32) {
    // The "jump to field" must be on the board
    if is_valid(size, row + 2 * y, col + 2 * x) {
        // There must be a peg on the "jump over field"
        if is_occupied(pegs, row + y, col + x) {
            // The "jump to field" must be empty
            if !is_occupied(pegs, row + 2 * y, col + 2 * x) {
                let mut next = pegs;
                remove_peg(&mut next, row, col);
                remove_peg(&mut next, row + y, col + x);
                set_peg(&mut next, row + 2 * y, col + 2 * x);
                boards[*index] = next;
                *index = *index + 1;
            }
        }
    }        
}

pub fn is_valid(size: i32, row: i32, col: i32) -> bool {
    let s: i32 = size as i32;
    if row < 0 || row >= s {
        false
    } else if col < 0 || col > row {
        false
    } else {
        true
    }
}

pub fn to_string(pegs: u32, size: i32) -> String {
    let mut buffer = String::new();
    for row in 0..size {
        (0..size - row - 1).for_each(|_| {
            buffer.push(' ');
        });
        for col in 0..row + 1 {
            buffer.push(
                if is_occupied(pegs, row as i32, col as i32) {
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

fn is_occupied(pegs: u32, row: i32, col: i32) -> bool {
    (pegs & (1 << position(row, col))) != 0
}

// Is this a solution (means: Exactly one peg on the board).
fn is_solution(pegs: u32) -> bool {
    pegs.count_ones() == 1
}

fn set_peg(pegs: &mut u32, row: i32, col: i32) {
    *pegs |= 1 << position(row, col)
}

fn remove_peg(pegs: &mut u32, row: i32, col: i32) {
    *pegs &= !(1 << position(row, col));
}


struct Game {
    size: i32,
    positions: i32,
}

impl Game {

    fn new(size: i32) -> Game {
        Game { size, positions: size * (size + 1) / 2 }
    }

    fn start_board(&self) -> u32 {
        let mut pegs: u32 = 0;
        if self.positions < 32 {
            for row in 0..self.size {
                for col in 0..row + 1 {
                    set_peg(&mut pegs, row, col)
                }
            }
            remove_peg(&mut pegs, 2, 1);
        } else {
            panic!("too big")
        }
        pegs
     }
}