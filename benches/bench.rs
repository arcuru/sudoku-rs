#![feature(test)]

extern crate sudoku_rs;
use sudoku_rs::SudokuBoard;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

extern crate test;
use test::Bencher;

fn bench_file(b: &mut Bencher, file: &str, full: bool) {
    let v: Vec<SudokuBoard> = {
        let f = File::open(file).unwrap();
        let mut ret: Vec<SudokuBoard> = Vec::new();

        for line in BufReader::new(&f).lines() {
            ret.push(SudokuBoard::from_string(&line.unwrap().chars().take(81).collect::<String>()).unwrap());
        }
        ret
    };
    if full {
        b.iter(|| for x in &v { x.get_solution().unwrap(); } );
    } else {
        b.iter(|| v[0].get_solution().unwrap() );
    }
}

#[bench]
fn all_euler(b: &mut Bencher) {
    bench_file(b, "data/96.txt", true);
}

#[bench]
fn one_hard(b: &mut Bencher) {
    bench_file(b, "data/hard.txt", false);
}

#[bench]
fn one_17(b: &mut Bencher) {
    bench_file(b, "data/sudoku17.txt", false);
}

#[bench]
fn one_euler(b: &mut Bencher) {
    bench_file(b, "data/96.txt", false);
}

#[bench]
fn inkala_2006(b: &mut Bencher) {
    let s = SudokuBoard::from_string("85...24..72......9..4.........1.7..23.5...9...4...........8..7..17..........36.4.").unwrap();
    b.iter(|| s.get_solution().unwrap());
}

#[bench]
fn inkala_2010(b: &mut Bencher) {
    let s = SudokuBoard::from_string("..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..").unwrap();
    b.iter(|| s.get_solution().unwrap());
}

// See: https://attractivechaos.wordpress.com/2011/06/19/an-incomplete-review-of-sudoku-solver-implementations/
#[bench]
fn veryhard(b: &mut Bencher) {
    bench_file(b, "data/veryhard.txt", true);
}
