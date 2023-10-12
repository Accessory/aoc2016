use std::fs;

use utils::get_input_path;

fn find_tripple(hex: &str) -> Option<char> {
    for window in hex.as_bytes().windows(3) {
        let first = window[0];
        if window[1] == first && window[2] == first {
            return Some(first as char);
        }
    }
    None
}

fn has_quintuple(tripple: char, hex: &str) -> bool {
    for window in hex.as_bytes().windows(5) {
        if window[0] == tripple as u8
            && window[1] == tripple as u8
            && window[2] == tripple as u8
            && window[3] == tripple as u8
            && window[4] == tripple as u8
        {
            return true;
        }
    }
    false
}

fn get_hex<'a>(pos: usize, secret: &'a str, cache: &'a mut Vec<String>) -> &'a str {
    while pos >= cache.len() {
        let to_test = format!("{secret}{}", cache.len());
        let md5_result = md5::compute(to_test.as_bytes());
        let hex_char = format!("{:x}", md5_result);
        cache.push(hex_char);
    }

    &cache[pos]
}

fn get_hex_part2<'a>(pos: usize, secret: &'a str, cache: &'a mut Vec<String>) -> &'a str {
    while pos >= cache.len() {
        let mut to_test = format!("{secret}{}", cache.len());
        for _ in 0..=2016 {
            let md5_result = md5::compute(to_test.as_bytes());
            to_test = format!("{:x}", md5_result);
        }
        cache.push(to_test);
    }
    &cache[pos]
}

fn run(input_file: &str) {
    // Preamble
    let mut cache: Vec<String> = Vec::new();
    let mut pos: usize = 0;
    let mut found_keys = 0;
    let result: usize;

    // Parse
    let secret = fs::read_to_string(input_file).unwrap().trim().to_string();

    // Solve
    'outer: loop {
        pos += 1;
        let hex = get_hex(pos, &secret, &mut cache);
        if let Some(tripple) = find_tripple(hex) {
            let inner_pos = pos + 1;
            for i in inner_pos..inner_pos + 1000 {
                let hex = get_hex(i, &secret, &mut cache);
                if has_quintuple(tripple, hex) {
                    found_keys += 1;
                    if found_keys == 64 {
                        result = pos;
                        break 'outer;
                    }
                }
            }
        }
    }

    // Result
    println!("Result of part 1 is: {result}");
}

fn run2(input_file: &str) {
    // Preamble
    let mut cache: Vec<String> = Vec::new();
    let mut pos: usize = 0;
    let mut found_keys = 0;
    let result: usize;

    // Parse
    let secret = fs::read_to_string(input_file).unwrap().trim().to_string();

    // Solve
    'outer: loop {
        pos += 1;
        let hex = get_hex_part2(pos, &secret, &mut cache);
        if let Some(tripple) = find_tripple(hex) {
            let inner_pos = pos + 1;
            for i in inner_pos..inner_pos + 1000 {
                let hex_inner = get_hex_part2(i, &secret, &mut cache);
                if has_quintuple(tripple, hex_inner) {
                    found_keys += 1;
                    // println!("Found keys: {found_keys} Idx: {pos}");
                    if found_keys == 64 {
                        result = pos;
                        break 'outer;
                    }
                    break;
                }
            }
        }
    }

    // Result
    println!("Result of part 2 is: {result}");
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
