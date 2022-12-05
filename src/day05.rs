use crate::day::Part;
use std::fs;
use std::io::{self, prelude::*};

pub fn run(part: Part, file_path: &str) {
    match part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}

fn get_start_stacks() -> [Box<Vec<char>>; 9] {
    // I'm not parsing this thing automatically, nuh uh:
    // [N]             [R]             [C]
    // [T] [J]         [S] [J]         [N]
    // [B] [Z]     [H] [M] [Z]         [D]
    // [S] [P]     [G] [L] [H] [Z]     [T]
    // [Q] [D]     [F] [D] [V] [L] [S] [M]
    // [H] [F] [V] [J] [C] [W] [P] [W] [L]
    // [G] [S] [H] [Z] [Z] [T] [F] [V] [H]
    // [R] [H] [Z] [M] [T] [M] [T] [Q] [W]
    //  1   2   3   4   5   6   7   8   9
    [
        Box::new(vec!['R', 'G', 'H', 'Q', 'S', 'B', 'T', 'N']),
        Box::new(vec!['H', 'S', 'F', 'D', 'P', 'Z', 'J']),
        Box::new(vec!['Z', 'H', 'V']),
        Box::new(vec!['M', 'Z', 'J', 'F', 'G', 'H']),
        Box::new(vec!['T', 'Z', 'C', 'D', 'L', 'M', 'S', 'R']),
        Box::new(vec!['M', 'T', 'W', 'V', 'H', 'Z', 'J']),
        Box::new(vec!['T', 'F', 'P', 'L', 'Z']),
        Box::new(vec!['Q', 'V', 'W', 'S']),
        Box::new(vec!['W', 'H', 'L', 'M', 'T', 'D', 'N', 'C']),
    ]
}

fn part1(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut lines_iter = reader.lines().map(|l| l.unwrap()).into_iter();

    // skip the header
    loop {
        match lines_iter.next() {
            Some(a) if a == "" => break,
            None => break,
            _ => (),
        }
    }

    let mut stacks = get_start_stacks();
    let move_regex = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    // move 3 from 9 to 7

    let mut lines_count = 0;
    for line in lines_iter {
        lines_count += 1;
        let captures = move_regex.captures(&line).expect("Didn't match");
        let count_match = captures.get(1).expect("No count");
        let from_match = captures.get(2).expect("No from");
        let to_match = captures.get(3).expect("No to");

        let count = count_match.as_str().parse::<i32>().expect("Parse count");
        let from = from_match.as_str().parse::<usize>().expect("Parse from");
        let to = to_match.as_str().parse::<usize>().expect("Parse to");

        let from_index = from - 1;
        let to_index = to - 1;

        for _ in 1..=count {
            let value = pop_from_index(&mut stacks, from_index);
            push_to_index(&mut stacks, to_index, value);
        }
    }

    println!("Lines count: {}", lines_count);
    print!("Code: ");
    for vec in stacks {
        let value = vec.last().unwrap();
        print!("{}", value);
    }
    println!();
}

fn pop_from_index(stacks: &mut [Box<Vec<char>>; 9], index: usize) -> char {
    let from_vec = stacks.get_mut(index).unwrap();
    return from_vec.pop().unwrap();
}

fn push_to_index(stacks: &mut [Box<Vec<char>>; 9], index: usize, c: char) {
    let from_vec = stacks.get_mut(index).unwrap();
    from_vec.push(c);
}

fn part2(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut lines_count = 0;
    for _line in reader.lines().map(|l| l.unwrap()) {
        lines_count += 1;
    }

    println!("Lines count: {}", lines_count);
}
