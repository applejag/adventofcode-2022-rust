use crate::day::Part;
use std::fs;
use std::io::{self, prelude::*};

pub fn run(part: Part, file_path: &str) {
    match part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}

fn part1(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut lines = 0;
    for _line in reader.lines().map(|l| l.unwrap()) {
        lines += 1;
    }

    println!("Lines: {}", lines);
}

fn part2(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut lines = 0;
    for _line in reader.lines().map(|l| l.unwrap()) {
        lines += 1;
    }

    println!("Lines: {}", lines);
}
