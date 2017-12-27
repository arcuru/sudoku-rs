extern crate sudoku_rs;
use sudoku_rs::SudokuBoard;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

// Convert puzzle to value at top left corner
fn to_val(board: &SudokuBoard) -> usize {
    (100 * board.val(0) as usize) + (10 * board.val(1) as usize) + board.val(2) as usize
}

fn test_file(file: &str, val: usize) {
    println!("Reading in file {}", file);
    let v: Vec<SudokuBoard> = {
        let f = File::open(file).unwrap();
        let mut ret: Vec<SudokuBoard> = Vec::new();

        for line in BufReader::new(&f).lines() {
            ret.push(
                SudokuBoard::from_string(&line.unwrap().chars().take(81).collect::<String>())
                    .unwrap(),
            );
        }
        ret
    };
    println!("Solving {} boards...", v.len());
    assert_eq!(
        val,
        v.iter()
            .fold(0, |a, b| a + to_val(&b.get_solution().unwrap()))
    );
}

#[test]
fn euler_96() {
    test_file("data/96.txt", 24702);
}

#[test]
fn sudoku17() {
    test_file("data/sudoku17.txt", 27558167);
}

#[test]
fn hard() {
    test_file("data/hard.txt", 738217);
}

#[test]
fn veryhard() {
    test_file("data/veryhard.txt", 9458);
}
