use std::fs;

use utils::get_input_path;

fn gets_presents(n: usize) -> usize {
    !gethighest_one_bit(n * 2) & (n << 1 | 1)
}

fn gethighest_one_bit(n: usize) -> usize {
    for i in (0..usize::BITS).rev() {
        let to_check = n & 1 << i;
        if to_check != 0 {
            return to_check;
        }
    }
    0
}

fn run(input_file: &str) {
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let amount_of_elves: usize = line.parse().unwrap();

    // Result
    println!("Result of part 1 {}", gets_presents(amount_of_elves));
}

fn run2(input_file: &str) {
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let amount_of_elves: usize = line.parse().unwrap();

    // Prepare
    let mut i = 1;

    // Solve
    while i * 3 < amount_of_elves {
        i *= 3;
    }

    let result  = amount_of_elves - i;

    // Result
    println!("Result of part 2 is {:?}", result);
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
