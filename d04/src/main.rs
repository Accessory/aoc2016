use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

use utils::get_input_path;

#[derive(Debug)]
#[allow(dead_code)]
struct CharCounter {
    count: usize,
    first: usize,
    ch: char,
}

fn calc_checksum(char_counter: HashMap<char, CharCounter>) -> String {
    let mut char_counters: Vec<&CharCounter> = char_counter.values().collect();

    char_counters.sort_by(|l, r| match r.count.cmp(&l.count) {
        std::cmp::Ordering::Equal => l.ch.cmp(&r.ch),
        res => res,
    });
    let mut rtn = String::with_capacity(5);
    let x = char_counters[0..5].iter().map(|f| f.ch);
    for c in x {
        rtn.push(c);
    }
    rtn
}

fn decrypt_room_name(full_line: &str, sector_id: usize) -> String {
    const BOTTOM: usize = 'a' as usize;
    const TOP: usize = 'z' as usize + 1;
    const RANGE: usize = TOP - BOTTOM;
    // println!("Bottom: {BOTTOM}, TOP: {TOP}, RANGE: {RANGE}");
    let mut rtn = String::with_capacity(full_line.len());
    for c in full_line.chars() {
        if c.is_digit(10) {
            break;
        }
        if c == '-' {
            rtn.push(' ');
            continue;
        }

        let nci: u8 = (((c as usize - BOTTOM + sector_id) % RANGE) + BOTTOM) as u8;
        rtn.push(nci as char);
    }

    rtn
}

fn run(input_file: &str) {
    // Preamble
    let mut result = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split('[').collect();
        let mut char_counter: HashMap<char, CharCounter> = HashMap::new();
        for (i, c) in split[0].chars().enumerate() {
            if !c.is_ascii_alphabetic() {
                continue;
            }

            if char_counter.contains_key(&c) {
                char_counter.get_mut(&c).unwrap().count += 1;
            } else {
                char_counter.insert(
                    c,
                    CharCounter {
                        ch: c,
                        first: i,
                        count: 1,
                    },
                );
            }
        }
        let check_sum_calc = calc_checksum(char_counter);
        let check_sum = &split[1][0..split[1].len() - 1];

        if check_sum == check_sum_calc {
            println!("{check_sum} == {check_sum_calc}");
            let sector_id: usize = split[0].split('-').last().unwrap().parse().unwrap();
            result += sector_id;
        } else {
            println!("{check_sum} != {check_sum_calc}");
        }
    }

    // Solve
    // Result
    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    // let mut result = 0;
    let mut room_lines = Vec::new();
    let mut room_sector_ids = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split('[').collect();
        let mut char_counter: HashMap<char, CharCounter> = HashMap::new();
        for (i, c) in split[0].chars().enumerate() {
            if !c.is_ascii_alphabetic() {
                continue;
            }

            if char_counter.contains_key(&c) {
                char_counter.get_mut(&c).unwrap().count += 1;
            } else {
                char_counter.insert(
                    c,
                    CharCounter {
                        ch: c,
                        first: i,
                        count: 1,
                    },
                );
            }
        }
        let check_sum_calc = calc_checksum(char_counter);
        let check_sum = &split[1][0..split[1].len() - 1];

        if check_sum == check_sum_calc {
            //  println!("{check_sum} == {check_sum_calc}");
            let sector_id: usize = split[0].split('-').last().unwrap().parse().unwrap();
            //  result += sector_id;
            let room_line = decrypt_room_name(split[0], sector_id);
            room_lines.push(room_line);
            room_sector_ids.push(sector_id);
        } else {
            println!("{check_sum} != {check_sum_calc}");
        }
    }

    // Solve
    // Result
    let mut file = fs::File::create("output.txt").unwrap();
    for (i, result) in room_lines.iter().enumerate() {
        println!("Room name {}, SectorId: {}", result, room_sector_ids[i]);
        file.write_fmt(format_args!(
            "Room name {}, SectorId: {}\n",
            result, room_sector_ids[i]
        ))
        .expect("Could not write to file");
    }
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
