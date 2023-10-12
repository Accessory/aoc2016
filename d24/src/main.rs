use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

use itertools::Itertools;
use utils::get_input_path;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

trait GridExtensions {
    fn get_by_point(&self, point: &Point) -> char;
}

impl GridExtensions for Vec<Vec<char>> {
    fn get_by_point(&self, point: &Point) -> char {
        self[point.y as usize][point.x as usize]
    }
}

impl Point {
    fn distance(&self, other: &Point) -> u64 {
        self.x.abs_diff(other.x).add(self.y.abs_diff(other.y))
    }

    fn generate_options(&self, max_x: i64, max_y: i64) -> Vec<Point> {
        let mut rtn: Vec<Point> = Vec::with_capacity(4);
        if self.x > 0 {
            rtn.push(Point {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y != 0 {
            rtn.push(Point {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x < max_x - 1 {
            rtn.push(Point {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y < max_y - 1 {
            rtn.push(Point {
                x: self.x,
                y: self.y + 1,
            });
        }

        rtn
    }
}

fn find_path(
    start: Point,
    end: Point,
    grid: &Vec<Vec<char>>,
    max_x: i64,
    max_y: i64,
) -> Vec<Point> {
    // Preamble
    let mut cache: HashSet<Point> = HashSet::new();
    let mut queue: Vec<FindState> = Vec::new();

    let mut current_state = FindState {
        pos: start,
        path: vec![start],
        _heuristic: start.distance(&end),
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
            .filter(|f| grid.get_by_point(f) != '#' && cache.insert(*f))
            .map(|i| {
                let mut new_path = current_state.path.clone();
                new_path.push(i);
                FindState {
                    pos: i,
                    path: new_path,
                    _heuristic: i.distance(&end),
                }
            })
            .collect();

        queue.append(&mut options);
        // queue.sort_unstable_by_key(|f| f.heuristic);
    }

    current_state.path
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Goal {
    point: Point,
    number: u32,
}

fn run(input_file: &str) {
    // Preamble
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut goals = Vec::with_capacity(5);
    let mut max_x = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let mut row: Vec<char> = Vec::with_capacity(max_x);

        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                row.push('#');
                continue;
            }

            if c.is_ascii_digit() {
                goals.push(Goal {
                    point: Point {
                        x: j as i64,
                        y: i as i64,
                    },
                    number: c.to_digit(10).unwrap(),
                })
            }
            row.push('.');
        }
        max_x = max_x.max(row.len());
        grid.push(row);
    }

    // Prepare
    let max_y = grid.len();
    goals.sort_by_key(|f| f.number);
    let mut result = usize::MAX;

    let start = goals.remove(0);

    let permutations: usize = (1..=goals.len()).product();

    for (i, perm) in goals.iter().permutations(goals.len()).unique().enumerate() {
        print!("\r");
        print!("Start {} of {permutations} or {}%", i + 1, (i+1)*100/permutations);

        let mut inner_result = 0;

        let mut perm_goals = vec![start];
        perm_goals.extend(perm);

        for inner_goals in perm_goals.windows(2) {
            let path = find_path(
                inner_goals[0].point,
                inner_goals[1].point,
                &grid,
                max_x as i64,
                max_y as i64,
            );
            // dbg!(&path);
            inner_result += path.len() - 1;
        }

        result = result.min(inner_result);
    }
    println!();
    // Solve
    // Result
    println!("Result of part 1 is {result}");
}

fn run2(input_file: &str) {
        // Preamble
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut goals = Vec::with_capacity(5);
        let mut max_x = 0;
    
        // Parse
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
    
        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap().trim().to_string();
            let mut row: Vec<char> = Vec::with_capacity(max_x);
    
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    row.push('#');
                    continue;
                }
    
                if c.is_ascii_digit() {
                    goals.push(Goal {
                        point: Point {
                            x: j as i64,
                            y: i as i64,
                        },
                        number: c.to_digit(10).unwrap(),
                    })
                }
                row.push('.');
            }
            max_x = max_x.max(row.len());
            grid.push(row);
        }
    
        // Prepare
        let max_y = grid.len();
        goals.sort_by_key(|f| f.number);
        let mut result = usize::MAX;
    
        let start = goals.remove(0);
    
        let permutations: usize = (1..=goals.len()).product();
    
        for (i, perm) in goals.iter().permutations(goals.len()).unique().enumerate() {
            print!("\r");
            print!("Start {} of {permutations} or {}%", i + 1, (i+1)*100/permutations);
    
            let mut inner_result = 0;
    
            let mut perm_goals = vec![start];
            perm_goals.extend(perm);
            perm_goals.push(start);
    
            for inner_goals in perm_goals.windows(2) {
                let path = find_path(
                    inner_goals[0].point,
                    inner_goals[1].point,
                    &grid,
                    max_x as i64,
                    max_y as i64,
                );
                // dbg!(&path);
                inner_result += path.len() - 1;
            }
    
            result = result.min(inner_result);
        }
        println!();
        // Solve
        // Result
        println!("Result of part 2 is {result}");
}

#[derive(Debug, Clone)]
struct FindState {
    pub pos: Point,
    pub path: Vec<Point>,
    pub _heuristic: u64,
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
