use std::fs;

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    const PASSWORD_LENGTH: usize = 8;
    let mut result: String = String::with_capacity(8);
    let mut pos: usize = 0;

    // Parse
    let secret = fs::read_to_string(input_file).unwrap().trim().to_string();

    // Solve
    for _ in 0..PASSWORD_LENGTH {
        loop {
            pos += 1;
            let to_test = format!("{secret}{pos}");

            let md5_result = md5::compute(to_test.as_bytes());

            if md5_result[0] == 0 && md5_result[1] == 0 && (md5_result[2] & 0xF0) == 0 {
                println!("Found Hash {:?}", md5_result);

                let hex_char = format!("{:x}", (md5_result[2] & 0x0F));
                result.push(hex_char.chars().nth(0).unwrap());
                break;
            }
        }
    }

    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    const PASSWORD_LENGTH: u8 = 8;
    let mut result: [char; PASSWORD_LENGTH as usize] = [' '; PASSWORD_LENGTH as usize];
    let mut pos: usize = 0;
    let mut found_positions = 0;

    // Parse
    let secret = fs::read_to_string(input_file).unwrap().trim().to_string();

    // Solve
    'outer: loop {
        loop {
            pos += 1;
            let to_test = format!("{secret}{pos}");

            let md5_result = md5::compute(to_test.as_bytes());

            if md5_result[0] == 0 && md5_result[1] == 0 && (md5_result[2] & 0xF0) == 0 {
                println!("Found Hash {:?}", md5_result);
                let hex_pos = md5_result[2] & 0x0F;
                if hex_pos < PASSWORD_LENGTH && result[hex_pos as usize] == ' ' {
                    let hex_char = format!("{:x}", (md5_result[3] & 0xF0));
                    result[hex_pos as usize] = hex_char.chars().nth(0).unwrap();
                    found_positions += 1;
                    if found_positions == PASSWORD_LENGTH {
                        break 'outer;
                    }
                    break;
                };
            }
        }
    }

    // Result
    println!("Result is: ");
    result.iter().for_each(|c| print!("{c}"));
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
