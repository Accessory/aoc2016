use std::fs;

use utils::get_input_path;

#[derive(Clone, Debug)]
struct Row {
    pub row: String,
    pub safe: usize,
}

impl Row {
    fn new(row: String) -> Self {
        let safe = row.chars().filter(|c| c == &'.').count();
        Self { row, safe }
    }
    fn next_row(&self) -> Self {
        let mut rtn = String::with_capacity(self.row.len());
        let mut safe = 0;
        for i in 0..self.row.len() {
            let left = i == 0 || self.row.chars().nth(i - 1).unwrap() == '.';
            let center = self.row.chars().nth(i).unwrap() == '.';
            let right = i == self.row.len() - 1 || self.row.chars().nth(i + 1).unwrap() == '.';

            if !left && !center && right {
                rtn.push('^');
                continue;
            }
            if left && !center && !right {
                rtn.push('^');
                continue;
            }
            if !left && center && right {
                rtn.push('^');
                continue;
            }
            if left && center && !right {
                rtn.push('^');
                continue;
            }
            rtn.push('.');
            safe += 1;
        }

        Self { row: rtn, safe }
    }
}

impl From<String> for Row {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut cache = Vec::new();

    #[cfg(test)]
    const ITERATIONS: usize = 10;

    #[cfg(not(test))]
    const ITERATIONS: usize = 40;

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let mut row: Row = line.into();
    // Solve
    for _ in 0..ITERATIONS {
        cache.push(row.clone());
        // println!("{}", row.row);
        row = row.next_row();
    }

    // Result
    let result:usize = cache.iter().map(|i| i.safe).sum();
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut result = 0;
    #[cfg(test)]
    const ITERATIONS: usize = 10;

    #[cfg(not(test))]
    const ITERATIONS: usize = 400000;

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let mut row: Row = line.into();
    // Solve
    for _ in 0..ITERATIONS {
        result += row.safe;
        row = row.next_row();
    }

    // Result
    println!("Result of part 2 is {}", result);
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