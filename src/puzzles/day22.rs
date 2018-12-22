extern crate petgraph;

use std::collections::HashMap;
use std::io::BufRead;

use regex::Regex;

use utils;

use self::petgraph::graphmap::UnGraphMap;

type Coordinate = (usize, usize);
type Map = HashMap<Coordinate, u32>;

const EROSION_MOD: u32 = 20183;

fn parse_input<T: BufRead>(input: T) -> (u32, Coordinate) {
    let regex = Regex::new(r"depth: (?P<depth>\d+)\s*target: (?P<target_x>[\d]+),(?P<target_y>[\d]+)").unwrap();
    let text = utils::get_lines(input).join("\n");

    let captures = regex.captures(text.as_str()).expect("invalid input");

    let depth = captures.name("depth").unwrap()
        .as_str()
        .parse().expect("depth not a number?");

    let target_x = captures.name("target_x").unwrap()
        .as_str()
        .parse().expect("target_x not a number?");

    let target_y = captures.name("target_y").unwrap()
        .as_str()
        .parse().expect("target_y not a number?");

    (depth, (target_x, target_y))
}

pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let (depth, target) = parse_input(input);

    let mut erosion_levels: Map = HashMap::new();
    let mut risk_level = 0;

    for x in 0..=target.0 {
        for y in 0..=target.1 {
            let geologic_index: u32;

            if (x, y) == target || (x, y) == (0, 0) {
                geologic_index = 0;
            } else if x == 0 {
                geologic_index = (y * 48271) as u32;
            } else if y == 0 {
                geologic_index = (x * 16807) as u32;
            } else {
                geologic_index = erosion_levels.get(&(x - 1, y)).unwrap() * erosion_levels.get(&(x, y - 1)).unwrap();
            }

            let erosion_level = (geologic_index + depth) % EROSION_MOD;
            risk_level += erosion_level % 3;
            erosion_levels.insert((x, y), erosion_level);
        }
    }


    risk_level
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Tool {
    None,
    Torch,
    ClimbingGear,
}

pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let (depth, target) = parse_input(input);

    let mut erosion_levels: Map = HashMap::new();
    let mut graph: UnGraphMap<(usize, usize, Tool), u32> = UnGraphMap::new();

    graph.add_edge((0, 0, Tool::Torch), (0, 0, Tool::ClimbingGear), 7);
    graph.add_node((target.0, target.1, Tool::Torch));

    for x in 0..=target.0 + 500 {
        for y in 0..=target.1 + 100 {
            let geologic_index: u32;

            if (x, y) == target || (x, y) == (0, 0) {
                geologic_index = 0;
            } else if x == 0 {
                geologic_index = (y * 48271) as u32;
            } else if y == 0 {
                geologic_index = (x * 16807) as u32;
            } else {
                geologic_index = erosion_levels.get(&(x - 1, y)).unwrap() * erosion_levels.get(&(x, y - 1)).unwrap();
            }

            let erosion_level = (geologic_index + depth) % EROSION_MOD;
            erosion_levels.insert((x, y), erosion_level);

            let region_type = erosion_level % 3;

            let tools = match region_type {
                0 => vec![Tool::Torch, Tool::ClimbingGear],
                1 => vec![Tool::None, Tool::ClimbingGear],
                2 => vec![Tool::None, Tool::Torch],
                _ => unreachable!(),
            };

            let mut neighbours = Vec::new();
            if x > 0 {
                neighbours.push((x - 1, y));
            }
            if y > 0 {
                neighbours.push((x, y - 1));
            }

            for &tool in tools.iter() {
                let first_tool = *tools.first().unwrap();
                if tool != first_tool {
                    graph.add_edge((x, y, tool), (x, y, first_tool), 7);
                }

                for &(n_x, n_y) in neighbours.iter() {
                    if graph.contains_node((n_x, n_y, tool)) {
                        graph.add_edge((n_x, n_y, tool), (x, y, tool), 1);
                    }
                }
            }
        }
    }


    let (dist, _) = petgraph::algo::astar(&graph, (0, 0, Tool::Torch), |goal| goal == (target.0, target.1, Tool::Torch), |e| *e.2, |_| 1)
        .expect("couldn't find path?");

    dist
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}