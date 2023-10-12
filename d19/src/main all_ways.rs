use std::fs;

use utils::get_input_path;

#[derive(Debug)]
struct Elv {
    pub id: usize,
    // pub presents: usize,
}

fn gets_presents(n: usize) -> usize {
    !gethighest_one_bit(n*2) & ((n<<1 | 1))
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

    // Prepare
    let mut elves: Vec<Elv> = Vec::with_capacity(amount_of_elves);

    for i in 0..amount_of_elves {
        elves.push(Elv {
            id: i + 1,
            // presents: 1,
        });
    }

    // Solve
    // let mut pos = 0;
    while elves.len() != 1 {
        let delete_first = elves.len() % 2 == 1;

        elves = elves
            .into_iter()
            .enumerate()
            .filter_map(|(i, elv)| if i % 2 == 0 { Some(elv) } else { None })
            .collect();
        if elves.len() == 1 {
            break;
        }

        if delete_first {
            elves.remove(0);
        }

        // println!("Elves left: {}", elves.len());
    }

    // Result
    println!("Result of part 1 is {:?}", elves[0].id);

    println!("Result of part 1 try 2 {}", gets_presents(amount_of_elves));
}

fn run2(input_file: &str) {
    // Parse
    let line = fs::read_to_string(input_file).unwrap().trim().to_string();
    let amount_of_elves: usize = line.parse().unwrap();

    // Prepare
    let mut elves: Vec<Elv> = Vec::with_capacity(amount_of_elves);

    for i in 0..amount_of_elves {
        elves.push(Elv {
            id: i + 1,
            // presents: 1,
        });
    }

    // Solve
    let mut pos = 0;
    while elves.len() != 1 {
        let to_remove = (pos + elves.len() / 2) % elves.len();
        elves.remove(to_remove);

        if to_remove > pos {
            pos = (pos + 1) % elves.len();
        }

        println!("Elves left: {}", elves.len());
    }

    // while elves.len() != 1 {
    //     let delete_if = if elves.len() % 2 == 0 { 0 } else { 1 };
    //     elves = elves
    //         .into_iter()
    //         .enumerate()
    //         .filter_map(|(i, elv)| if i % 2 == delete_if { Some(elv) } else { None })
    //         .collect();
    // }

    // Result
    println!("Result of part 2 is {:?}", elves[0].id);
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
