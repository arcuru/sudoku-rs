#![feature(test)]
use std::fmt;

mod solver;

#[derive(Clone)]
pub struct SudokuBoard {
    /// Temporary variable for holding the board info
    v: [u8; 81],
}

impl SudokuBoard {
    /// Create empty Sudoku Board with no set values
    pub fn new() -> SudokuBoard {
        SudokuBoard { v: [0; 81] }
    }

    /// Creates a new Sudoku Board from a string
    pub fn from_string(s: &str) -> Result<SudokuBoard, ()> {
        if s.len() != 81 {
            return Err(());
        }
        let mut ret = SudokuBoard::new();
        for (i, c) in s.chars().enumerate() {
            // Default to 0 for every character other than 1-9
            ret.v[i] = c.to_digit(10).unwrap_or(0) as u8;
        }
        Ok(ret)
    }

    /// Creates a board from a SudokuSolver object
    /// Always succeeeds
    fn from_solver(s: solver::SudokuSolver) -> SudokuBoard {
        let mut ret = SudokuBoard::new();
        for i in 0..81 {
            ret.v[i] = s.val(i) as u8;
        }
        ret
    }

    /// Assigns a value to a position on the board. Doesn't perform any checks.
    pub fn assign(&mut self, idx: usize, val: u8) {
        // May want to actually check that 'val' is valid
        self.v[idx] = val;
    }

    /// Returns the value at a given position on the board.
    pub fn val(&self, idx: usize) -> u8 {
        self.v[idx]
    }

    /// Solves the puzzle if possible and returns the first solution found.
    pub fn get_solution(&self) -> Result<SudokuBoard, ()> {
        use solver::SudokuSolver;
        let mut s = SudokuSolver::new(&self.v)?;
        if !s.solve() {
            // Couldn't find a solution
            return Err(());
        }
        Ok(SudokuBoard::from_solver(s))
    }

    /// Gets all possible solutions
    pub fn get_all_solutions(&self) -> Result<Vec<SudokuBoard>, ()> {
        use solver::SudokuSolver;
        let v = SudokuSolver::new(&self.v)?.get_all_solutions();

        Ok(v.into_iter().map(|s| SudokuBoard::from_solver(s)).collect())
    }

    /// Checks whether this board is fully solved
    pub fn is_solved(&self) -> bool {
        // Check if everything is 1-9
        if self.v.iter().any(|&x| (x < 1) || (x > 9)) {
            return false;
        }

        // Little helper function (rust closure)
        let val = |x: usize, y: usize| self.v[(x * 9) + y];

        // Rows and columns
        for i in 0..9 {
            let (mut a, mut b) = (0, 0);
            for j in 0..9 {
                a |= 1 << val(i, j);
                b |= 1 << val(j, i);
            }
            if (a != 0x3FE) || (b != 0x3FE) {
                return false;
            }
        }
        // Boxes
        for i in 0..3 {
            for j in 0..3 {
                let mut a = 0;
                for k in (i * 3)..(i * 3 + 3) {
                    for l in (j * 3)..(j * 3 + 3) {
                        a |= 1 << val(k, l);
                    }
                }
                if a != 0x3FE {
                    return false;
                }
            }
        }
        true
    }
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Little helper function (rust closure)
        let val = |x: usize, y: usize| self.v[(x * 9) + y];

        for i in 0..9 {
            for j in 0..9 {
                write!(f, "{} ", val(i, j))?;
                if (j == 2) || (j == 5) {
                    write!(f, "| ")?;
                }
            }
            writeln!(f, "")?;
            if (i == 2) || (i == 5) {
                writeln!(f, "---------------------")?;
            }
        }
        write!(f, "")
    }
}
