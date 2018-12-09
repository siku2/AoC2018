use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io::BufRead;

use regex::Regex;

use utils;

struct CoordinateIter {
    start: (u32, u32),
    end: (u32, u32),
    current: (u32, u32),
}

impl CoordinateIter {
    fn new(x: u32, y: u32, width: u32, height: u32) -> CoordinateIter {
        CoordinateIter { start: (x, y), end: (x + width, y + height), current: (x, y) }
    }
}

impl Iterator for CoordinateIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.current.0 < self.end.0 - 1 {
            self.current.0 += 1;
        } else if self.current.1 < self.end.1 - 1 {
            self.current.0 = self.start.0;
            self.current.1 += 1;
        } else {
            return None;
        }

        Some(self.current)
    }
}

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn parse(text: &str) -> Result<Claim, Box<Error>> {
        let claim_parser = Regex::new(r#"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)"#).unwrap();

        let caps = claim_parser.captures(text).unwrap();
        Result::Ok(Claim {
            id: caps.name("id").unwrap().as_str().parse()?,
            x: caps.name("x").unwrap().as_str().parse()?,
            y: caps.name("y").unwrap().as_str().parse()?,
            width: caps.name("width").unwrap().as_str().parse()?,
            height: caps.name("height").unwrap().as_str().parse()?,
        })
    }

    fn coordinates(&self) -> CoordinateIter {
        CoordinateIter::new(self.x, self.y, self.width, self.height)
    }
}

pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let lines = utils::get_lines(input);
    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();

    let mut overlaps = 0;

    for line in lines {
        let claim = Claim::parse(line.as_str()).unwrap();
        for (i, j) in claim.coordinates() {
            let claims = grid.entry((i, j)).or_insert(0);
            *claims += 1;
            if *claims == 2 {
                overlaps += 1;
            }
        }
    }

    overlaps
}

pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let lines = utils::get_lines(input);
    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();

    let mut valid_claims: HashSet<u32> = HashSet::new();

    for line in lines {
        let claim = Claim::parse(line.as_str()).unwrap();
        valid_claims.insert(claim.id);

        for (i, j) in claim.coordinates() {
            match grid.entry((i, j)) {
                Entry::Occupied(o) => {
                    valid_claims.remove(o.get());
                    valid_claims.remove(&claim.id);
                }
                Entry::Vacant(v) => { v.insert(claim.id); }
            };
        }
    }

    *valid_claims.iter().next().expect("No valid claim!")
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}