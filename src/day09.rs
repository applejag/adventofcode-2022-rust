use crate::day::Part;
use std::fmt::Display;
use std::fs;
use std::io::{self, prelude::*};

pub fn run(part: Part, file_path: &str) {
    match part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn try_parse(c: char) -> Option<Direction> {
        match c {
            'U' => Some(Direction::Up),
            'R' => Some(Direction::Right),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Move {
    dir: Direction,
    steps: usize,
}

impl Move {
    fn new(dir: Direction, steps: usize) -> Self {
        Self { dir, steps }
    }

    fn try_parse(s: &str) -> Option<Move> {
        let (dir, steps_str) = s.split_once(' ')?;
        let steps: usize = steps_str.parse().ok()?;
        let dir = Direction::try_parse(dir.chars().nth(0)?)?;

        Some(Self::new(dir, steps))
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} x {}", self.dir, self.steps)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn mv(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn follow(&mut self, other: &Point) {
        let (dx, dy) = self.delta(other);
        let (ax, ay) = (dx.abs() as usize, dy.abs() as usize);
        let (sx, sy) = (dx.signum(), dy.signum());

        match (ax, ay) {
            (0..=1, 0..=1) => (), // Don't need to move
            (0, 2) => self.y = other.y - sy,
            (2, 0) => self.x = other.x - sx,
            (1, 2) => {
                self.y = other.y - sy;
                self.x = other.x;
            }
            (2, 1) => {
                self.x = other.x - sx;
                self.y = other.y;
            }
            (2, 2) => {
                self.x = other.x - sx;
                self.y = other.y - sy;
            }
            _ => panic!("woa too big diff: {}, {}", dx, dy),
        }
    }

    fn delta(&self, other: &Point) -> (isize, isize) {
        (other.x - self.x, other.y - self.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

fn count_unique_tail_points(moves: Vec<Move>) -> usize {
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let mut tail_points = vec![tail];

    for mv in moves {
        for _ in 0..mv.steps {
            head.mv(mv.dir);
            tail.follow(&head);
            if !tail_points.contains(&tail) {
                tail_points.push(tail);
            }
        }
    }

    tail_points.len()
}

struct Rope {
    head: Point,
    tail1: Point,
    tail2: Point,
    tail3: Point,
    tail4: Point,
    tail5: Point,
    tail6: Point,
    tail7: Point,
    tail8: Point,
    tail9: Point,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Point::new(0, 0),
            tail1: Point::new(0, 0),
            tail2: Point::new(0, 0),
            tail3: Point::new(0, 0),
            tail4: Point::new(0, 0),
            tail5: Point::new(0, 0),
            tail6: Point::new(0, 0),
            tail7: Point::new(0, 0),
            tail8: Point::new(0, 0),
            tail9: Point::new(0, 0),
        }
    }

    fn mv_head(&mut self, dir: Direction) {
        self.head.mv(dir);
        self.tail1.follow(&self.head);
        self.tail2.follow(&self.tail1);
        self.tail3.follow(&self.tail2);
        self.tail4.follow(&self.tail3);
        self.tail5.follow(&self.tail4);
        self.tail6.follow(&self.tail5);
        self.tail7.follow(&self.tail6);
        self.tail8.follow(&self.tail7);
        self.tail9.follow(&self.tail8);
    }
}

fn count_unique_tail_points_rope(moves: Vec<Move>) -> usize {
    let mut rope = Rope::new();
    let mut tail_points = vec![rope.tail9];

    for mv in moves {
        for _ in 0..mv.steps {
            rope.mv_head(mv.dir);
            if !tail_points.contains(&rope.tail9) {
                tail_points.push(rope.tail9);
            }
        }
    }

    tail_points.len()
}

fn part1(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut moves = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        let mv = Move::try_parse(&line).unwrap();
        moves.push(mv);
    }

    let tail_points_count = count_unique_tail_points(moves);

    println!("Unique tail points: {}", tail_points_count);
}

fn part2(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut moves = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        let mv = Move::try_parse(&line).unwrap();
        moves.push(mv);
    }

    let tail_points_count = count_unique_tail_points_rope(moves);

    println!("Unique tail points: {}", tail_points_count);
}

#[cfg(test)]
mod tests {
    use crate::day09::count_unique_tail_points;

    use super::{Direction, Move};

    fn example_moves() -> Vec<Move> {
        vec![
            Move::new(Direction::Right, 4),
            Move::new(Direction::Up, 4),
            Move::new(Direction::Left, 3),
            Move::new(Direction::Down, 1),
            Move::new(Direction::Right, 4),
            Move::new(Direction::Down, 1),
            Move::new(Direction::Left, 5),
            Move::new(Direction::Right, 2),
        ]
    }

    #[test]
    fn test_count_unique_tail_points() {
        let moves = example_moves();
        assert_eq!(count_unique_tail_points(moves), 13);
    }
}
