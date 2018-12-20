extern crate petgraph;

use std::cmp::Ordering::Equal;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufRead;

use utils;

use self::petgraph::graphmap::UnGraphMap;

type AxisValue = i32;
type Position = (AxisValue, AxisValue);


fn get_instructions<T: BufRead>(input: T) -> String {
    let mut lines = utils::get_lines(input);
    let line = lines.remove(0);

    let len = line.len();
    line[1..len - 1].to_string()
}

fn build_graph(instructions: &String) -> UnGraphMap<Position, f32> {
    let mut maze: UnGraphMap<Position, f32> = UnGraphMap::new();

    let mut positions: HashSet<Position> = HashSet::new();
    let mut start_positions: HashSet<Position> = HashSet::new();
    let mut end_positions: HashSet<Position> = HashSet::new();

    positions.insert((0, 0));
    start_positions.insert((0, 0));

    let mut stack = VecDeque::new();

    for instruction in instructions.chars() {
        match instruction {
            'N' | 'E' | 'S' | 'W' => {
                let mut new_positions = HashSet::new();

                for &position in positions.iter() {
                    let next_position = match instruction {
                        'N' => (position.0, position.1 + 1),
                        'E' => (position.0 + 1, position.1),
                        'S' => (position.0, position.1 - 1),
                        'W' => (position.0 - 1, position.1),
                        _ => unreachable!()
                    };

                    maze.add_edge(position, next_position, 1.0);
                    new_positions.insert(next_position);
                }

                positions = new_positions;
            }
            '(' => {
                stack.push_back((start_positions.clone(), end_positions.clone()));
                start_positions = positions.clone();
                end_positions = HashSet::new();
            }
            ')' => {
                positions.extend(end_positions.iter());
                let (start, end) = stack.pop_back().unwrap();
                start_positions = start;
                end_positions = end;
            }
            '|' => {
                end_positions.extend(positions.iter());
                positions = start_positions.clone();
            }
            _ => panic!("what's this???")
        }
    }

    maze
}

pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let instructions = get_instructions(input);
    let maze = build_graph(&instructions);

    let (path_costs, _) = petgraph::algo::bellman_ford(&maze, (0, 0)).unwrap();

    *path_costs.iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
        .unwrap() as u32
}

pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let instructions = get_instructions(input);
    let maze = build_graph(&instructions);

    let (path_costs, _) = petgraph::algo::bellman_ford(&maze, (0, 0)).unwrap();

    path_costs.iter()
        .filter(|&&a| a >= 1000.0)
        .count() as u32
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}