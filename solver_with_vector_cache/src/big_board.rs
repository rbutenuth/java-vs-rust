use crate::Board;

pub struct BigBoard {
    size: i32,
    pegs: u64,
}

impl BigBoard {
    pub fn new(size: i32) -> BigBoard {
        BigBoard { size, pegs: 0 }
    }
}

impl Board for BigBoard {

    fn is_occupied(&self, row: i32, col: i32) -> bool {
        (self.pegs & (1 << self.position(row, col))) != 0
    }

    // Is this a solution (means: Exactly one peg on the board).
    fn is_solution(&self) -> bool {
        self.pegs.count_ones() == 1
    }

    // Represention with pegs as bits
    fn as_usize(&self) -> usize {
        self.pegs as usize
    }

    fn size(&self) -> i32 {
        self.size
    }
    
    fn set_peg(&mut self, row: i32, col: i32) {
        self.pegs |= 1 << self.position(row, col)
    }

    fn remove_peg(&mut self, row: i32, col: i32) {
        self.pegs &= !(1 << self.position(row, col));
    }
    
    fn copy(&self) -> Box<dyn Board> {
        Box::new(BigBoard { size: self.size, pegs: self.pegs })
    }
}
