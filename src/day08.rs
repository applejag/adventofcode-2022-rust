use crate::day::Part;
use std::fmt::Debug;
use std::fs;
use std::io::{self, prelude::*};
use std::ops::Range;

pub fn run(part: Part, file_path: &str) {
    match part {
        Part::Part1 => part1(file_path),
        Part::Part2 => part2(file_path),
    }
}

struct Grid {
    width: usize,
    height: usize,
    vec: Vec<Vec<u8>>,
}

impl Grid {
    fn new(vec: Vec<Vec<u8>>) -> Self {
        Self {
            width: vec[0].len(),
            height: vec.len(),
            vec,
        }
    }

    fn read(file_path: &str) -> Grid {
        let file = fs::File::open(file_path).expect("Read the input file");
        let reader = io::BufReader::new(file);

        let mut vec: Vec<Vec<u8>> = vec![];
        for line in reader.lines().map(|l| l.unwrap()) {
            let mut line_vec: Vec<u8> = vec![];
            for c in line.chars() {
                let digit = c.to_digit(10).unwrap() as u8;
                line_vec.push(digit);
            }
            vec.push(line_vec);
        }

        Grid::new(vec)
    }

    fn get_tree_height(&self, x: usize, y: usize) -> u8 {
        self.vec[y][x]
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        match (x, y) {
            (0, _) => true,
            (_, 0) => true,
            (x, y) if x == self.width - 1 || y == self.height - 1 => true,
            _ => {
                let tree_height = self.get_tree_height(x, y);
                self.trees_lower_x_range(tree_height, 0..x, y)
                    || self.trees_lower_x_range(tree_height, (x + 1)..self.width, y)
                    || self.trees_lower_y_range(tree_height, x, 0..y)
                    || self.trees_lower_y_range(tree_height, x, (y + 1)..self.height)
            }
        }
    }

    fn trees_lower_x_range(&self, tree_height: u8, x_range: Range<usize>, y: usize) -> bool {
        for new_x in x_range {
            if self.get_tree_height(new_x, y) >= tree_height {
                return false;
            }
        }
        true
    }

    fn trees_lower_y_range(&self, tree_height: u8, x: usize, y_range: Range<usize>) -> bool {
        for new_y in y_range {
            if self.get_tree_height(x, new_y) >= tree_height {
                return false;
            }
        }
        true
    }

    fn visible_trees_count(&self) -> usize {
        let mut visible_count = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.is_tree_visible(x, y) {
                    visible_count += 1;
                }
            }
        }
        visible_count
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let tree_height = self.get_tree_height(x, y);
        self.scenic_score_x_range(tree_height, (0..x).rev(), y)
            * self.scenic_score_x_range(tree_height, (x + 1)..self.width, y)
            * self.scenic_score_y_range(tree_height, x, (0..y).rev())
            * self.scenic_score_y_range(tree_height, x, (y + 1)..self.height)
    }

    fn scenic_score_x_range<T: IntoIterator<Item = usize>>(
        &self,
        tree_height: u8,
        x_range: T,
        y: usize,
    ) -> usize {
        let mut count = 0;
        for new_x in x_range {
            count += 1;
            if self.get_tree_height(new_x, y) >= tree_height {
                break;
            }
        }
        return count;
    }

    fn scenic_score_y_range<T: IntoIterator<Item = usize>>(
        &self,
        tree_height: u8,
        x: usize,
        y_range: T,
    ) -> usize {
        let mut count = 0;
        for new_y in y_range {
            count += 1;
            if self.get_tree_height(x, new_y) >= tree_height {
                break;
            }
        }
        return count;
    }

    fn highest_scenic_score(&self) -> usize {
        let mut highest_score = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                let score = self.scenic_score(x, y);
                if score > highest_score {
                    highest_score = score;
                }
            }
        }
        highest_score
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

fn part1(file_path: &str) {
    let grid = Grid::read(file_path);
    let visible_count = grid.visible_trees_count();

    println!("Grid: {:?}", grid);
    println!("Visible trees: {}", visible_count);
}

fn part2(file_path: &str) {
    let grid = Grid::read(file_path);
    let highest_score = grid.highest_scenic_score();

    println!("Grid: {:?}", grid);
    println!("Highest scenic score: {}", highest_score);
}

#[cfg(test)]
mod tests {
    use crate::day08::Grid;

    fn example_grid() -> Grid {
        let vec: Vec<Vec<u8>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        Grid::new(vec)
    }

    #[test]
    fn test_is_tree_visible_corners() {
        let grid = example_grid();
        assert_eq!(grid.is_tree_visible(0, 0), true);
        assert_eq!(grid.is_tree_visible(4, 0), true);
        assert_eq!(grid.is_tree_visible(0, 4), true);
        assert_eq!(grid.is_tree_visible(4, 4), true);
    }

    #[test]
    fn test_is_tree_visible_interior() {
        let grid = example_grid();
        assert_eq!(grid.is_tree_visible(1, 1), true);
        assert_eq!(grid.is_tree_visible(3, 1), false);
    }

    #[test]
    fn test_visible_trees_count() {
        let grid = example_grid();
        assert_eq!(grid.visible_trees_count(), 21);
    }

    #[test]
    fn test_highest_scenic_score() {
        let grid = example_grid();
        assert_eq!(grid.highest_scenic_score(), 8);
    }
}
