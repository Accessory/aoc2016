use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

use regex::Regex;
use utils::get_input_path;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
    use_p: usize,
}

impl Node {
    #[cfg(test)]
    fn is_too_big(&self) -> bool {
        self.size > 25
    }

    #[cfg(not(test))]
    fn is_too_big(&self) -> bool {
        self.size > 150
    }
}
fn x_y_to_grid(x: usize, y: usize, max_x: usize) -> usize {
    y * max_x + x
}

impl TryFrom<String> for Node {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r#"/dev/grid/node-x(\d+)-y(\d+)\W+(\d+)T\W+(\d+)T\W+(\d+)T\W+(\d+)%"#)
            .unwrap();
        match re.captures(&value) {
            Some(caputres) => {
                let x: usize = caputres[1].parse().unwrap();
                let y: usize = caputres[2].parse().unwrap();
                let size: usize = caputres[3].parse().unwrap();
                let used: usize = caputres[4].parse().unwrap();
                let avail: usize = caputres[5].parse().unwrap();
                let use_p: usize = caputres[6].parse().unwrap();

                Ok(Self {
                    x,
                    y,
                    size,
                    used,
                    avail,
                    use_p,
                })
            }
            None => Err("Could not parse Input."),
        }
    }
}

trait USizeExtensions {
    fn to_grid(&self, max_x: usize) -> [usize; 2];
    fn distance(&self, end: usize, max_x: usize) -> usize;
    fn generate_options(&self, max_x: usize, max_x: usize) -> Vec<usize>;
}

impl USizeExtensions for usize {
    fn to_grid(&self, max_x: usize) -> [usize; 2] {
        let x = self % max_x;
        let y = self / max_x;
        [x, y]
    }

    fn distance(&self, end: usize, max_x: usize) -> usize {
        let a = self.to_grid(max_x);
        let b = end.to_grid(max_x);
        a[0].abs_diff(b[0]).add(a[1].abs_diff(b[1]))
    }

    fn generate_options(&self, max_x: usize, max_y: usize) -> Vec<usize> {
        let mut rtn: Vec<usize> = Vec::with_capacity(4);
        let [x, y] = self.to_grid(max_x);
        if x > 0 {
            rtn.push(x_y_to_grid(x - 1, y, max_x));
        }
        if y != 0 {
            rtn.push(x_y_to_grid(x, y - 1, max_x));
        }
        if x < max_x - 1 {
            rtn.push(x_y_to_grid(x + 1, y, max_x));
        }
        if y < max_y - 1 {
            rtn.push(x_y_to_grid(x, y + 1, max_x));
        }

        rtn
    }
}

fn find_path(
    start: usize,
    end: usize,
    nodes: &Vec<Node>,
    max_x: usize,
    max_y: usize,
    blocked: usize,
) -> Vec<usize> {
    // Preamble
    let mut cache: HashSet<usize> = HashSet::new();
    let mut queue: Vec<FindState> = Vec::new();

    let mut current_state = FindState {
        pos: start,
        path: vec![start],
        heuristic: start.distance(end, max_x),
    };

    queue.push(current_state.clone());

    while !queue.is_empty() {
        current_state = queue.remove(0);

        if current_state.pos == end {
            break;
        }

        let mut options = current_state
            .pos
            .generate_options(max_x, max_y)
            .into_iter()
            .filter(|f| *f != blocked && !nodes[*f].is_too_big() && cache.insert(*f))
            .map(|i| {
                let mut new_path = current_state.path.clone();
                new_path.push(i);
                FindState {
                    pos: i,
                    path: new_path,
                    heuristic: i.distance(end, max_x),
                }
            })
            .collect();

        queue.append(&mut options);
        queue.sort_unstable_by_key(|f| f.heuristic);
    }

    current_state.path
}

fn run(input_file: &str) {
    // Preamble
    let mut nodes: Vec<Node> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if let Ok(node) = line.try_into() {
            nodes.push(node);
        }
    }

    let mut result = 0;

    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            // if i == j {
            //     continue;
            // }
            let a = nodes.get(i).unwrap();
            let b = nodes.get(j).unwrap();

            if a.used != 0 && a.used < b.avail {
                result += 1;
            }
            if b.used != 0 && b.used < a.avail {
                result += 1;
            }
        }
    }

    // Solve
    // Result
    println!("Result of part 1 is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut nodes: Vec<Node> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut empty_x = 0;
    let mut empty_y = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if let Ok(node) = TryInto::<Node>::try_into(line) {
            max_x = max_x.max(node.x);
            max_y = max_y.max(node.y);

            if node.used == 0 {
                empty_x = node.x;
                empty_y = node.y;
            }

            nodes.push(node);
        }
    }

    // Prepare
    let g_x = max_x;
    let g_y = 0;

    max_x += 1;
    max_y += 1;

    nodes.sort_by(|l, r| x_y_to_grid(l.x, l.y, max_x).cmp(&x_y_to_grid(r.x, r.y, max_x)));

    let mut path = find_path(
        x_y_to_grid(g_x, g_y, max_x),
        x_y_to_grid(0, 0, max_x),
        &nodes,
        max_x,
        max_y,
        x_y_to_grid(max_x + 1, max_y + 1, max_x),
    );

    let empty = x_y_to_grid(empty_x, empty_y, max_x);

    path.insert(0, empty);

    // Debug
    // for (i, node) in nodes.iter().enumerate() {
    //     if i == empty {
    //         print!("E");
    //     } else if i == x_y_to_grid(g_x, g_y, max_x) {
    //         print!("S");
    //     } else if node.is_too_big() {
    //         print!("#");
    //     } else {
    //         print!(".");
    //     }
    //     if (i + 1) % max_x == 0 {
    //         println!();
    //     }
    // }
    // println!();
    // Debug End

    let mut result = 0;

    for goal in path.windows(3) {
        let inner_path = find_path(goal[0], goal[2], &nodes, max_x, max_y, goal[1]);
        // inner_path
        //     .iter()
        //     .for_each(|i| println!("{:?}", i.to_grid(max_x)));
        result += inner_path.len();
    }
    println!("Result of part 2 is: {}", result);
}

#[derive(Debug, Clone)]
struct FindState {
    pub pos: usize,
    pub path: Vec<usize>,
    pub heuristic: usize,
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
