use std::fs;

use utils::get_input_path;

struct Marker {
    subsequent: usize,
    repeat: usize,
    len: usize,
    subsequent_length: usize,
}

fn run(input_file: &str) {
    // Preamble
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let line_size = line.len();
    let mut pos: usize = 0;
    // let mut result_string: String = String::with_capacity(line.len());
    let mut result: usize = 0;

    let mut last_marker: Option<Marker> = None;
    // Solve
    while pos < line_size {
        let c = line.chars().nth(pos).unwrap();
        if let Some(marker) = &mut last_marker {
            if c == '(' {
                while pos < line_size {
                    (*marker).subsequent_length += 1;
                    pos += 1;
                    let x = line.chars().nth(pos).unwrap();
                    if x == ')' {
                        break;
                    }
                }
            }
            (*marker).subsequent_length += 1;
            (*marker).subsequent -= 1;
            if (*marker).subsequent == 0 {
                result += marker.subsequent_length * marker.repeat;
                last_marker = None;
            }
        } else {
            if c.is_ascii_alphabetic() {
                result += 1;
            } else if c == '(' {
                let mut substring = String::new();
                let start = pos;
                while pos < line_size {
                    pos += 1;
                    let x = line.chars().nth(pos).unwrap();
                    if x == ')' {
                        break;
                    }
                    substring.push(x);
                }

                let mut split = substring.split('x');
                let subsequent = split.next().unwrap().parse().unwrap();
                let repeat = split.next().unwrap().parse().unwrap();

                last_marker = Some(Marker {
                    subsequent,
                    repeat,
                    len: pos - start,
                    subsequent_length: 0,
                });
            }
        }

        pos += 1;
    }

    if let Some(marker) = last_marker {
        result += marker.subsequent_length * marker.repeat;
    }

    // Result
    println!("Result is {}", result);
}

fn run2(_input_file: &str) {}

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
