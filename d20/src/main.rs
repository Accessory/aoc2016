use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LowHigh {
    pub low: usize,
    pub high: usize,
}

impl LowHigh {
    fn is_in_range(&self, n: usize) -> bool {
        n >= self.low && n <= self.high
    }

    fn range_count(&self) -> usize {
        self.high - self.low + 1
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut ranges = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split('-');
        let low: usize = split.next().unwrap().parse().unwrap();
        let high: usize = split.next().unwrap().parse().unwrap();

        ranges.push(LowHigh { low, high });
    }

    ranges.sort_unstable();

    // Solve
    let mut current_low = 0;
    for range in &ranges {
        if range.is_in_range(current_low) {
            current_low = range.high + 1;
        }
    }

    // Result
    println!("Result is {}", current_low);
}

fn run2(input_file: &str) {
    // Preamble
    #[cfg(test)]
    const MAX: usize = 9;
    #[cfg(not(test))]
    const MAX: usize = 4294967295;

    let mut ranges = Vec::new();
    let mut ranges_merged = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split('-');
        let low: usize = split.next().unwrap().parse().unwrap();
        let high: usize = split.next().unwrap().parse().unwrap();

        ranges.push(LowHigh { low, high });
    }

    ranges.sort_unstable();

    // Solve
    let mut low = 0;
    let mut high = 0;

    for range in &ranges {
        if range.low <= high+1 {
            high = high.max(range.high);
        } else {
            ranges_merged.push(LowHigh { low, high });
            low = range.low;
            high = range.high;
        }
    }
    ranges_merged.push(LowHigh { low, high });

    // Result
    let blocked = ranges_merged.iter().map(|r| r.range_count()).sum::<usize>();
    let result: usize = MAX - blocked + 1;
    println!("Result is 1 {}", result);
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
