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

    println!(
        "Start of packet: {}",
        find_first_start_of_packet(&data).unwrap()
    );
}

fn find_first_start_of_packet(data: &str) -> Option<usize> {
    find_where_ends_with_n_distinct(data, 4)
}

fn find_where_ends_with_n_distinct(data: &str, n: usize) -> Option<usize> {
    let mut vec: Vec<char> = vec![];
    for c in data.chars() {
        vec.push(c);
        if last_n_are_distinct(&vec, n) {
            return Some(vec.len());
        }
    }
    None
}

fn last_n_are_distinct(vec: &Vec<char>, n: usize) -> bool {
    let len = vec.len();
    if len < n {
        return false;
    }
    let mut unique: Vec<char> = vec![];
    for i in len - n..len {
        let v = vec[i];
        if unique.contains(&v) {
            return false;
        }
        unique.push(v);
    }
    true
}

fn part2(file_path: &str) {
    let data = fs::read_to_string(file_path).expect("Read the input file");

    println!(
        "Start of message: {}",
        find_first_start_of_message(&data).unwrap()
    );
}

fn find_first_start_of_message(data: &str) -> Option<usize> {
    find_where_ends_with_n_distinct(data, 14)
}

#[cfg(test)]
mod tests {
    use crate::day06::{find_first_start_of_message, find_first_start_of_packet};

    #[test]
    fn test_find_first_start_of_packet() {
        assert_eq!(find_first_start_of_packet(""), None);
        assert_eq!(
            find_first_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(7)
        );
        assert_eq!(
            find_first_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(5)
        );
        assert_eq!(
            find_first_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(6)
        );
        assert_eq!(
            find_first_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(10)
        );
        assert_eq!(
            find_first_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(11)
        );
    }

    #[test]
    fn test_find_first_start_of_message() {
        assert_eq!(find_first_start_of_message(""), None);
        assert_eq!(
            find_first_start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(19)
        );
        assert_eq!(
            find_first_start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(23)
        );
        assert_eq!(
            find_first_start_of_message("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(23)
        );
        assert_eq!(
            find_first_start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(29)
        );
        assert_eq!(
            find_first_start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(26)
        );
    }
}
