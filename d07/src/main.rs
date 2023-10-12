use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    // Preamble
    let mut result: usize = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut in_bracket = false;
        let mut is_tls = false;
        let mut has_tls_in_backet = false;
        // let mut result_match = String::new();
        for bytes in line.as_bytes().windows(4) {
            let c1 = bytes[0];
            let c2 = bytes[1];
            let c3 = bytes[2];
            let c4 = bytes[3];

            if c4 == b'[' || c3 == b'[' || c2 == b'[' || c1 == b'[' {
                in_bracket = true;
                continue;
            }

            if c1 == c4 && c2 == c3 && c1 != c2 {
                if in_bracket {
                    has_tls_in_backet = true;
                    break;
                }
                // result_match = format!("{}{}{}{}", c1 as char, c2 as char, c3 as char, c4 as char);
                is_tls = true;
            }

            if in_bracket {
                if c1 == b']' {
                    in_bracket = false;
                }
            }
        }

        if is_tls && !has_tls_in_backet {
            // println!("{line} does support tls. Found {result_match}");
            result += 1;
        }
    }

    // Solve
    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut result: usize = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut in_bracket = false;
        let mut has_ssl = false;
        // let mut result_match = String::new();
        let mut ssl_check_in_brackets: HashSet<String> = HashSet::new();
        let mut ssl_check_out_brackets: HashSet<String> = HashSet::new();
        for bytes in line.as_bytes().windows(3) {
            let c1 = bytes[0];
            let c2 = bytes[1];
            let c3 = bytes[2];

            if c3 == b'[' || c2 == b'[' || c1 == b'[' {
                in_bracket = true;
                continue;
            }

            if in_bracket {
                if c1 == b']' {
                    in_bracket = false;
                    continue;
                }
            }

            if c1 == c3 && c1 != c2 {
                if in_bracket {
                    if ssl_check_in_brackets
                        .contains(&format!("{}{}{}", c1 as char, c2 as char, c3 as char))
                    {
                        has_ssl = true;
                        // result_match = format!("{}{}{}", c2 as char, c1 as char, c2 as char);
                        break;
                    } else {
                        ssl_check_out_brackets
                            .insert(format!("{}{}{}", c2 as char, c1 as char, c2 as char));
                    }
                } else {
                    if ssl_check_out_brackets
                        .contains(&format!("{}{}{}", c1 as char, c2 as char, c3 as char))
                    {
                        has_ssl = true;
                        // result_match = format!("{}{}{}", c1 as char, c2 as char, c3 as char);
                        break;
                    } else {
                        ssl_check_in_brackets.insert(format!("{}{}{}", c2 as char, c1 as char, c2 as char));
                    }
                }
            }
        }

        if has_ssl {
            // println!("{line} does support SSL. Found {result_match}");
            result += 1;
        }
    }

    // Solve
    // Result
    println!("Result is {}", result);
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
