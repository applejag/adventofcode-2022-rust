use crate::day::Part;
use std::fs;
use std::io;

pub fn run(part: Part, file_path: &str) {
    match part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}

fn part1(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let _reader = io::BufReader::new(file);
}

fn part2(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let _reader = io::BufReader::new(file);
}
