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

    let mut sum = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        let (a, b) = split_compartments(&line);
        let common = find_common(a, b).expect("No common letter :(");
        let score = letter_score(common);
        sum += score;
    }

    println!("Prio score sum: {}", sum);
}

fn split_compartments(s: &str) -> (&str, &str) {
    let a = &s[0..s.len()/2];
    let b = &s[s.len()/2..];
    return (a, b);
}

fn find_common(a: &str, b: &str) -> Option<char> {
    for a_char in a.chars() {
        for b_char in b.chars() {
            if a_char == b_char {
                return Some(a_char);
            }
        }
    }
    return None;
}

fn letter_score(c: char) -> u32 {
    match c {
        c if c >= 'a' && c <= 'z' => c as u32 - 'a' as u32 + 1,
        c if c >= 'A' && c <= 'Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("Bad letter"),
    }
}

fn part2(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let reader = io::BufReader::new(file);

    for _line in reader.lines().map(|l| l.unwrap()) {
    }
}

#[cfg(test)]
mod tests {
    use super::letter_score;

    #[test]
    fn test_letter_score() {
        assert_eq!(letter_score('a'), 1);
        assert_eq!(letter_score('z'), 26);
        assert_eq!(letter_score('A'), 27);
        assert_eq!(letter_score('Z'), 52);
    }
}
