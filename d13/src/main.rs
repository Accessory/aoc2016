use std::{collections::HashSet, fs};

use utils::get_input_path;

fn is_wall(x: i64, y: i64, favorite_number: i64) -> bool {
    x < 0
        || y < 0
        || (x * x + 3 * x + 2 * x * y + y + y * y + favorite_number).count_ones() % 2 == 1
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn create_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn create_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn create_up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn create_down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    steps: usize,
    position: Point,
}

fn create_new_options(point: Point, favorite_number: i64) -> Vec<Point> {
    vec![
        point.create_up(),
        point.create_down(),
        point.create_left(),
        point.create_right(),
    ]
    .iter()
    .filter(|p| !is_wall(p.x, p.y, favorite_number))
    .map(|p| *p)
    .collect::<Vec<Point>>()
}

fn run(input_file: &str) {
    // Preamble
    #[cfg(test)]
    const FINAL_X: i64 = 7;
    #[cfg(test)]
    const FINAL_Y: i64 = 4;
    #[cfg(not(test))]
    const FINAL_X: i64 = 31;
    #[cfg(not(test))]
    const FINAL_Y: i64 = 39;

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let favorite_number: i64 = line.parse().unwrap();

    // Prepare
    let mut cache: HashSet<Point> = HashSet::new();
    let mut queue: Vec<State> = Vec::new();

    queue.push(State {
        steps: 0,
        position: Point { x: 1, y: 1 },
    });
    let mut current_state = queue.first().unwrap().clone();
    // Solve
    while !queue.is_empty() {
        current_state = queue.remove(0);

        if current_state.position.x == FINAL_X && current_state.position.y == FINAL_Y {
            break;
        }

        if !cache.insert(current_state.position) {
            continue;
        }

        let options = create_new_options(current_state.position, favorite_number);
        for option in options {
            queue.push(State {
                steps: current_state.steps + 1,
                position: option,
            });
        }
    }

    // Result
    println!("Result is {:?}", current_state);
}

fn run2(input_file: &str) {
    // Preamble
    const MAX_STEPS: usize = 50;

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let favorite_number: i64 = line.parse().unwrap();

    // Prepare
    let mut cache: HashSet<Point> = HashSet::new();
    let mut queue: Vec<State> = Vec::new();

    queue.push(State {
        steps: 0,
        position: Point { x: 1, y: 1 },
    });
    // Solve
    while !queue.is_empty() {
        let current_state = queue.remove(0);

        if current_state.steps > MAX_STEPS {
            continue;
        }

        if !cache.insert(current_state.position) {
            continue;
        }

        let options = create_new_options(current_state.position, favorite_number);
        for option in options {
            queue.push(State {
                steps: current_state.steps + 1,
                position: option,
            });
        }
    }

    // Result
    println!("Result is {}", cache.len());
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
