use std::collections::HashSet;
use std::hash::{
    BuildHasher,
    Hash,
};
use std::io::BufRead;
use std::usize;

use regex::Regex;

use utils;

type Position = (usize, usize);
type TileSet = HashSet<Position>;
type Map = (TileSet, TileSet, TileSet);
type BoundingBox = (Position, Position);


pub fn hashset_take_arbitrary<K, S>(set: &mut HashSet<K, S>) -> Option<K>
    where K: Hash + Eq, S: BuildHasher
{
    let key_ref = {
        if let Some(key_ref) = set.iter().next() {
            unsafe { &*(key_ref as *const _) }
        } else {
            return None;
        }
    };
    set.take(key_ref)
}

fn get_map<T: BufRead>(input: T) -> (Map, BoundingBox) {
    let x_first_parser = Regex::new(r#"(?:x=(?P<x_start>\d+), y=(?P<y_start>\d+)(?:\.{2}(?P<y_end>\d+))?)"#).unwrap();
    let y_first_parser = Regex::new(r#"(?:y=(?P<y_start>\d+), x=(?P<x_start>\d+)(?:\.{2}(?P<x_end>\d+))?)"#).unwrap();

    let mut clay_tiles = HashSet::new();
    let mut top_left = (usize::MAX, usize::MAX);
    let mut bottom_right = (usize::MIN, usize::MIN);

    let lines = utils::get_lines(input);
    for line in lines.iter() {
        let captures = x_first_parser.captures(line)
            .or(y_first_parser.captures(line))
            .expect("line didn't match format");

        let x_start = captures.name("x_start")
            .unwrap().as_str().parse()
            .unwrap();
        let x_end = captures.name("x_end")
            .and_then(|m| Some(m.as_str().parse().unwrap()))
            .unwrap_or(x_start);

        let y_start = captures.name("y_start")
            .unwrap().as_str().parse()
            .unwrap();
        let y_end = captures.name("y_end")
            .and_then(|m| Some(m.as_str().parse().unwrap()))
            .unwrap_or(y_start);

        if x_start < top_left.0 {
            top_left.0 = x_start;
        }
        if x_end > bottom_right.0 {
            bottom_right.0 = x_end;
        }
        if y_start < top_left.1 {
            top_left.1 = y_start;
        }
        if y_end > bottom_right.1 {
            bottom_right.1 = y_end;
        }

        for x in x_start..=x_end {
            for y in y_start..=y_end {
                clay_tiles.insert((x, y));
            }
        }
    }

    let map: Map = (clay_tiles, HashSet::new(), HashSet::new());

    (map, (top_left, bottom_right))
}

#[allow(dead_code)]
fn render_map(map: &Map, bounding_box: &BoundingBox) -> String {
    let mut render = String::new();

    let top_left = bounding_box.0;
    let bottom_right = bounding_box.1;

    let (clay, still_water, flowing_water) = map;

    for y in top_left.1..=bottom_right.1 {
        for x in top_left.0..=bottom_right.0 {
            let c;
            let pos = &(x, y);
            if clay.contains(pos) {
                c = '#';
            } else if still_water.contains(pos) && flowing_water.contains(pos) {
                c = '$';
            } else if still_water.contains(pos) {
                c = '~';
            } else if flowing_water.contains(pos) {
                c = '|';
            } else {
                c = ' ';
            }

            render.push(c);
        }

        render.push('\n');
    }

    render
}

fn fall(map: &mut Map, mut pos: Position, max_y: usize) -> Option<Position> {
    let (clay, _, flowing_water) = map;

    while pos.1 <= max_y {
        let pos_down = (pos.0, pos.1 + 1);
        if !clay.contains(&pos_down) {
            flowing_water.insert(pos);
            pos = pos_down;
        } else {
            return Some(pos);
        }
    }

    None
}

fn spread_dir(map: &Map, temp: &mut HashSet<Position>, pos: Position, direction: isize) -> Option<Position> {
    let (clay, still_water, _) = map;

    let mut pos1 = pos;

    while !clay.contains(&pos1) {
        temp.insert(pos1);
        let pos2 = (pos1.0, pos1.1 + 1);
        if !(clay.contains(&pos2) || still_water.contains(&pos2)) {
            return Some(pos1);
        }

        pos1 = ((pos1.0 as isize + direction) as usize, pos1.1);
    }

    None
}

fn spread(map: &mut Map, pos: Position) -> (Option<Position>, Option<Position>) {
    let mut temp: HashSet<Position> = HashSet::new();

    let pl = spread_dir(map, &mut temp, pos, -1);
    let pr = spread_dir(map, &mut temp, pos, 1);

    let (_, still_water, flowing_water) = map;
    if pl.is_none() && pr.is_none() {
        still_water.extend(temp.iter());
    } else {
        flowing_water.extend(temp.iter());
    }

    (pl, pr)
}

fn flow(map: &mut Map, pos: Position, max_y: usize) {
    let mut to_fall = HashSet::new();
    to_fall.insert(pos);

    let mut to_spread = HashSet::new();

    while !(to_fall.is_empty() && to_spread.is_empty()) {
        while let Some(pos) = hashset_take_arbitrary(&mut to_fall) {
            if let Some(origin) = fall(map, pos, max_y) {
                to_spread.insert(origin);
            }
        }

        while let Some(origin) = hashset_take_arbitrary(&mut to_spread) {
            let (pl, pr) = spread(map, origin);

            if pl.is_none() && pr.is_none() {
                to_spread.insert((origin.0, origin.1 - 1));
            } else {
                if let Some(pos) = pl {
                    to_fall.insert(pos);
                }
                if let Some(pos) = pr {
                    to_fall.insert(pos);
                }
            }
        }
    }
}

pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let (mut map, bounding_box) = get_map(input);
    let min_y = (bounding_box.0).1;
    let max_y = (bounding_box.1).1;

    flow(&mut map, (500, 0), max_y);

    map.1.union(&map.2)
        .filter(|&pos| min_y <= pos.1 && max_y >= pos.1)
        .count() as u32
}

pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let (mut map, bounding_box) = get_map(input);
    let min_y = (bounding_box.0).1;
    let max_y = (bounding_box.1).1;

    flow(&mut map, (500, 0), max_y);

    map.1.iter()
        .filter(|&pos| min_y <= pos.1 && max_y >= pos.1)
        .count() as u32
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}