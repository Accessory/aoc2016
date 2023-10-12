use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[cfg(test)]
const VALUE_LENGTH: usize = 5;

#[cfg(not(test))]
const VALUE_LENGTH: usize = 8;

#[derive(Debug, Clone, Copy)]
enum InstructionType {
    Swap,
    Rotate,
    Move,
    Reverse,
}

#[derive(Debug, Clone, Copy)]
enum InstructionSubType {
    Letter,
    Position,
    Right,
    Left,
    Based,
}

impl From<&str> for InstructionSubType {
    fn from(value: &str) -> Self {
        match value {
            "letter" => InstructionSubType::Letter,
            "position" | "positions" => InstructionSubType::Position,
            "right" => InstructionSubType::Right,
            "left" => InstructionSubType::Left,
            "based" => InstructionSubType::Based,
            _ => panic!("Should not be here"),
        }
    }
}

impl From<&str> for InstructionType {
    fn from(value: &str) -> Self {
        match value {
            "swap" => InstructionType::Swap,
            "rotate" => InstructionType::Rotate,
            "move" => InstructionType::Move,
            "reverse" => InstructionType::Reverse,
            _ => panic!("Should not be here"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Register {
    A,
    B,
    C,
    D,
    None,
}

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        match value {
            "a" | "a," => Register::A,
            "b" | "b," => Register::B,
            "c" | "c," => Register::C,
            "d" | "d," => Register::D,
            _ => Register::None,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    instruction_sub_type: InstructionSubType,
    letter_1: char,
    letter_2: char,
    value_1: usize,
    value_2: usize,
}
impl Instruction {
    pub(crate) fn new_from(line: String) -> Self {
        let split: Vec<&str> = line.split(' ').collect();

        let instruction_type = split[0].into();
        let instruction_sub_type = split[1].into();

        let mut letter_1 = '0';
        let mut letter_2 = '0';
        let mut value_1 = 0;
        let mut value_2 = 0;

        match instruction_type {
            InstructionType::Reverse => {
                value_1 = split[2].parse().unwrap();
                value_2 = split[4].parse().unwrap();
            }
            InstructionType::Move | InstructionType::Swap => match instruction_sub_type {
                InstructionSubType::Letter => {
                    letter_1 = split[2].chars().next().unwrap();
                    letter_2 = split[5].chars().next().unwrap();
                }
                InstructionSubType::Position => {
                    value_1 = split[2].parse().unwrap();
                    value_2 = split[5].parse().unwrap();
                }
                _ => panic!("Should not be here"),
            },
            InstructionType::Rotate => match instruction_sub_type {
                InstructionSubType::Left | InstructionSubType::Right => {
                    value_1 = split[2].parse().unwrap();
                }
                InstructionSubType::Based => {
                    letter_1 = split[6].chars().next().unwrap();
                }
                _ => panic!("Should not be here!"),
            },
        }

        Self {
            instruction_type,
            instruction_sub_type,
            letter_1,
            letter_2,
            value_1,
            value_2,
        }
    }

    fn swap_letter(&self, values: &mut [char; VALUE_LENGTH]) {
        let idx_1 = values
            .iter()
            .enumerate()
            .find(|i| i.1 == &self.letter_1)
            .map(|i| i.0)
            .unwrap();
        let idx_2 = values
            .iter()
            .enumerate()
            .find(|i| i.1 == &self.letter_2)
            .map(|i| i.0)
            .unwrap();

        values.swap(idx_1, idx_2);
    }

    fn swap_position(&self, values: &mut [char; VALUE_LENGTH]) {
        values.swap(self.value_1, self.value_2);
    }

    fn rotate_right(&self, values: &mut [char; VALUE_LENGTH]) {
        values.rotate_right(self.value_1);
    }

    fn rotate_left(&self, values: &mut [char; VALUE_LENGTH]) {
        values.rotate_left(self.value_1);
    }

    fn rotate_based(&self, values: &mut [char; VALUE_LENGTH]) {
        let idx_1 = values
            .iter()
            .enumerate()
            .find(|i| i.1 == &self.letter_1)
            .map(|i| i.0)
            .unwrap();

        let mut rotations = 1 + idx_1;
        if idx_1 >= 4 {
            rotations += 1;
        }
        if rotations > values.len() {
            rotations %= values.len();
        }
        values.rotate_right(rotations);
    }

    fn rotate_based_undo(&self, values: &mut [char; VALUE_LENGTH]) {
        let idx_1 = values
            .iter()
            .enumerate()
            .find(|i| i.1 == &self.letter_1)
            .map(|i| i.0)
            .unwrap();

        let mut rotations = idx_1 / 2;
        rotations += if idx_1 % 2 == 1 || idx_1 == 0 { 1 } else { 5 };

        values.rotate_left(rotations);
    }

    fn move_letter(&self, values: &mut [char; VALUE_LENGTH]) {
        let idx_1 = values
            .iter()
            .enumerate()
            .find(|i| i.1 == &self.letter_1)
            .map(|i| i.0)
            .unwrap();
        let idx_2 = values
            .iter()
            .enumerate()
            .find(|i| i.1 == &self.letter_2)
            .map(|i| i.0)
            .unwrap();

        let idx_list: Vec<usize> = if idx_1 < idx_2 {
            (idx_1..=idx_2).collect()
        } else {
            (idx_2..=idx_1).rev().collect()
        };

        for i in idx_list.windows(2) {
            values.swap(i[0], i[1]);
        }
    }

    fn move_position(&self, values: &mut [char; VALUE_LENGTH]) {
        let idx_list: Vec<usize> = if self.value_1 < self.value_2 {
            (self.value_1..=self.value_2).collect()
        } else {
            (self.value_2..=self.value_1).rev().collect()
        };

        for i in idx_list.windows(2) {
            values.swap(i[0], i[1]);
        }
    }

    fn reverse_position(&self, values: &mut [char; VALUE_LENGTH]) {
        let end: usize = self.value_1 + (self.value_2 - self.value_1 + 1) / 2;
        for i in self.value_1..end {
            let end_swap = self.value_2 - i + self.value_1;
            values.swap(i, end_swap);
        }
    }

    fn move_letter_undo(&self, values: &mut [char; VALUE_LENGTH]) {
        let idx_1 = values
            .iter()
            .enumerate()
            .find(|i| i.1 == &self.letter_2)
            .map(|i| i.0)
            .unwrap();
        let idx_2 = values
            .iter()
            .enumerate()
            .find(|i| i.1 == &self.letter_1)
            .map(|i| i.0)
            .unwrap();

        let idx_list: Vec<usize> = if idx_1 < idx_2 {
            (idx_1..=idx_2).collect()
        } else {
            (idx_2..=idx_1).rev().collect()
        };

        for i in idx_list.windows(2) {
            values.swap(i[0], i[1]);
        }
    }

    fn move_position_undo(&self, values: &mut [char; VALUE_LENGTH]) {
        let idx_list: Vec<usize> = if self.value_2 < self.value_1 {
            (self.value_2..=self.value_1).collect()
        } else {
            (self.value_1..=self.value_2).rev().collect()
        };

        for i in idx_list.windows(2) {
            values.swap(i[0], i[1]);
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    #[cfg(test)]
    let mut values: [char; VALUE_LENGTH] = ['a', 'b', 'c', 'd', 'e'];

    #[cfg(not(test))]
    let mut values: [char; VALUE_LENGTH] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    let mut instruction_list = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let instruction = Instruction::new_from(line);
        instruction_list.push(instruction);
    }

    // for instruction in &instruction_list {
    //     println!("{instruction:?}");
    // }

    // Solve
    for instruction in &instruction_list {
        match instruction.instruction_type {
            InstructionType::Swap => match instruction.instruction_sub_type {
                InstructionSubType::Letter => instruction.swap_letter(&mut values),
                InstructionSubType::Position => instruction.swap_position(&mut values),
                _ => panic!("Should not be here!"),
            },
            InstructionType::Rotate => match instruction.instruction_sub_type {
                InstructionSubType::Right => instruction.rotate_right(&mut values),
                InstructionSubType::Left => instruction.rotate_left(&mut values),
                InstructionSubType::Based => instruction.rotate_based(&mut values),
                _ => panic!("Should not be here!"),
            },
            InstructionType::Move => match instruction.instruction_sub_type {
                InstructionSubType::Letter => instruction.move_letter(&mut values),
                InstructionSubType::Position => instruction.move_position(&mut values),
                _ => panic!("Should not be here!"),
            },
            InstructionType::Reverse => match instruction.instruction_sub_type {
                InstructionSubType::Position => instruction.reverse_position(&mut values),
                _ => panic!("Should not be here!"),
            },
        }
    }

    // Result
    print!("Result of part 1 is: ");
    values.iter().for_each(|c| print!("{c}"));
    println!();
}

fn run2(input_file: &str) {
    // Preamble
    #[cfg(test)]
    let mut values: [char; VALUE_LENGTH] = ['d', 'e', 'c', 'a', 'b'];

    #[cfg(not(test))]
    let mut values: [char; VALUE_LENGTH] = ['f', 'b', 'g', 'd', 'c', 'e', 'a', 'h'];

    let mut instruction_list = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let instruction = Instruction::new_from(line);
        instruction_list.push(instruction);
    }

    // for instruction in &instruction_list {
    //     println!("{instruction:?}");
    // }

    // Solve
    for instruction in instruction_list.iter().rev() {
        match instruction.instruction_type {
            InstructionType::Swap => match instruction.instruction_sub_type {
                InstructionSubType::Letter => instruction.swap_letter(&mut values),
                InstructionSubType::Position => instruction.swap_position(&mut values),
                _ => panic!("Should not be here!"),
            },
            InstructionType::Rotate => match instruction.instruction_sub_type {
                InstructionSubType::Left => instruction.rotate_right(&mut values),
                InstructionSubType::Right => instruction.rotate_left(&mut values),
                InstructionSubType::Based => instruction.rotate_based_undo(&mut values),
                _ => panic!("Should not be here!"),
            },
            InstructionType::Move => match instruction.instruction_sub_type {
                InstructionSubType::Letter => instruction.move_letter_undo(&mut values),
                InstructionSubType::Position => instruction.move_position_undo(&mut values),
                _ => panic!("Should not be here!"),
            },
            InstructionType::Reverse => match instruction.instruction_sub_type {
                InstructionSubType::Position => instruction.reverse_position(&mut values),
                _ => panic!("Should not be here!"),
            },
        }
    }

    // Result
    print!("Result of part 2 is: ");
    values.iter().for_each(|c| print!("{c}"));
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
