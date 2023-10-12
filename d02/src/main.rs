use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut x: i32 = 1;
    let mut y: i32 = 1;
    let mut result = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        for c in line.chars() {
            match c {
                'U' => y = y.sub(1).max(0),
                'D' => y = y.add(1).min(2),
                'L' => x = x.sub(1).max(0),
                'R' => x = x.add(1).min(2),
                _ => panic!("Should not be here!"),
            };
        }
        result.push(y * 3 + x + 1);
    }

    // Solve
    // Result
    print!("Result is: ");
    result.iter().for_each(|n| print!("{n}"));
    println!();
}

fn run2(input_file: &str) {
    // Preamble
    let mut x: i32 = 0;
    let mut y: i32 = 2;
    let mut result = Vec::new();
    //  let keypad = [
    //     "  1  ",
    //     " 234 ",
    //     "56789",
    //     " ABC ",
    //     "  D  ",
    //  ];

    let keypad = "  1   234 56789 ABC   D  ";
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        for c in line.chars() {
            match c {
                'U' => {
                    let bounding_y = (x - 2).abs();
                    y = y.sub(1).max(bounding_y)
                }
                'D' => {
                    let bounding_y = (x - 2).abs();
                    y = y.add(1).min(4 - bounding_y)
                }
                'L' => {
                    let bounding_x = (y - 2).abs();
                    x = x.sub(1).max(bounding_x)
                }
                'R' => {
                    let bounding_x = (y - 2).abs();
                    x = x.add(1).min(4 - bounding_x)
                }
                _ => panic!("Should not be here!"),
            };
        }
        result.push(y * 5 + x);
    }

    // Solve
    // Result
    print!("Result is: ");
    result
        .iter()
        .for_each(|n| print!("{}", keypad.chars().nth((*n) as usize).unwrap()));
    println!();
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
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
