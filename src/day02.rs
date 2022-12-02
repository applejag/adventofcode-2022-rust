use std::fs;
use std::io::{self, prelude::*};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn try_parse(c: char) -> Option<Move> {
        match c {
            'A' => Some(Move::Rock),
            'X' => Some(Move::Rock),
            'B' => Some(Move::Paper),
            'Y' => Some(Move::Paper),
            'C' => Some(Move::Scissors),
            'Z' => Some(Move::Scissors),
            _ => None,
        }
    }

    fn parse(c: char) -> Move {
        Move::try_parse(c).expect("You dumb fool, pass")
    }

    fn shape_score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn battle(&self, other: &Move) -> Outcome {
        match (self, other) {
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (a, b) if a == b => Outcome::Draw,
            _ => Outcome::Lose,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn outcome_score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

pub fn part1() {
    let file_path = "inputs/day02.txt";
    println!("In file {}", file_path);

    let file = fs::File::open(file_path)
        .expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        let (opponent, your) = parse_moves(&line);

        let outcome = your.battle(&opponent);
        let score = your.shape_score() + outcome.outcome_score();
        sum += score;
    }

    println!("Total score: {}", sum);
}

fn parse_moves(line: &str) -> (Move, Move) {
    let opponent_letter = line.chars().nth(0).unwrap();
    let your_letter = line.chars().nth(2).unwrap();
    (Move::parse(opponent_letter), Move::parse(your_letter))
}
