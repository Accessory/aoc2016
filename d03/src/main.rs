#![feature(iter_array_chunks)]
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut result: usize = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut lines: Vec<i64> = line
            .split(' ')
            .filter(|f| !f.is_empty())
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        lines.sort_unstable();

        if lines[0] + lines[1] > lines[2] {
            result += 1;
        }
    }

    // Solve
    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut result: usize = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for [line1, line2, line3] in reader.lines().into_iter().array_chunks() {
        let lines1: Vec<i64> = line1
            .unwrap()
            .split(' ')
            .filter(|f| !f.is_empty())
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        let lines2: Vec<i64> = line2
            .unwrap()
            .split(' ')
            .filter(|f| !f.is_empty())
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        let lines3: Vec<i64> = line3
            .unwrap()
            .split(' ')
            .filter(|f| !f.is_empty())
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        let mut check_lines1: Vec<i64> = vec![lines1[0], lines2[0], lines3[0]];
        check_lines1.sort_unstable();
        if check_lines1[0] + check_lines1[1] > check_lines1[2] {
            result += 1;
        }

        let mut check_lines2: Vec<i64> = vec![lines1[1], lines2[1], lines3[1]];
        check_lines2.sort_unstable();
        if check_lines2[0] + check_lines2[1] > check_lines2[2] {
            result += 1;
        }

        let mut check_lines3: Vec<i64> = vec![lines1[2], lines2[2], lines3[2]];
        check_lines3.sort_unstable();
        if check_lines3[0] + check_lines3[1] > check_lines3[2] {
            result += 1;
        }
    }

    // Solve
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
