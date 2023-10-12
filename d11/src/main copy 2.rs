use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    elevator_position: u8,
    floor_chips: [u8; 4],
    floor_generators: [u8; 4],
    turns: u8,
    hyristic: u32,
    // path: Vec<State>,
}

#[allow(dead_code)]
fn print_state(state: &State, elements: &HashMap<String, u8>) {
    print_floors(
        state.elevator_position,
        state.floor_chips,
        state.floor_generators,
        elements,
    )
}

fn print_floors(
    elevator_position: u8,
    floor_chips: [u8; 4],
    floor_generators: [u8; 4],
    elements: &HashMap<String, u8>,
) {
    for i in (0..4).rev() {
        print!("F{}", i + 1);

        if elevator_position == i as u8 {
            print!(" E ");
        } else {
            print!(" . ");
        }

        let chip = floor_chips[i];
        for (key, value) in elements {
            let generator = floor_generators[i];
            if generator & value != 0 {
                print!(
                    " {}{}G ",
                    key.chars().next().unwrap(),
                    key.chars().nth(1).unwrap()
                );
            } else {
                print!(" ... ");
            }

            if chip & value != 0 {
                print!(
                    " {}{}M ",
                    key.chars().next().unwrap(),
                    key.chars().nth(1).unwrap()
                );
            } else {
                print!(" ... ");
            }
        }
        println!()
    }
}

fn get_possibilities(values_on_floor: u8, elements: &HashMap<String, u8>) -> Vec<u8> {
    let mut rtn: Vec<u8> = vec![0, 0];
    for value in elements.values() {
        if values_on_floor & *value != 0 {
            rtn.push(*value);
        }
    }
    rtn
}

fn is_floor_valid(chips: u8, generators: u8) -> bool {
    if chips == 0 || generators == 0 {
        return true;
    }

    chips & generators == chips
}

fn generate_next_possibilities(
    current_state: &State,
    next_elevator: u8,
    elements: &HashMap<String, u8>,
) -> HashSet<State> {
    let mut next_possibilities = HashSet::new();
    let next_turn = current_state.turns + 1;

    if next_turn > 34 {
        return next_possibilities;
    }

    let current_floor = current_state.elevator_position;
    let chips_on_floor: u8 = current_state.floor_chips[current_state.elevator_position as usize];
    let generators_on_floor: u8 =
        current_state.floor_generators[current_state.elevator_position as usize];
    let chips_on_next_floor: u8 = current_state.floor_chips[next_elevator as usize];
    let generators_on_next_floor: u8 = current_state.floor_generators[next_elevator as usize];

    let chips_possibilities = get_possibilities(chips_on_floor, elements);
    let generators_possibilities = get_possibilities(generators_on_floor, elements);

    // let mut pair_was_moved = false;

    for (i1, c1) in chips_possibilities.iter().enumerate() {
        let mut moving_chips = *c1;
        for c2 in chips_possibilities.iter().skip(i1 + 1) {
            moving_chips += c2;
            for (i2, g1) in generators_possibilities.iter().enumerate() {
                let mut moving_generators = *g1;
                for g2 in generators_possibilities.iter().skip(i2 + 1) {
                    moving_generators += g2;

                    let moving_generators_count = moving_generators.count_ones();
                    let moving_chips_count = moving_chips.count_ones();

                    let ones = moving_generators_count + moving_chips_count;

                    if ones == 0 || ones > 2 {
                        continue;
                    }

                    let next_chips_current_floor = chips_on_floor - moving_chips;
                    let next_chips_next_floor = chips_on_next_floor + moving_chips;

                    let next_generators_current_floor = generators_on_floor - moving_generators;
                    let next_generators_next_floor = generators_on_next_floor + moving_generators;

                    if is_floor_valid(next_chips_current_floor, next_generators_current_floor)
                        && is_floor_valid(next_chips_next_floor, next_generators_next_floor)
                    {

                        // if moving_chips & moving_generators != 0 {
                        //     if pair_was_moved {
                        //         continue;
                        //     } else {
                        //         pair_was_moved = true;
                        //     }
                        // }

                        let mut next_state = *current_state;
                        next_state.elevator_position = next_elevator;
                        next_state.floor_chips[current_floor as usize] = next_chips_current_floor;
                        next_state.floor_generators[current_floor as usize] =
                            next_generators_current_floor;
                        next_state.floor_chips[next_elevator as usize] = next_chips_next_floor;
                        next_state.floor_generators[next_elevator as usize] =
                            next_generators_next_floor;
                        next_state.turns = next_turn;
                        // next_state.hyristic += 0;
                        // current_state.floor_chips[0] as usize
                        // + current_state.floor_chips[1] as usize * 2
                        // + current_state.floor_chips[2] as usize * 3
                        // + current_state.floor_chips[3] as usize * 4
                        // + current_state.floor_generators[0] as usize
                        // + current_state.floor_generators[1] as usize * 2
                        // + current_state.floor_generators[2] as usize * 3
                        // + current_state.floor_generators[3] as usize * 4;
                        next_possibilities.insert(next_state);
                    }
                }
            }
        }
    }
    next_possibilities
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CacheItem {
    elevator_position: u8,
    floor_chips: [u8; 4],
    floor_generators: [u8; 4],
}

fn run(input_file: &str) {
    // Preamble
    let mut elements: HashMap<String, u8> = HashMap::new();
    let mut floor_chips: [u8; 4] = [0; 4];
    let mut floor_generators: [u8; 4] = [0; 4];

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut element_count = 1;

    for (current_floor, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split(' ').collect();
        let mut window = split.windows(2).skip(4);
        while let Some(&[w1, w2]) = window.next() {
            if w2.starts_with("microchip") {
                let name = w1.split('-').next().unwrap();
                if !elements.contains_key(name) {
                    elements.insert(name.to_string(), 1 << element_count);
                    element_count += 1;
                }
                floor_chips[current_floor] += elements.get(name).unwrap();
            } else if w2.starts_with("generator") {
                if !elements.contains_key(w1) {
                    elements.insert(w1.to_string(), 1 << element_count);
                    element_count += 1;
                }
                floor_generators[current_floor] += elements.get(w1).unwrap();
            }
        }
    }

    // Solve
    let mut cache:HashSet<CacheItem> = HashSet::new();
    let init_state = State {
        elevator_position: 0,
        floor_chips,
        floor_generators,
        turns: 0,
        hyristic: 0,
    };

    let final_sum: u8 = elements.values().sum();

    let mut queue: Vec<State> = vec![init_state];

    let mut current_state: State = init_state;
    let mut current_num_steps = 0;
    while !queue.is_empty() {
        current_state = queue.remove(0);

        if current_state.floor_chips[3] == final_sum
            && current_state.floor_generators[3] == final_sum
        {
            break;
        }

        if current_num_steps < current_state.turns {
            current_num_steps = current_state.turns;
            println!("Finished steps {}", current_num_steps);
        }

        // if !cache.insert(CacheItem {
        //     elevator_position: current_state.elevator_position,
        //     floor_chips: [
        //         current_state.floor_chips[0] | current_state.floor_generators[0],
        //         current_state.floor_chips[1] | current_state.floor_generators[1],
        //         current_state.floor_chips[2] | current_state.floor_generators[2],
        //         current_state.floor_chips[3] | current_state.floor_generators[3],
        //     ],
        //     floor_generators: [0,0,0,0],
        // }) {
        //     continue;
        // }

        if !cache.insert(CacheItem {
            elevator_position: current_state.elevator_position,
            floor_chips: current_state.floor_chips,
            floor_generators: current_state.floor_generators,
        }) {
            continue;
        }

        let mut possibilities: HashSet<State> = HashSet::new();
        if current_state.elevator_position != 3 {
            let next_elevator = current_state.elevator_position + 1;
            let next_possibilities =
                generate_next_possibilities(&current_state, next_elevator, &elements);
            possibilities.extend(next_possibilities.into_iter());
        }
        if current_state.elevator_position != 0 {
            let next_elevator = current_state.elevator_position - 1;
            let next_possibilities =
                generate_next_possibilities(&current_state, next_elevator, &elements);
            possibilities.extend(next_possibilities.into_iter());
        }

        queue.extend(possibilities);
        queue.sort_unstable_by(|l, r| match r.hyristic.cmp(&l.hyristic) {
            std::cmp::Ordering::Equal => l.turns.cmp(&r.turns),
            rtn => rtn,
        });
    }

    print_state(&current_state, &elements);
    println!("Result is {:?}", current_state.turns);
}

fn run2(input_file: &str) {
    return;
    // Preamble
    let mut elements: HashMap<String, u8> = HashMap::new();
    let mut floor_chips: [u8; 4] = [0; 4];
    let mut floor_generators: [u8; 4] = [0; 4];

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut element_count = 1;

    for (current_floor, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split(' ').collect();
        let mut window = split.windows(2).skip(4);
        while let Some(&[w1, w2]) = window.next() {
            if w2.starts_with("microchip") {
                let name = w1.split('-').next().unwrap();
                if !elements.contains_key(name) {
                    elements.insert(name.to_string(), 1 << element_count);
                    element_count += 1;
                }
                floor_chips[current_floor] += elements.get(name).unwrap();
            } else if w2.starts_with("generator") {
                if !elements.contains_key(w1) {
                    elements.insert(w1.to_string(), 1 << element_count);
                    element_count += 1;
                }
                floor_generators[current_floor] += elements.get(w1).unwrap();
            }
        }
    }

    // Part 2
    elements.insert(String::from("elerium"), 1 << element_count);
    floor_chips[0] += 1 << element_count;
    floor_generators[0] += 1 << element_count;
    element_count += 1;
    elements.insert(String::from("dilithium"), 1 << element_count);
    floor_chips[0] += 1 << element_count;
    floor_generators[0] += 1 << element_count;

    // Solve
    let mut cache = HashSet::new();
    let init_state = State {
        elevator_position: 0,
        floor_chips,
        floor_generators,
        turns: 0,
        hyristic: 0,
    };

    let final_sum: u8 = elements.values().sum();

    let mut queue: Vec<State> = vec![init_state];

    let mut current_state: State = init_state;
    let mut current_num_steps = 0;
    while !queue.is_empty() {
        current_state = queue.remove(0);

        if current_state.floor_chips[3] == final_sum
            && current_state.floor_generators[3] == final_sum
        {
            break;
        }

        if current_num_steps < current_state.turns {
            current_num_steps = current_state.turns;
            println!("Finished steps {}", current_num_steps);
        }

        if !cache.insert(CacheItem {
            elevator_position: current_state.elevator_position,
            floor_chips: current_state.floor_chips,
            floor_generators: current_state.floor_generators,
        }) {
            continue;
        }

        let mut possibilities: HashSet<State> = HashSet::new();
        if current_state.elevator_position != 3 {
            let next_elevator = current_state.elevator_position + 1;
            let next_possibilities =
                generate_next_possibilities(&current_state, next_elevator, &elements);
            possibilities.extend(next_possibilities.into_iter());
        }
        if current_state.elevator_position != 0
            && current_state.floor_chips[0] != 0
            && current_state.floor_generators[0] != 0
        {
            let next_elevator = current_state.elevator_position - 1;
            let next_possibilities =
                generate_next_possibilities(&current_state, next_elevator, &elements);
            possibilities.extend(next_possibilities.into_iter());
        }

        queue.extend(possibilities);
        // queue.sort_unstable_by_key(|f| f.hyristic);
    }

    print_state(&current_state, &elements);
    println!("Result is {:?}", current_state.turns);
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
