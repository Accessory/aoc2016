use std::{collections::HashSet, fs};

use utils::get_input_path;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    secret: String,
    hex: String,
    distance: usize,
}

impl State {
    fn new(x: usize, y: usize, secret: String, distance: usize) -> Self {
        let hex = format!("{:x}", md5::compute(secret.as_bytes()));
        Self {
            x,
            y,
            secret: secret.to_string(),
            hex,
            distance,
        }
    }

    fn get_result(&self) -> &str {
        const TO_CHECK:[char;4] = ['U', 'D', 'L', 'R'];
        if let Some(idx) = self.secret.find(|c:char| TO_CHECK.contains(&c)) {
            &self.secret[idx..]
        } else {
            ""
        }

    }

    fn generate_options(&self) -> Vec<State> {
        const TO_CHECK: [char; 5] = ['b', 'c', 'd', 'e', 'f'];
        let new_distance = self.distance + 1;
        let mut rtn: Vec<State> = Vec::new();
        if self.y != 0 && TO_CHECK.contains(&self.hex.chars().next().unwrap()) {
            rtn.push(State::new(
                self.x,
                self.y - 1,
                format!("{}{}", self.secret, 'U'),
                new_distance,
            ));
        }
        if self.y != 3 && TO_CHECK.contains(&self.hex.chars().nth(1).unwrap()) {
            rtn.push(State::new(
                self.x,
                self.y + 1,
                format!("{}{}", self.secret, 'D'),
                new_distance,
            ));
        }
        if self.x != 0 && TO_CHECK.contains(&self.hex.chars().nth(2).unwrap()) {
            rtn.push(State::new(
                self.x - 1,
                self.y,
                format!("{}{}", self.secret, 'L'),
                new_distance,
            ));
        }
        if self.x != 3 && TO_CHECK.contains(&self.hex.chars().nth(3).unwrap()) {
            rtn.push(State::new(
                self.x + 1,
                self.y,
                format!("{}{}", self.secret, 'R'),
                new_distance,
            ));
        }

        rtn
    }
}

fn run(input_file: &str) {
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    // Solve
    let mut cache: HashSet<String> = HashSet::new();
    let mut current_state = State::new(0, 0, String::from(line), 0);
    let mut queue: Vec<State> = vec!(current_state.clone());
    while !queue.is_empty() {
        current_state = queue.remove(0);
        if current_state.x == 3 && current_state.y == 3 {
            break;
        }
        if !cache.insert(current_state.hex.clone()) {
            continue;
        }
        let mut options = current_state.generate_options();
        queue.append(&mut options);
    }

    // Result
    println!("Result of part 1 is: {}", current_state.get_result());
}

fn run2(input_file: &str) {
        // Parse
        let line = fs::read_to_string(input_file).unwrap().trim().to_string();
        // Solve
        let mut cache: HashSet<String> = HashSet::new();
        let mut result = 0;
        let mut current_state = State::new(0, 0, String::from(line), 0);
        let mut queue: Vec<State> = vec!(current_state.clone());
        while !queue.is_empty() {
            current_state = queue.remove(0);
            if current_state.x == 3 && current_state.y == 3 {
                result = result.max(current_state.distance);
                continue;
            }
            if !cache.insert(current_state.hex.clone()) {
                continue;
            }
            let mut options = current_state.generate_options();
            queue.append(&mut options);
        }
    
        // Result
        println!("Result of part 2 is: {}", result);
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
