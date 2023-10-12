use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

struct Screen {
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

enum InstructionType {
    Rect,
    RotateColumn,
    RotateRow,
}

struct Instruction {
    instruction_type: InstructionType,
    v1: usize,
    v2: usize,
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let split: Vec<&str> = value.split(' ').collect();
        let instruction_type;
        let v1;
        let v2;
        if split[0] == "rect" {
            instruction_type = InstructionType::Rect;
            let split2: Vec<&str> = split[1].split('x').collect();
            v1 = split2[0].parse().unwrap();
            v2 = split2[1].parse().unwrap();
        } else if split[1] == "row" {
            instruction_type = InstructionType::RotateRow;
            let split2: Vec<&str> = split[2].split('=').collect();
            v1 = split2[1].parse().unwrap();
            v2 = split[4].parse().unwrap();
        } else {
            let split2: Vec<&str> = split[2].split('=').collect();
            v1 = split2[1].parse().unwrap();
            v2 = split[4].parse().unwrap();
            instruction_type = InstructionType::RotateColumn;
        }

        Self {
            instruction_type,
            v1,
            v2,
        }
    }
}

impl Screen {
    fn new_with_dimensions(x: usize, y: usize) -> Self {
        let grid = vec![vec![false; x]; y];
        Self {
            grid,
            width: x,
            height: y,
        }
    }

    fn create_rect(&mut self, x: usize, y: usize) {
        for x in 0..x {
            for y in 0..y {
                self.grid[y][x] = true;
            }
        }
    }

    fn rotate_column(&mut self, x: usize, by: usize) {
        let mut column = Vec::with_capacity(self.height);
        self.grid.iter().for_each(|i| {
            column.push(i[x]);
        });

        for (i, r) in column.iter().enumerate() {
            let new_place = (i + by) % self.height;
            self.grid[new_place][x] = *r;
        }
    }

    fn rotate_row(&mut self, y: usize, by: usize) {
        let row = self.grid[y].clone();
        for (i, r) in row.iter().enumerate() {
            let new_place = (i + by) % self.width;
            self.grid[y][new_place] = *r;
        }
    }

    fn print(&self) {
        for row in &self.grid {
            for column in row {
                let ch = if *column { '#' } else { '.' };
                print!("{ch}");
            }
            println!();
        }
    }

    fn on_count(&self) -> usize {
        let mut rtn = 0;
        for row in &self.grid {
            for column in row {
                if *column {
                    rtn += 1;
                }
            }
        }
        rtn
    }
}

fn run(input_file: &str) {
    // Preamble
    #[cfg(test)]
    let mut screen = Screen::new_with_dimensions(7, 3);
    #[cfg(not(test))]
    let mut screen = Screen::new_with_dimensions(50, 6);

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        instructions.push(line.into());
    }

    // Solve
    for instruction in instructions {
        match instruction.instruction_type {
            InstructionType::Rect => screen.create_rect(instruction.v1, instruction.v2),
            InstructionType::RotateColumn => screen.rotate_column(instruction.v1, instruction.v2),
            InstructionType::RotateRow => screen.rotate_row(instruction.v1, instruction.v2),
        }
    }

    // Result
    // screen.print();

    let result = screen.on_count();

    println!("Result is {}", result);
}

fn run2(input_file: &str) {
        // Preamble
        #[cfg(test)]
        let mut screen = Screen::new_with_dimensions(7, 3);
        #[cfg(not(test))]
        let mut screen = Screen::new_with_dimensions(50, 6);
    
        // Parse
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
    
        let mut instructions: Vec<Instruction> = Vec::new();
    
        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();
            instructions.push(line.into());
        }
    
        // Solve
        for instruction in instructions {
            match instruction.instruction_type {
                InstructionType::Rect => screen.create_rect(instruction.v1, instruction.v2),
                InstructionType::RotateColumn => screen.rotate_column(instruction.v1, instruction.v2),
                InstructionType::RotateRow => screen.rotate_row(instruction.v1, instruction.v2),
            }
        }
    
        // Result
        screen.print();
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
