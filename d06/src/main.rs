use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut char_counter_pos_1: HashMap<char, usize> = HashMap::new();
    let mut char_counter_pos_2: HashMap<char, usize> = HashMap::new();
    let mut char_counter_pos_3: HashMap<char, usize> = HashMap::new();
    let mut char_counter_pos_4: HashMap<char, usize> = HashMap::new();
    let mut char_counter_pos_5: HashMap<char, usize> = HashMap::new();
    let mut char_counter_pos_6: HashMap<char, usize> = HashMap::new();
    #[cfg(not(test))]
    let mut char_counter_pos_7: HashMap<char, usize> = HashMap::new();
    #[cfg(not(test))]
    let mut char_counter_pos_8: HashMap<char, usize> = HashMap::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let c1 = line.chars().nth(0).unwrap();
        let c2 = line.chars().nth(1).unwrap();
        let c3 = line.chars().nth(2).unwrap();
        let c4 = line.chars().nth(3).unwrap();
        let c5 = line.chars().nth(4).unwrap();
        let c6 = line.chars().nth(5).unwrap();
        #[cfg(not(test))]
        let c7 = line.chars().nth(6).unwrap();
        #[cfg(not(test))]
        let c8 = line.chars().nth(7).unwrap();

        if let Some(old) = char_counter_pos_1.get(&c1) {
            char_counter_pos_1.insert(c1, old + 1);
        } else {
            char_counter_pos_1.insert(c1, 1);
        }

        if let Some(old) = char_counter_pos_2.get(&c2) {
            char_counter_pos_2.insert(c2, old + 1);
        } else {
            char_counter_pos_2.insert(c2, 1);
        }

        if let Some(old) = char_counter_pos_3.get(&c3) {
            char_counter_pos_3.insert(c3, old + 1);
        } else {
            char_counter_pos_3.insert(c3, 1);
        }

        if let Some(old) = char_counter_pos_4.get(&c4) {
            char_counter_pos_4.insert(c4, old + 1);
        } else {
            char_counter_pos_4.insert(c4, 1);
        }

        if let Some(old) = char_counter_pos_5.get(&c5) {
            char_counter_pos_5.insert(c5, old + 1);
        } else {
            char_counter_pos_5.insert(c5, 1);
        }

        if let Some(old) = char_counter_pos_6.get(&c6) {
            char_counter_pos_6.insert(c6, old + 1);
        } else {
            char_counter_pos_6.insert(c6, 1);
        }

        #[cfg(not(test))]
        if let Some(old) = char_counter_pos_7.get(&c7) {
            char_counter_pos_7.insert(c7, old + 1);
        } else {
            char_counter_pos_7.insert(c7, 1);
        }

        #[cfg(not(test))]
        if let Some(old) = char_counter_pos_8.get(&c8) {
            char_counter_pos_8.insert(c8, old + 1);
        } else {
            char_counter_pos_8.insert(c8, 1);
        }
    }


    let max_1 = *char_counter_pos_1.iter().max_by_key(|i| i.1).unwrap().0;
    let max_2 = *char_counter_pos_2.iter().max_by_key(|i| i.1).unwrap().0;
    let max_3 = *char_counter_pos_3.iter().max_by_key(|i| i.1).unwrap().0;
    let max_4 = *char_counter_pos_4.iter().max_by_key(|i| i.1).unwrap().0;
    let max_5 = *char_counter_pos_5.iter().max_by_key(|i| i.1).unwrap().0;
    let max_6 = *char_counter_pos_6.iter().max_by_key(|i| i.1).unwrap().0;
    #[cfg(not(test))]
    let max_7 = *char_counter_pos_7.iter().max_by_key(|i| i.1).unwrap().0;
    #[cfg(not(test))]
    let max_8 = *char_counter_pos_8.iter().max_by_key(|i| i.1).unwrap().0;

    // Solve
    // Result
    print!("Result is: ");

    print!("{max_1}");
    print!("{max_2}");
    print!("{max_3}");
    print!("{max_4}");
    print!("{max_5}");
    print!("{max_6}");
    #[cfg(not(test))]
    print!("{max_7}");
    #[cfg(not(test))]
    print!("{max_8}");
    println!();
}

fn run2(input_file: &str) {
        // Preamble
        let mut char_counter_pos_1: HashMap<char, usize> = HashMap::new();
        let mut char_counter_pos_2: HashMap<char, usize> = HashMap::new();
        let mut char_counter_pos_3: HashMap<char, usize> = HashMap::new();
        let mut char_counter_pos_4: HashMap<char, usize> = HashMap::new();
        let mut char_counter_pos_5: HashMap<char, usize> = HashMap::new();
        let mut char_counter_pos_6: HashMap<char, usize> = HashMap::new();
        #[cfg(not(test))]
        let mut char_counter_pos_7: HashMap<char, usize> = HashMap::new();
        #[cfg(not(test))]
        let mut char_counter_pos_8: HashMap<char, usize> = HashMap::new();
    
        // Parse
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();
    
            let c1 = line.chars().nth(0).unwrap();
            let c2 = line.chars().nth(1).unwrap();
            let c3 = line.chars().nth(2).unwrap();
            let c4 = line.chars().nth(3).unwrap();
            let c5 = line.chars().nth(4).unwrap();
            let c6 = line.chars().nth(5).unwrap();
            #[cfg(not(test))]
            let c7 = line.chars().nth(6).unwrap();
            #[cfg(not(test))]
            let c8 = line.chars().nth(7).unwrap();
    
            if let Some(old) = char_counter_pos_1.get(&c1) {
                char_counter_pos_1.insert(c1, old + 1);
            } else {
                char_counter_pos_1.insert(c1, 1);
            }
    
            if let Some(old) = char_counter_pos_2.get(&c2) {
                char_counter_pos_2.insert(c2, old + 1);
            } else {
                char_counter_pos_2.insert(c2, 1);
            }
    
            if let Some(old) = char_counter_pos_3.get(&c3) {
                char_counter_pos_3.insert(c3, old + 1);
            } else {
                char_counter_pos_3.insert(c3, 1);
            }
    
            if let Some(old) = char_counter_pos_4.get(&c4) {
                char_counter_pos_4.insert(c4, old + 1);
            } else {
                char_counter_pos_4.insert(c4, 1);
            }
    
            if let Some(old) = char_counter_pos_5.get(&c5) {
                char_counter_pos_5.insert(c5, old + 1);
            } else {
                char_counter_pos_5.insert(c5, 1);
            }
    
            if let Some(old) = char_counter_pos_6.get(&c6) {
                char_counter_pos_6.insert(c6, old + 1);
            } else {
                char_counter_pos_6.insert(c6, 1);
            }
    
            #[cfg(not(test))]
            if let Some(old) = char_counter_pos_7.get(&c7) {
                char_counter_pos_7.insert(c7, old + 1);
            } else {
                char_counter_pos_7.insert(c7, 1);
            }
    
            #[cfg(not(test))]
            if let Some(old) = char_counter_pos_8.get(&c8) {
                char_counter_pos_8.insert(c8, old + 1);
            } else {
                char_counter_pos_8.insert(c8, 1);
            }
        }
    
    
        let max_1 = *char_counter_pos_1.iter().min_by_key(|i| i.1).unwrap().0;
        let max_2 = *char_counter_pos_2.iter().min_by_key(|i| i.1).unwrap().0;
        let max_3 = *char_counter_pos_3.iter().min_by_key(|i| i.1).unwrap().0;
        let max_4 = *char_counter_pos_4.iter().min_by_key(|i| i.1).unwrap().0;
        let max_5 = *char_counter_pos_5.iter().min_by_key(|i| i.1).unwrap().0;
        let max_6 = *char_counter_pos_6.iter().min_by_key(|i| i.1).unwrap().0;
        #[cfg(not(test))]
        let max_7 = *char_counter_pos_7.iter().min_by_key(|i| i.1).unwrap().0;
        #[cfg(not(test))]
        let max_8 = *char_counter_pos_8.iter().min_by_key(|i| i.1).unwrap().0;
    
        // Solve
        // Result
        print!("Result is: ");
    
        print!("{max_1}");
        print!("{max_2}");
        print!("{max_3}");
        print!("{max_4}");
        print!("{max_5}");
        print!("{max_6}");
        #[cfg(not(test))]
        print!("{max_7}");
        #[cfg(not(test))]
        print!("{max_8}");
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
