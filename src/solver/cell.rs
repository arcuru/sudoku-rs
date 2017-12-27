#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    bs: u16,
}

impl Cell {
    /// Create an empty cell
    pub fn new() -> Cell {
        Cell { bs: 0x1FF }
    }

    pub fn len(&self) -> usize {
        self.bs.count_ones() as usize
    }

    pub fn is_lonely(&self) -> bool {
        self.bs.is_power_of_two()
    }

    pub fn contains(&self, x: u8) -> bool {
        0 != self.bs & ((1 as u16) << x)
    }

    pub fn remove(&mut self, x: u8) {
        // Note, bit twiddling fails if it's not set
        self.bs ^= (1 as u16) << x;
    }

    pub fn val(&self) -> usize {
        self.bs.trailing_zeros() as usize
    }
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use solver::cell::test::Bencher;

    #[test]
    fn creation() {
        assert_eq!(Cell::new().bs, 0x1FFu16);
    }

    #[bench]
    fn cell_len(b: &mut Bencher) {
        b.iter(|| {
            let mut s: usize = 0;
            for _ in 0..test::black_box(1000) {
                s += Cell::new().len();
            }
            s
        });
    }
}
