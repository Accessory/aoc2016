use std::fs;

use bit_vec::BitVec;
use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    #[cfg(test)]
    const DISK_LENGTH: usize = 20;
    #[cfg(not(test))]
    const DISK_LENGTH: usize = 272;

    let mut state = BitVec::new();

    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    for c in line.chars() {
        if c == '1' {
            state.push(true);
        } else {
            state.push(false);
        }
    }

    // Solve
    while state.len() < DISK_LENGTH {
        // Extend
        let mut b: BitVec = state.iter().rev().map(|i| !i).collect();
        state.push(false);
        state.append(&mut b);
    }

    // Cut
    state.split_off(DISK_LENGTH);

    // Shrink
    while state.len() % 2 == 0 {
        let mut new_state = BitVec::new();
        for i in (0..state.len() - 1).step_by(2) {
            let b1 = state.get(i).unwrap();
            let b2 = state.get(i + 1).unwrap();

            new_state.push(b1 == b2);
        }

        state = new_state;
    }

    // Result
    println!("Result is {:?}", state);
}

fn run2(input_file: &str) {
        // Preamble
        #[cfg(test)]
        const DISK_LENGTH: usize = 20;
        #[cfg(not(test))]
        const DISK_LENGTH: usize = 35651584;
    
        let mut state = BitVec::new();
    
        // Parse
        let line = fs::read_to_string(input_file).unwrap().trim().to_string();
        for c in line.chars() {
            if c == '1' {
                state.push(true);
            } else {
                state.push(false);
            }
        }
    
        // Solve
        while state.len() < DISK_LENGTH {
            // Extend
            let mut b: BitVec = state.iter().rev().map(|i| !i).collect();
            state.push(false);
            state.append(&mut b);
        }
    
        // Cut
        state.split_off(DISK_LENGTH);

        // Shrink
        while state.len() % 2 == 0 {
            let mut new_state = BitVec::new();
            for i in (0..state.len() - 1).step_by(2) {
                let b1 = state.get(i).unwrap();
                let b2 = state.get(i + 1).unwrap();
    
                new_state.push(b1 == b2);
            }
    
            state = new_state;
        }
    
        // Result
        println!("Result is {:?}", state);
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
