use crate::day::Part;
use std::fs;

pub fn run(part: Part, file_path: &str) {
    match part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}

fn part1(file_path: &str) {
    let data = fs::read_to_string(file_path).expect("Read the input file");

    println!("First marker at: {}", find_first_marker(&data).unwrap());
}

fn find_first_marker(data: &str) -> Option<usize> {
    let mut vec: Vec<char> = vec![];
    for c in data.chars() {
        vec.push(c);
        if last_4_is_marker(&vec) {
            return Some(vec.len());
        }
    }
    None
}

fn last_4_is_marker(vec: &Vec<char>) -> bool {
    let len = vec.len();
    if len < 4 {
        return false;
    }
    let mut unique: Vec<char> = vec![];
    for i in len-4..len {
        let v = vec[i];
        if unique.contains(&v) {
            return false;
        }
        unique.push(v);
    }
    true
}

fn part2(file_path: &str) {
    let _data = fs::read_to_string(file_path).expect("Read the input file");
}

#[cfg(test)]
mod tests {
    use crate::day06::find_first_marker;

    #[test]
    fn test_find_first_marker() {
        assert_eq!(find_first_marker(""), None);
        assert_eq!(find_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(find_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(find_first_marker("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(find_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(find_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }
}
