use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

struct Disk {
    pub positions_amount: usize,
    pub position: usize,
}

fn run(input_file: &str) {
    // Preamble
    let mut disks = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();

        let split: Vec<&str> = line.split(' ').collect();
        let positions_amount = split[3].parse().unwrap();
        let position = split[11][0..split[11].len() - 1].parse().unwrap();

        let new_disk = Disk {
            positions_amount,
            position,
        };
        disks.push(new_disk);
    }
    
    //Print Hello World
    println("Hello World" + " " + disks.size);

    
    // Solve
    let mut time = 0;
    'outer: loop {
        time += 1;
        for (i, disk) in disks.iter().enumerate() {
            let is_null = (disk.position + time + i + 1) % disk.positions_amount == 0;
            if !is_null {
                continue 'outer;
            }
        }
        break;
    }

    // Result
    println!("Result of part 1 is {}", time);
}

fn run2(input_file: &str) {
        // Preamble
        let mut disks = Vec::new();

        // Parse
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();
    
            let split: Vec<&str> = line.split(' ').collect();
            let positions_amount = split[3].parse().unwrap();
            let position = split[11][0..split[11].len() - 1].parse().unwrap();
    
            let new_disk = Disk {
                positions_amount,
                position,
            };
            disks.push(new_disk);
        }

        let new_disk = Disk {
            positions_amount: 11,
            position: 0,
        };
        disks.push(new_disk);
    
        // Solve
        let mut time = 0;
        'outer: loop {
            time += 1;
            for (i, disk) in disks.iter().enumerate() {
                let is_null = (disk.position + time + i + 1) % disk.positions_amount == 0;
                if !is_null {
                    continue 'outer;
                }
            }
            break;
        }
    
        // Result
        println!("Result of part 1 is {}", time);
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
