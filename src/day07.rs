use crate::day::Part;
//use std::borrow::BorrowMut;
//use std::fs;
//use std::io::{self, prelude::*};
//use std::rc::Rc;

pub fn run(_part: Part, _file_path: &str) {
    //match part {
    //    Part::Part1 => part1(file_path),
    //    Part::Part2 => part2(file_path),
    //}
}

/*
#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Rc<Dir>>,
}

impl Dir {
    fn new_root() -> Rc<Self> {
        Self::new("/")
    }

    fn new(name: &str) -> Rc<Self> {
        Rc::new(Self { name: name.to_string(), files: vec![], dirs: vec![] })
    }

    fn get_dir(&self, name: &str) -> Option<Rc<Dir>> {
        for dir in &self.dirs {
            if dir.name == name {
                return Some(dir.clone());
            }
        }
        None
    }

    fn get_or_add_dir(&mut self, name: &str) -> Rc<Dir> {
        match self.get_dir(name) {
            Some(dir) => dir,
            None => {
                let dir = Dir::new(name);
                self.dirs.push(dir.clone());
                dir
            },
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct File {
    name: String,
    size: u64,
}

impl File {
    fn new(name: String, size: u64) -> Self { Self { name, size } }
}

#[derive(Debug)]
struct Filesystem<'a> {
    path_stack: Vec<Rc<Dir>>,
    path_head: Rc<Dir>,
    root: &'a mut Rc<Dir>,
}

impl<'a> Filesystem<'a> {
    fn new(root: &'a mut Rc<Dir>) -> Self {
        Self { path_stack: vec![], path_head: root.clone(), root}
    }

    fn cd(&mut self, name: &str) -> Rc<Dir> {
        let x = self.root.get_or_add_dir(name);
        x
    }
}

fn part1(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut root = Dir::new_root();
    let mut _fs = Filesystem::new(&mut root);
    let mut lines = 0;
    for _line in reader.lines().map(|l| l.unwrap()) {
        lines += 1;
    }

    println!("Lines: {}", lines);
}

fn part2(file_path: &str) {
    let file = fs::File::open(file_path).expect("Read the input file");
    let reader = io::BufReader::new(file);

    let mut lines = 0;
    for _line in reader.lines().map(|l| l.unwrap()) {
        lines += 1;
    }

    println!("Lines: {}", lines);
}
*/
