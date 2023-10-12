#![feature(const_trait_impl)]
use std::{collections::HashSet, fs, panic};

use utils::get_input_path;

#[derive(Clone, Copy, Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn walk(x: i64, y: i64, direction: Direction, distance: i64) -> (i64, i64) {
    match direction {
        Direction::NORTH => (x, y - distance),
        Direction::EAST => (x + distance, y),
        Direction::SOUTH => (x, y + distance),
        Direction::WEST => (x - distance, y),
    }
}

impl Direction {
    fn right(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }
    fn left(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::WEST,
            Direction::EAST => Direction::NORTH,
            Direction::SOUTH => Direction::EAST,
            Direction::WEST => Direction::SOUTH,
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::NORTH;
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let mut split = line.split(' ');
    while let Some(turn) = split.next() {
        direction = match turn.chars().nth(0).unwrap() {
            'R' => direction.right(),
            'L' => direction.left(),
            _ => panic!("Should never be here"),
        };
        let number = turn[1..].split(',').next().unwrap();
        let distance = number.parse().unwrap();
        (x, y) = walk(x, y, direction, distance);
    }

    // Solve
    let result = x.abs() + y.abs();
    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut x = 0;
    let mut y = 0;
    let mut direction = Direction::NORTH;
    let mut positions: HashSet<(i64, i64)> = HashSet::new();
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let mut split = line.split(' ');

    'outer: while let Some(turn) = split.next() {
        direction = match turn.chars().nth(0).unwrap() {
            'R' => direction.right(),
            'L' => direction.left(),
            _ => panic!("Should never be here"),
        };
        let number = turn[1..].split(',').next().unwrap();
        let distance = number.parse().unwrap();
        for _ in 0..distance {
            (x, y) = walk(x, y, direction, 1);
            if !positions.insert((x, y)) {
                break 'outer;
            }
        }
    }

    // Solve
    let result = x.abs() + y.abs();
    // Result
    println!("Result is {}", result);
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

    run(input_file);
    run2(input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_2_path;
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_2() {
        let input_path = get_test_input_2_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
