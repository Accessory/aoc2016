use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

use utils::get_input_path;

#[derive(Debug)]
struct Bot {
    id: usize,
    chips: Vec<usize>,
    high: usize,
    low: usize,
    low_to_output: bool,
    high_to_output: bool,
}
impl Bot {
    fn new_with_id_and_low_and_high(
        id: usize,
        low: usize,
        high: usize,
        low_to_output: bool,
        high_to_output: bool,
    ) -> Bot {
        Self {
            id,
            chips: Vec::new(),
            high,
            low,
            low_to_output,
            high_to_output,
        }
    }
}

struct ValueLine {
    value: usize,
    to: usize,
}

fn give_value(
    value: usize,
    to: usize,
    bots: &mut HashMap<usize, Rc<RefCell<Bot>>>,
) -> Option<usize> {
    let chips_len;
    {
        let bot = bots.get(&to).unwrap().clone();
        bot.borrow_mut().chips.push(value);
        chips_len = bot.borrow().chips.len();
    }
    if chips_len == 2 {
        let to_low;
        let to_high;
        let low;
        let high;
        {
            let bot = bots.get(&to).unwrap().clone();
            to_low = bot.borrow().low;
            to_high = bot.borrow().high;
            bot.borrow_mut().chips.sort_unstable();
            low = bot.borrow().chips[0];
            high = bot.borrow().chips[1];
            if high == 61 && low == 17 {
                return Some(to);
            }
            bot.borrow_mut().chips.clear();
        }

        if let Some(rtn) = give_value(low, to_low, bots) {
            return Some(rtn);
        }
        return give_value(high, to_high, bots);
    }
    None
}

fn give_value_no_check(
    value: usize,
    to: usize,
    bots: &mut HashMap<usize, Rc<RefCell<Bot>>>,
    output: &mut HashMap<usize, usize>,
) {
    let chips_len;
    {
        let bot = bots.get(&to).unwrap().clone();
        bot.borrow_mut().chips.push(value);
        chips_len = bot.borrow().chips.len();
    }
    if chips_len == 2 {
        let to_low;
        let to_high;
        let low;
        let high;
        let low_to_output;
        let high_to_output;
        {
            let bot = bots.get(&to).unwrap().clone();
            to_low = bot.borrow().low;
            to_high = bot.borrow().high;
            bot.borrow_mut().chips.sort_unstable();
            low = bot.borrow().chips[0];
            high = bot.borrow().chips[1];
            bot.borrow_mut().chips.clear();
            low_to_output = bot.borrow().low_to_output;
            high_to_output = bot.borrow().high_to_output;
        }
        if low_to_output {
            output.insert(to_low, low);
        } else {
            give_value_no_check(low, to_low, bots, output);
        }

        if high_to_output {
            output.insert(to_high, high);
        } else {
            give_value_no_check(high, to_high, bots, output);
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut bots: HashMap<usize, Rc<RefCell<Bot>>> = HashMap::new();
    let mut value_lines: Vec<ValueLine> = Vec::new();
    let mut result: usize = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split(' ').collect();

        match split[0] {
            "value" => {
                let to: usize = split[5].parse().unwrap();
                let value: usize = split[1].parse().unwrap();

                value_lines.push(ValueLine { value, to });
            }
            "bot" => {
                let bot: usize = split[1].parse().unwrap();
                let low: usize = split[6].parse().unwrap();
                let high: usize = split[11].parse().unwrap();
                let low_to_output: bool = split[5] == "output";
                let high_to_output: bool = split[10] == "output";

                bots.insert(
                    bot,
                    Rc::new(RefCell::new(Bot::new_with_id_and_low_and_high(
                        bot,
                        low,
                        high,
                        low_to_output,
                        high_to_output,
                    ))),
                );
            }
            _ => panic!("Should not be here"),
        }
    }
    // Solve
    for value_line in value_lines {
        if let Some(rtn) = give_value(value_line.value, value_line.to, &mut bots) {
            result = rtn;
            break;
        }
    }

    // Result
    let result_bot = bots.get(&result).unwrap().clone();
    let id = result_bot.borrow().id;
    println!("Result is: {} with  the bot:{:?}", id, result_bot);
}

fn run2(input_file: &str) {
    // Preamble
    let mut bots: HashMap<usize, Rc<RefCell<Bot>>> = HashMap::new();
    let mut output: HashMap<usize, usize> = HashMap::new();
    let mut value_lines: Vec<ValueLine> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let split: Vec<&str> = line.split(' ').collect();

        match split[0] {
            "value" => {
                let to: usize = split[5].parse().unwrap();
                let value: usize = split[1].parse().unwrap();

                value_lines.push(ValueLine { value, to });
            }
            "bot" => {
                let bot: usize = split[1].parse().unwrap();
                let low: usize = split[6].parse().unwrap();
                let high: usize = split[11].parse().unwrap();
                let low_to_output: bool = split[5] == "output";
                let high_to_output: bool = split[10] == "output";

                bots.insert(
                    bot,
                    Rc::new(RefCell::new(Bot::new_with_id_and_low_and_high(
                        bot,
                        low,
                        high,
                        low_to_output,
                        high_to_output,
                    ))),
                );
            }
            _ => panic!("Should not be here"),
        }
    }
    // Solve
    for value_line in &value_lines {
        give_value_no_check(value_line.value, value_line.to, &mut bots, &mut output);
    }

    // Result
    let o0 = *output.get(&0).unwrap();
    let o1 = *output.get(&1).unwrap();
    let o2 = *output.get(&2).unwrap();

    let final_result = o0 * o1 * o2;

    println!("Result is {:?}", final_result);
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
