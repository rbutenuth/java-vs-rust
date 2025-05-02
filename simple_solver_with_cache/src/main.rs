use std::env;
use std::time::Instant;
use crate::small_board::SmallBoard;

pub mod small_board;

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
        5
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

    println!("Start: \n{}", board);

    let count = count_solutions(cache, board);

    let elapsed = start.elapsed();
    println!("Time: {} ms", elapsed.as_millis());
    println!("Number of solutions: {}", count);
}


fn count_solutions(cache: &mut[i64], board: SmallBoard) -> i64 {
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

struct Game {
    size: i32,
    positions: i32,
}

impl Game {

    fn new(size: i32) -> Game {
        Game { size, positions: size * (size + 1) / 2 }
    }

    fn start_board(self) -> SmallBoard {
        if self.positions < 32 {
            SmallBoard::new(self.size)
        } else {
            panic!("too big")
        }
     }
}