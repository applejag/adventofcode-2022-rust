use crate::day::Part;
use std::fs;
use std::io::{self, prelude::*};
use std::vec::IntoIter;

pub fn run(part: Part, file_path: &str) {
    match part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Noop,
    AddX(i64),
}

impl Op {
    fn parse(s: &str) -> Result<Op, String> {
        let split = s.split_once(' ');
        let op = split.map(|s| s.0).unwrap_or(s);
        let arg = split.map(|s| s.1);
        match (op, arg) {
            ("noop", None) => Ok(Op::Noop),
            ("addx", Some(n)) => Ok(Op::AddX(n.parse::<i64>().map_err(|e| e.to_string())?)),
            _ => Err(format!("unknown op: {}", s)),
        }
    }

    fn extra_cycles(&self) -> usize {
        match self {
            Self::Noop => 0,
            Self::AddX(_) => 1,
        }
    }

    fn read_ops_from_file(file_path: &str) -> Result<Vec<Op>, String> {
        let file = fs::File::open(file_path).expect("Read the input file");
        let reader = io::BufReader::new(file);

        let mut ops = vec![];
        for line in reader.lines().map(|l| l.unwrap()) {
            ops.push(Self::parse(&line)?);
        }

        Ok(ops)
    }

    fn on_complete(&self, state: &mut State) {
        match self {
            Self::Noop => (),
            Self::AddX(n) => state.x += n,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    x: i64,
    cycle: i64,
}

impl State {
    fn new() -> Self {
        Self { x: 1, cycle: 0 }
    }
}

struct VM {
    state: State,

    ops_iter: IntoIter<Op>,
    current_op: Option<Op>,
    current_op_cycles: usize,
}

impl VM {
    fn new(ops: Vec<Op>) -> Self {
        Self {
            state: State::new(),
            ops_iter: ops.into_iter(),
            current_op: None,
            current_op_cycles: 0,
        }
    }
}

impl Iterator for VM {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.cycle += 1;
        if self.current_op_cycles > 0 {
            self.current_op_cycles -= 1;
            return Some(self.state);
        }

        match self.current_op {
            Some(op) => op.on_complete(&mut self.state),
            _ => (),
        }

        self.current_op = self.ops_iter.next();
        match self.current_op {
            Some(op) => {
                self.current_op_cycles = op.extra_cycles();
                Some(self.state)
            }
            None => None,
        }
    }
}

fn calc_signal_strength(vm: &mut VM) -> i64 {
    let mut sum = 0;
    for state in vm {
        match state.cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                sum += state.cycle * state.x;
            }
            _ => (),
        }
    }
    sum
}

fn part1(file_path: &str) {
    let ops = Op::read_ops_from_file(file_path).expect("Read ops from input file");
    let mut vm = VM::new(ops);
    let sum = calc_signal_strength(&mut vm);

    println!("VM state: {:?}", vm.state);
    println!("Sum: {}", sum);
}

fn part2(file_path: &str) {
    let ops = Op::read_ops_from_file(file_path).expect("Read ops from input file");
    let vm = VM::new(ops);

    println!("VM state: {:?}", vm.state);
}

// noop
// addx 3
// addx -5

#[cfg(test)]
mod tests {
    use crate::day10::{calc_signal_strength, Op, VM};

    #[test]
    fn test_small_program() {
        let mut vm = VM::new(vec![Op::Noop, Op::AddX(3), Op::AddX(-5)]);
        for _ in &mut vm {}
        assert_eq!(vm.state.x, -1);
    }

    #[test]
    fn test_large_program() {
        let mut vm = VM::new(vec![
            Op::AddX(15),
            Op::AddX(-11),
            Op::AddX(6),
            Op::AddX(-3),
            Op::AddX(5),
            Op::AddX(-1),
            Op::AddX(-8),
            Op::AddX(13),
            Op::AddX(4),
            Op::Noop,
            Op::AddX(-1),
            Op::AddX(5),
            Op::AddX(-1),
            Op::AddX(5),
            Op::AddX(-1),
            Op::AddX(5),
            Op::AddX(-1),
            Op::AddX(5),
            Op::AddX(-1),
            Op::AddX(-35),
            Op::AddX(1),
            Op::AddX(24),
            Op::AddX(-19),
            Op::AddX(1),
            Op::AddX(16),
            Op::AddX(-11),
            Op::Noop,
            Op::Noop,
            Op::AddX(21),
            Op::AddX(-15),
            Op::Noop,
            Op::Noop,
            Op::AddX(-3),
            Op::AddX(9),
            Op::AddX(1),
            Op::AddX(-3),
            Op::AddX(8),
            Op::AddX(1),
            Op::AddX(5),
            Op::Noop,
            Op::Noop,
            Op::Noop,
            Op::Noop,
            Op::Noop,
            Op::AddX(-36),
            Op::Noop,
            Op::AddX(1),
            Op::AddX(7),
            Op::Noop,
            Op::Noop,
            Op::Noop,
            Op::AddX(2),
            Op::AddX(6),
            Op::Noop,
            Op::Noop,
            Op::Noop,
            Op::Noop,
            Op::Noop,
            Op::AddX(1),
            Op::Noop,
            Op::Noop,
            Op::AddX(7),
            Op::AddX(1),
            Op::Noop,
            Op::AddX(-13),
            Op::AddX(13),
            Op::AddX(7),
            Op::Noop,
            Op::AddX(1),
            Op::AddX(-33),
            Op::Noop,
            Op::Noop,
            Op::Noop,
            Op::AddX(2),
            Op::Noop,
            Op::Noop,
            Op::Noop,
            Op::AddX(8),
            Op::Noop,
            Op::AddX(-1),
            Op::AddX(2),
            Op::AddX(1),
            Op::Noop,
            Op::AddX(17),
            Op::AddX(-9),
            Op::AddX(1),
            Op::AddX(1),
            Op::AddX(-3),
            Op::AddX(11),
            Op::Noop,
            Op::Noop,
            Op::AddX(1),
            Op::Noop,
            Op::AddX(1),
            Op::Noop,
            Op::Noop,
            Op::AddX(-13),
            Op::AddX(-19),
            Op::AddX(1),
            Op::AddX(3),
            Op::AddX(26),
            Op::AddX(-30),
            Op::AddX(12),
            Op::AddX(-1),
            Op::AddX(3),
            Op::AddX(1),
            Op::Noop,
            Op::Noop,
            Op::Noop,
            Op::AddX(-9),
            Op::AddX(18),
            Op::AddX(1),
            Op::AddX(2),
            Op::Noop,
            Op::Noop,
            Op::AddX(9),
            Op::Noop,
            Op::Noop,
            Op::Noop,
            Op::AddX(-1),
            Op::AddX(2),
            Op::AddX(-37),
            Op::AddX(1),
            Op::AddX(3),
            Op::Noop,
            Op::AddX(15),
            Op::AddX(-21),
            Op::AddX(22),
            Op::AddX(-6),
            Op::AddX(1),
            Op::Noop,
            Op::AddX(2),
            Op::AddX(1),
            Op::Noop,
            Op::AddX(-10),
            Op::Noop,
            Op::Noop,
            Op::AddX(20),
            Op::AddX(1),
            Op::AddX(2),
            Op::AddX(2),
            Op::AddX(-6),
            Op::AddX(-11),
            Op::Noop,
            Op::Noop,
            Op::Noop,
        ]);
        assert_eq!(calc_signal_strength(&mut vm), 13140);
    }
}
