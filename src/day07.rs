use crate::day::Part;
use std::fs;
use std::io::{self, prelude::*};
use std::slice::Iter;

pub fn run(part: Part, file_path: &str) {
    match part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}

#[derive(Debug, Clone)]
struct Command {
    exe: String,
    arg: Option<String>,
    output_lines: Vec<String>,
}

impl Command {
    fn new() -> Self {
        Self {
            exe: "".to_string(),
            arg: None,
            output_lines: vec![],
        }
    }
}

fn parse_commands(file_path: &str) -> Vec<Command> {
    let file = fs::File::open(file_path).expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut commands = vec![];
    let mut current = Command::new();
    let mut lines = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.starts_with("$ ") {
            if lines > 0 {
                commands.push(current.clone());
                current = Command::new();
            }
            let full_command = line.strip_prefix("$ ").unwrap();
            match full_command.split_once(' ') {
                Some((exe, arg)) => {
                    current.exe = exe.to_string();
                    current.arg = Some(arg.to_string());
                }
                None => {
                    current.exe = full_command.to_string();
                }
            }
        } else {
            current.output_lines.push(line);
        }
        lines += 1;
    }

    commands.push(current);

    println!("Lines: {}", lines);
    println!("Commands: {}", commands.len());
    commands
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    size: u64,
}

impl Dir {
    fn new(name: &str, size: u64) -> Self { Self { name: name.to_string(), size } }
}

fn calc_sizes_of_dirs(commands: Vec<Command>) -> Vec<Dir> {
    let mut iter = commands.iter();
    let mut all_dirs = vec![];
    calc_sizes_of_dirs_rec(&mut iter, &mut all_dirs, "/");
    return all_dirs;
}

fn calc_sizes_of_dirs_rec(iter: &mut Iter<Command>, all_dirs: &mut Vec<Dir>, name: &str) -> u64 {
    let mut sum = 0;
    loop {
        match iter.next() {
            Some(cmd) => match (cmd.exe.as_str(), cmd.arg.as_ref().map(|s| s.as_str())) {
                ("cd", Some("..")) => {
                    println!("{} $ cd ..", name);
                    all_dirs.push(Dir::new(name, sum));
                    return sum;
                }
                ("cd", Some(dir)) => {
                    let new_name = match (name, dir) {
                        ("/", "/") => "/".to_string(),
                        ("/", dir) => format!("/{}", dir),
                        (name, dir) => format!("{}/{}", name, dir),
                    };
                    println!("{} $ cd {}", name, dir);
                    sum += calc_sizes_of_dirs_rec(iter, all_dirs, &new_name);
                }
                ("ls", None) => {
                    for line in &cmd.output_lines {
                        if line.starts_with("dir ") {
                            continue;
                        }
                        match line.split_once(' ') {
                            Some((size_str, _)) => {
                                let size_num: u64 = size_str.parse().unwrap();
                                sum += size_num;
                            }
                            None => continue,
                        }
                    }
                }
                _ => panic!("Unknown command and argument: {} {:?}", cmd.exe, cmd.arg),
            },
            None => {
                all_dirs.push(Dir::new(name, sum));
                return sum;
            }
        };
    }
}

fn part1(file_path: &str) {
    let commands = parse_commands(file_path);

    let all_dirs = calc_sizes_of_dirs(commands);
    let mut sum = 0;
    for dir in all_dirs {
        if dir.size <= 100000 {
            println!("Dir is small enough: {}\t{}", dir.name, dir.size);
            sum += dir.size;
        }
    }
    println!("Sum of sizes: {}", sum);
}

fn part2(file_path: &str) {
    let commands = parse_commands(file_path);

    let all_dirs = calc_sizes_of_dirs(commands);

    let storage_total = 70000000;
    let storage_used = all_dirs.iter().find(|dir| dir.name == "/").unwrap().size;
    let storage_avail = storage_total - storage_used;
    let storage_avail_goal = 30000000;
    let storage_to_remove = storage_avail_goal - storage_avail;

    println!("Storage size:        {}", storage_total);
    println!("Storage used:        {}", storage_used);
    println!("Available:           {}", storage_avail);
    println!("Target availability: {}", storage_avail_goal);
    println!("Minimum to remove:   {}", storage_to_remove);

    let mut smallest_candidate: Option<Dir> = None;
    for dir in all_dirs {
        if dir.size < storage_to_remove {
            continue;
        }
        match smallest_candidate {
            None => smallest_candidate = Some(dir),
            Some(small) if dir.size < small.size => smallest_candidate = Some(dir),
            _ => (),
        }
    }

    println!("Smallest to remove: {:?}", smallest_candidate.unwrap());
}
