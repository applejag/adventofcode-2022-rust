use std::fs;
use std::io::{self, prelude::*};

pub fn part1() {
    let file_path = "inputs/day01.txt";
    println!("In file {}", file_path);

    let file = fs::File::open(file_path)
        .expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut lines = 0;
    let mut sum = 0;
    let mut biggest_sum = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        lines += 1;
        if line == "" {
            if sum > biggest_sum {
                biggest_sum = sum;
            }
            sum = 0;
            continue
        }
        let value: i32 = line.parse().unwrap();
        sum += value;
    }

    println!("Lines: {}", lines);
    println!("Biggest sum: {}", biggest_sum);
}

pub fn part2() {
    let file_path = "inputs/day01.txt";
    println!("In file {}", file_path);

    let file = fs::File::open(file_path)
        .expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut lines = 0;
    let mut sum = 0;
    let mut sums = vec![];

    for line in reader.lines().map(|l| l.unwrap()) {
        lines += 1;
        if line == "" {
            sums.push(sum);
            sum = 0;
            continue
        }
        let value: i32 = line.parse().unwrap();
        sum += value;
    }

    sums.sort_by(|a, b| b.cmp(a));

    println!("Lines: {}", lines);
    println!("0: {}", sums[0]);
    println!("1: {}", sums[1]);
    println!("2: {}", sums[2]);
    println!("Sum: {}",  sums[0] + sums[1] + sums[2]);
}
