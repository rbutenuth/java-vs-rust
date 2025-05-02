use std::fmt;

pub struct SmallBoard {
    size: i32,
    pegs: u32,
}

impl fmt::Display for SmallBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.size() {
            for _ in 0..self.size() - row - 1 {
                write!(f, " ")?;
            };
            for col in 0..row + 1 {
                if self.is_occupied(row, col) { 
                    write!(f, "x")?;
                } else { 
                    write!(f, "o")?;
                };
                if col < row {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl SmallBoard {
    pub fn new(size: i32) -> SmallBoard {
        let mut b = SmallBoard { size, pegs: 0 };
        b.start_position();
        b
    }
    pub fn position(&self, row: i32, col: i32) -> i32 {
        row * (row + 1) / 2 + col
    }

    pub fn start_position(&mut self) {
        for row in 0..self.size() {
            for col in 0..row + 1 {
                self.set_peg(row, col)
            }
        }
        self.remove_peg(2, 1)
    }

    pub fn next_boards(&self) -> Vec<SmallBoard> {
        let mut boards: Vec<SmallBoard> = Vec::new();
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

    pub fn add_move(&self, boards: &mut Vec<SmallBoard>, row: i32, col: i32, y: i32, x: i32) {
        // The "jump to field" must be on the board
        if self.is_valid(row + 2 * y, col + 2 * x) {
            // There must be a peg on the "jump over field"
            if self.is_occupied(row + y, col + x) {
                // The "jump to field" must be empty
                if !self.is_occupied(row + 2 * y, col + 2 * x) {
                    let mut next = self.copy();
                    next.remove_peg(row, col);
                    next.remove_peg(row + y, col + x);
                    next.set_peg(row + 2 * y, col + 2 * x);
                    boards.push(next);
                }
            }
        }
    }

    pub fn is_valid(&self, row: i32, col: i32) -> bool {
        let s: i32 = self.size().try_into().unwrap();
        if row < 0 || row >= s {
            false
        } else if col < 0 || col > row {
            false
        } else {
            true
        }
    }

    pub fn is_occupied(&self, row: i32, col: i32) -> bool {
        (self.pegs & (1 << self.position(row, col))) != 0
    }

    // Is this a solution (means: Exactly one peg on the board).
    pub fn is_solution(&self) -> bool {
        self.pegs.count_ones() == 1
    }

    // Represention with pegs as bits
    pub fn as_usize(&self) -> usize {
        self.pegs as usize
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    // Number of pegs on the board.
    pub fn cardinality(&self) -> i32 {
        self.pegs.count_ones() as i32
    }

    pub fn set_peg(&mut self, row: i32, col: i32) {
        self.pegs |= 1 << self.position(row, col)
    }

    pub fn remove_peg(&mut self, row: i32, col: i32) {
        self.pegs &= !(1 << self.position(row, col));
    }

    pub fn copy(&self) -> SmallBoard {
        SmallBoard {
            size: self.size,
            pegs: self.pegs,
        }
    }
}
