use core::panic;
use std::cell::{RefCell, RefMut};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

use utils::get_input_path;

#[derive(Debug, Clone, Copy)]
enum InstructionType {
    Inc,
    Dec,
    Jnz,
    Cpy,
    Tgl,
    Out,
}

impl From<&str> for InstructionType {
    fn from(value: &str) -> Self {
        match value {
            "inc" => InstructionType::Inc,
            "dec" => InstructionType::Dec,
            "cpy" => InstructionType::Cpy,
            "jnz" => InstructionType::Jnz,
            "tgl" => InstructionType::Tgl,
            "out" => InstructionType::Out,
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
    register: Register,
    register_2: Register,
    value: i64,
    value_2: i64,
}
impl Instruction {
    pub(crate) fn new_from(line: String) -> Self {
        let split: Vec<&str> = line.split(' ').collect();

        let instruction_type = split[0].into();
        let register;
        let register_2;
        let value;
        let value_2;

        if split[1].chars().next().unwrap().is_ascii_alphabetic() {
            register = split[1].into();
            value = 0;
        } else {
            register = Register::None;
            value = split[1].parse().unwrap();
        }

        if split.len() > 2 {
            if split[2].chars().next().unwrap().is_ascii_alphabetic() {
                register_2 = split[2].into();
                value_2 = 0;
            } else {
                register_2 = Register::None;
                value_2 = split[2].parse().unwrap();
            }
        } else {
            register_2 = Register::None;
            value_2 = 0;
        }

        Self {
            instruction_type,
            register,
            register_2,
            value,
            value_2,
        }
    }
}

fn cpy_instruction(
    instruction: &RefMut<Instruction>,
    register_a: &mut i64,
    register_b: &mut i64,
    register_c: &mut i64,
    register_d: &mut i64,
) {
    let to_copy = match instruction.register {
        Register::A => *register_a,
        Register::B => *register_b,
        Register::C => *register_c,
        Register::D => *register_d,
        Register::None => instruction.value,
    };

    match instruction.register_2 {
        Register::A => *register_a = to_copy,
        Register::B => *register_b = to_copy,
        Register::C => *register_c = to_copy,
        Register::D => *register_d = to_copy,
        Register::None => {}
    };
}

fn run(input_file: &str) {
    // Preamble
    let mut instruction_list: Vec<Rc<RefCell<Instruction>>> = Vec::new();
    let mut register_start_position: i64 = 0;

    let mut register_a: i64;
    let mut register_b: i64;
    let mut register_c: i64;
    let mut register_d: i64;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let instruction = Instruction::new_from(line);
        instruction_list.push(Rc::new(RefCell::new(instruction)));
    }

    // for instruction in &instruction_list {
    //     println!("{instruction:?}");
    // }

    // Solve
    'outer: loop {
        register_start_position += 1;
        register_a = register_start_position;
        register_b = 0;
        register_c = 0;
        register_d = 0;
        let mut next_tik = 0;
        let mut tik_count = 0;
        let mut pos: i64 = 0;
        while pos >= 0 && pos < instruction_list.len() as i64 {
            let instruction_ref = instruction_list.get(pos as usize).unwrap().clone();
            let mut instruction = instruction_ref.borrow_mut();

            match instruction.instruction_type {
                InstructionType::Inc => match instruction.register {
                    Register::A => register_a += 1,
                    Register::B => register_b += 1,
                    Register::C => register_c += 1,
                    Register::D => register_d += 1,
                    Register::None => panic!("Should not be here!"),
                },
                InstructionType::Dec => match instruction.register {
                    Register::A => register_a -= 1,
                    Register::B => register_b -= 1,
                    Register::C => register_c -= 1,
                    Register::D => register_d -= 1,
                    Register::None => panic!("Should not be here!"),
                },
                InstructionType::Jnz => {
                    let v1 = match instruction.register {
                        Register::A => register_a,
                        Register::B => register_b,
                        Register::C => register_c,
                        Register::D => register_d,
                        Register::None => instruction.value,
                    };
                    if v1 != 0 {
                        let v2 = match instruction.register_2 {
                            Register::A => register_a,
                            Register::B => register_b,
                            Register::C => register_c,
                            Register::D => register_d,
                            Register::None => instruction.value_2,
                        };

                        pos += v2 - 1;
                    }
                }
                InstructionType::Cpy => cpy_instruction(
                    &instruction,
                    &mut register_a,
                    &mut register_b,
                    &mut register_c,
                    &mut register_d,
                ),
                InstructionType::Tgl => {
                    let goto = match instruction.register {
                        Register::A => register_a,
                        Register::B => register_b,
                        Register::C => register_c,
                        Register::D => register_d,
                        Register::None => panic!("Should not be here!"),
                    } + pos;
                    if pos == goto {
                        instruction.instruction_type = InstructionType::Inc
                    } else if let Some(inner_instruction_rc) = instruction_list.get(goto as usize) {
                        let inner_instracution = inner_instruction_rc.clone();
                        let mut inner_instruction_borrow = inner_instracution.borrow_mut();
                        match inner_instruction_borrow.instruction_type {
                            InstructionType::Inc => {
                                inner_instruction_borrow.instruction_type = InstructionType::Dec
                            }
                            InstructionType::Dec => {
                                inner_instruction_borrow.instruction_type = InstructionType::Inc
                            }
                            InstructionType::Jnz => {
                                inner_instruction_borrow.instruction_type = InstructionType::Cpy
                            }
                            InstructionType::Cpy => {
                                inner_instruction_borrow.instruction_type = InstructionType::Jnz
                            }
                            InstructionType::Tgl => {
                                inner_instruction_borrow.instruction_type = InstructionType::Inc
                            }
                            InstructionType::Out => {
                                inner_instruction_borrow.instruction_type = InstructionType::Inc
                            }
                        }
                    }
                }
                InstructionType::Out => {
                    let out = match instruction.register{
                        Register::A => register_a,
                        Register::B => register_b,
                        Register::C => register_c,
                        Register::D => register_d,
                        Register::None => panic!("Should not be here!"),
                    };
                    if !(out != 0 || out != 1) || out != next_tik {
                        // println!("Current failure Start: {register_start_position} Out: {out}");
                        break;
                    }

                    next_tik = if next_tik == 0 { 1 } else { 0 };

                    tik_count += 1;
                    if tik_count >= 100 {
                        break 'outer;
                    }
                }
            }
            pos += 1;
        }
    }

    // Result
    println!("Result is {}", register_start_position);
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

    run(input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }
}
