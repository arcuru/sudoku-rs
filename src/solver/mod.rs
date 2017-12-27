mod cell;
use solver::cell::Cell;

static GROUPS: [[usize; 9]; 27] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    [0, 9, 18, 27, 36, 45, 54, 63, 72],
    [9, 10, 11, 12, 13, 14, 15, 16, 17],
    [1, 10, 19, 28, 37, 46, 55, 64, 73],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [2, 11, 20, 29, 38, 47, 56, 65, 74],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    [3, 12, 21, 30, 39, 48, 57, 66, 75],
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    [4, 13, 22, 31, 40, 49, 58, 67, 76],
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
    [5, 14, 23, 32, 41, 50, 59, 68, 77],
    [54, 55, 56, 57, 58, 59, 60, 61, 62],
    [6, 15, 24, 33, 42, 51, 60, 69, 78],
    [63, 64, 65, 66, 67, 68, 69, 70, 71],
    [7, 16, 25, 34, 43, 52, 61, 70, 79],
    [72, 73, 74, 75, 76, 77, 78, 79, 80],
    [8, 17, 26, 35, 44, 53, 62, 71, 80],
    [0, 1, 2, 9, 10, 11, 18, 19, 20],
    [3, 4, 5, 12, 13, 14, 21, 22, 23],
    [6, 7, 8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

static BELONGS_TO: [[usize; 3]; 81] = [
    [0, 1, 18],
    [0, 3, 18],
    [0, 5, 18],
    [0, 7, 19],
    [0, 9, 19],
    [0, 11, 19],
    [0, 13, 20],
    [0, 15, 20],
    [0, 17, 20],
    [1, 2, 18],
    [2, 3, 18],
    [2, 5, 18],
    [2, 7, 19],
    [2, 9, 19],
    [2, 11, 19],
    [2, 13, 20],
    [2, 15, 20],
    [2, 17, 20],
    [1, 4, 18],
    [3, 4, 18],
    [4, 5, 18],
    [4, 7, 19],
    [4, 9, 19],
    [4, 11, 19],
    [4, 13, 20],
    [4, 15, 20],
    [4, 17, 20],
    [1, 6, 21],
    [3, 6, 21],
    [5, 6, 21],
    [6, 7, 22],
    [6, 9, 22],
    [6, 11, 22],
    [6, 13, 23],
    [6, 15, 23],
    [6, 17, 23],
    [1, 8, 21],
    [3, 8, 21],
    [5, 8, 21],
    [7, 8, 22],
    [8, 9, 22],
    [8, 11, 22],
    [8, 13, 23],
    [8, 15, 23],
    [8, 17, 23],
    [1, 10, 21],
    [3, 10, 21],
    [5, 10, 21],
    [7, 10, 22],
    [9, 10, 22],
    [10, 11, 22],
    [10, 13, 23],
    [10, 15, 23],
    [10, 17, 23],
    [1, 12, 24],
    [3, 12, 24],
    [5, 12, 24],
    [7, 12, 25],
    [9, 12, 25],
    [11, 12, 25],
    [12, 13, 26],
    [12, 15, 26],
    [12, 17, 26],
    [1, 14, 24],
    [3, 14, 24],
    [5, 14, 24],
    [7, 14, 25],
    [9, 14, 25],
    [11, 14, 25],
    [13, 14, 26],
    [14, 15, 26],
    [14, 17, 26],
    [1, 16, 24],
    [3, 16, 24],
    [5, 16, 24],
    [7, 16, 25],
    [9, 16, 25],
    [11, 16, 25],
    [13, 16, 26],
    [15, 16, 26],
    [16, 17, 26],
];

/*
// this is how you would do it if you wanted global variables with dynamic init
lazy_static! {
    static ref GROUPS: Vec<Vec<usize>> = {
        let mut groups: Vec<Vec<usize>> = vec![];

        // Create all the different groups

        // Rows + Columns
        for r in 0..9 {
            let mut row: Vec<usize> = vec![];
            let mut col: Vec<usize> = vec![];
            for c in 0..9 {
                row.push(9 * r + c);
                col.push(9 * c + r);
            }
            groups.push(row);
            groups.push(col);
        }

        // Boxes
        for r in 0..3 {
            for c in 0..3 {
                let mut bx: Vec<usize> = vec![];
                for i in (r * 3)..((r + 1) * 3) {
                    for j in (c * 3)..((c + 1) * 3) {
                        bx.push(9 * i + j);
                    }
                }
                groups.push(bx);
            }
        }
        groups
    };

    static ref BELONGS_TO: Vec<Vec<usize>> = {
        let mut v: Vec<Vec<usize>> = vec![vec![]; 81];
        for (i, x) in GROUPS.iter().enumerate() {
            for &j in x {
                v[j].push(i);
            }
        }
        v
    };
}
*/

/// Represents a full Sudoku Board
#[derive(Clone)]
pub struct SudokuSolver {
    /// Holds the board info
    v: [Cell; 81],
    /// For each group in GROUPS, keeps track of the number of vals left to find
    ///   e.g. if group number 3 has 4 possible places left for val 5, then
    ///        group_counts[3][5] == 4
    group_counts: [[usize; 9]; 27], //Vec<Vec<u8>>,
    removed: usize,
}

impl SudokuSolver {
    /// Creates a new solver from a Sudoku Board
    pub fn new(board: &[u8; 81]) -> Result<SudokuSolver, ()> {
        // Print groups
        let mut ret = SudokuSolver {
            v: [Cell::new(); 81],
            group_counts: [[9; 9]; 27],
            removed: 0,
        };
        for (i, x) in board.iter().enumerate() {
            if *x != 0 {
                if !ret.assign(i, *x - 1) {
                    return Err(());
                }
            }
        }
        Ok(ret)
    }

    /// Assigns an element into the board
    /// Returns false if it's an illegal move
    pub fn assign(&mut self, idx: usize, val: u8) -> bool {
        // Val is zero indexed
        assert!(self.v[idx].contains(val));

        if self.v[idx].is_lonely() {
            return true;
        }

        for x in 0..9 {
            if x != val {
                if !self.remove(idx, x) {
                    return false;
                }
            }
        }
        true
    }

    // Removes the value from the given cell
    fn remove(&mut self, idx: usize, val: u8) -> bool {
        // Do nothing if it's not here
        if !self.v[idx].contains(val) {
            return true;
        }

        // Fails if we're removing the last element
        if self.v[idx].is_lonely() {
            // Need to actually remove it here
            self.v[idx].remove(val);
            return false;
        }

        self.v[idx].remove(val);
        self.removed += 1;

        if self.v[idx].is_lonely() {
            // This cell now has 1 value
            // Loop through every neighbor and remove that val

            let rem = self.v[idx].val() as u8;

            for n in &BELONGS_TO[idx] {
                for x in &GROUPS[*n] {
                    if *x != idx {
                        // Skip this cell
                        if !self.remove(*x, rem) {
                            // propagate failure
                            return false;
                        }
                    }
                }
            }
        }

        // For every group that this cell is a part of, loop through each cell.
        // Checking to see if 'val' is unique in any group. If it is, assign it.
        for n in &BELONGS_TO[idx] {
            self.group_counts[*n][val as usize] -= 1;
            if self.group_counts[*n][val as usize] == 1 {
                // 1 left, find and assign it
                for x in &GROUPS[*n] {
                    if self.v[*x].contains(val) {
                        // Found it
                        if !self.assign(*x, val) {
                            // Assign
                            return false; // Propagate the failue
                        }
                        break; // Only one in group
                    }
                }
            }
        }

        true
    }

    /// Finds smallest idx that isn't solved
    fn smallest(&self) -> usize {
        let mut min = 10;
        let mut mini = 81;
        for (i, x) in self.v.iter().enumerate() {
            if x.is_lonely() {
                continue;
            }
            if x.len() < min {
                min = x.len();
                mini = i;
            }
        }
        mini
    }

    // Returns the value of a place on the board
    pub fn val(&self, idx: usize) -> usize {
        self.v[idx].val() + 1
    }

    /// Solves board with guessing
    pub fn solve(&mut self) -> bool {
        if self.is_solved() {
            return true;
        }

        // Guess starting with the place with the least guesses
        let idx = self.smallest();

        for x in 0..9 {
            if self.v[idx].contains(x) {
                // Create copy and guess
                let mut cpy = self.clone();
                if cpy.assign(idx, x) {
                    if cpy.solve() {
                        *self = cpy;
                        return true;
                    }
                }
            }
        }
        false
    }

    fn solve_all(&self, v: &mut Vec<SudokuSolver>) {
        if self.is_solved() {
            v.push(self.clone());
            return;
        }

        // Guess at the smallest place
        let idx = self.smallest();

        for x in 0..9 {
            if self.v[idx].contains(x) {
                // Create copy and guess
                let mut cpy = self.clone();
                if !cpy.assign(idx, x) {
                    continue;
                }
                cpy.solve_all(v)
            }
        }
    }

    pub fn get_all_solutions(&self) -> Vec<SudokuSolver> {
        let mut ret: Vec<SudokuSolver> = Vec::new();
        self.solve_all(&mut ret);
        ret
    }

    // Checks if board is solved
    pub fn is_solved(&self) -> bool {
        // The board performs checks on input, so as long as there's only one
        //   option in each cell left we're solved
        self.removed == (81 * 8)
    }
}
