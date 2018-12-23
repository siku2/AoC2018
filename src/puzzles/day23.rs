use std::collections::HashSet;
use std::io::BufRead;
use std::iter::FromIterator;
use std::thread;
use std::time::Duration;

use rayon::prelude::*;
use regex::Regex;

use utils;

type Position = (i32, i32, i32);

trait VectorLike {
    fn get_average<T: Iterator<Item=Position>>(positions: T) -> Option<Position>;
    fn length(&self) -> u32;
    fn direction(&self) -> Position;
}

impl VectorLike for Position {
    fn get_average<T: Iterator<Item=Position>>(positions: T) -> Option<Position> {
        let mut sum_position = (0, 0, 0);
        let mut len = 0;

        for pos in positions {
            sum_position.0 += pos.0 as i64;
            sum_position.1 += pos.1 as i64;
            sum_position.2 += pos.2 as i64;
            len += 1;
        }
        if len > 0 {
            Some(((sum_position.0 / len) as i32, (sum_position.1 / len) as i32, (sum_position.2 / len) as i32))
        } else {
            None
        }
    }

    //noinspection RsUnresolvedReference
    fn length(&self) -> u32 {
        (self.0.abs() + self.1.abs() + self.2.abs()) as u32
    }

    //noinspection RsUnresolvedReference
    fn direction(&self) -> Position {
        let lengths = [self.0.abs(), self.1.abs(), self.2.abs()];

        let (index, _) = lengths.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(&b))
            .unwrap();

        match index {
            0 => (self.0.signum(), 0, 0),
            1 => (0, self.1.signum(), 0),
            2 => (0, 0, self.2.signum()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Nanobot {
    position: Position,
    radius: u32,
}

impl Nanobot {
    fn parse(text: &str) -> Option<Nanobot> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"pos=<(?P<x>-?\d+),(?P<y>-?\d+),(?P<z>-?\d+)>, r=(?P<r>-?\d+)").unwrap();
        };

        let captures = REGEX.captures(text)?;
        let x = captures.name("x").unwrap().as_str().parse().ok()?;
        let y = captures.name("y").unwrap().as_str().parse().ok()?;
        let z = captures.name("z").unwrap().as_str().parse().ok()?;
        let radius = captures.name("r").unwrap().as_str().parse().ok()?;

        Some(Nanobot { position: (x, y, z), radius })
    }
    //noinspection RsUnresolvedReference
    fn distance_to_point(&self, point: &Position) -> u32 {
        let (sx, sy, sz) = self.position;
        let (ox, oy, oz) = point;

        (sx - ox, sy - oy, sz - oz).length()
    }

    fn distance_to(&self, other: &Nanobot) -> u32 {
        self.distance_to_point(&other.position)
    }

    fn point_in_range(&self, point: &Position) -> bool {
        self.distance_to_point(point) <= self.radius
    }

    fn other_in_range(&self, other: &Nanobot) -> bool {
        self.distance_to(other) <= self.radius
    }

    fn range_intersects(&self, other: &Nanobot) -> bool {
        self.distance_to(other) <= self.radius + other.radius
    }
}

fn get_nanobots<T: BufRead>(input: T) -> Vec<Nanobot> {
    let lines = utils::get_lines(input);
    lines.iter()
        .map(|line|
            Nanobot::parse(line.as_str())
                .expect("couldn't parse line"))
        .collect()
}

pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let bots = get_nanobots(input);

    let master = bots.iter()
        .max_by(|a, b| a.radius.cmp(&b.radius))
        .expect("where da master at?");

    bots.iter()
        .filter(|bot| master.other_in_range(bot))
        .count() as u32
}

pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let bots = get_nanobots(input);

    let mut intersection_groups: Vec<HashSet<Nanobot>> = Vec::new();
    for bot in bots {
        {
            let mut target_group = intersection_groups.par_iter_mut()
                .find_first(|group|
                    group.par_iter().all(|other| bot.range_intersects(other)));

            if let Some(group) = target_group {
                group.insert(bot);
                continue;
            }
        }

        intersection_groups.push(HashSet::from_iter(vec![bot]));
    }

    let most: &HashSet<Nanobot> = intersection_groups.par_iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .expect("no intersections");

    let mut point = (0, 0, 0);
    let mut step_factors = (0, 0, 0);
    let mut step_signs = (0, 0, 0);

    static DEBUG: bool = false;

    loop {
        let mut out_of_range = HashSet::new();

        for bot in most.iter() {
            if !bot.point_in_range(&point) {
                out_of_range.insert(bot.position);
            }
        }

        if out_of_range.is_empty() {
            break;
        }

        let avg = Position::get_average(out_of_range.iter()
            .map(|pos| pos.clone())).unwrap();

        let direction = (avg.0 - point.0, avg.1 - point.1, avg.2 - point.2);
        let dir_norm = direction.direction();

        let mut step_factor = 0;

        if dir_norm.0 != 0 {
            if dir_norm.0 == step_signs.0 {
                step_factors.0 += 1;
            } else {
                step_factors.0 = dir_norm.0;
                step_signs.0 = dir_norm.0;
            }

            step_factor = step_factors.0;
        }

        if dir_norm.1 != 0 {
            if dir_norm.1 == step_signs.1 {
                step_factors.1 += 1;
            } else {
                step_factors.1 = dir_norm.1;
                step_signs.1 = dir_norm.1;
            }

            step_factor = step_factors.1;
        }

        if dir_norm.2 != 0 {
            if dir_norm.2 == step_signs.2 {
                step_factors.2 += 1;
            } else {
                step_factors.2 = dir_norm.2;
                step_signs.2 = dir_norm.2;
            }

            step_factor = step_factors.2;
        }

        let step = 1.5f32.powf(step_factor as f32) as i32;

        point.0 += dir_norm.0 * step;
        point.1 += dir_norm.1 * step;
        point.2 += dir_norm.2 * step;

        if DEBUG {
            println!("missing {}", out_of_range.len());
            println!("target {:?}", avg);
            println!("factors {:?}", step_factors);
            println!("direction {:?} {} -> speed {} x{}", dir_norm, direction.length(), step, step_factor);
            println!("new pos: {:?}\n", point);
            thread::sleep(Duration::from_millis(25));
        }
    }

    point.length()
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}