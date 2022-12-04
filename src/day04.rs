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

    let mut lines_count = 0;
    let mut contain_count = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
            lines_count += 1;

        let (a, b) = try_parse_2_ranges(&line).expect("Failed to parse line");
        let any_contains = a.contains(&b) || b.contains(&a);

        if any_contains {
            contain_count += 1;
        }
    }

    println!("Lines count: {}", lines_count);
    println!("Count where either contains the other: {}", contain_count);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Range {
    from: i32,
    to: i32,
}

impl Range {
    fn new(from: i32, to: i32) -> Self {
        Self { from, to }
    }

    fn try_parse(s: &str) -> Option<Range> {
        let (from_s, to_s) = s.split_once('-')?;
        let from_i = from_s.parse::<i32>().expect("Failed to parse 'from'");
        let to_i = to_s.parse::<i32>().expect("Failed to parse 'to'");
        return Some(Self::new(from_i, to_i));
    }

    fn contains(&self, other: &Range) -> bool {
        return self.from <= other.from && self.to >=other.to;
    }
}

fn try_parse_2_ranges(s: &str) -> Option<(Range, Range)> {
    let (first, second) = s.split_once(',')?;
    let first_range = Range::try_parse(first)?;
    let second_range = Range::try_parse(second)?;
    return Some((first_range, second_range));
}

fn part2(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let reader = io::BufReader::new(file);

    for _line in reader.lines().map(|l| l.unwrap()) {}
}

#[cfg(test)]
mod tests {
    use crate::day04::{try_parse_2_ranges, Range};

    #[test]
    fn test_range_parse() {
        assert_eq!(Range::try_parse("1-2"), Some(Range::new(1, 2)));
        assert_eq!(Range::try_parse("15-20"), Some(Range::new(15, 20)));
    }

    #[test]
    fn test_parse_2_ranges() {
        assert_eq!(
            try_parse_2_ranges("1-2,3-4"),
            Some((Range::new(1, 2), Range::new(3, 4)))
        );
    }
}
