use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufRead;
use std::iter::FromIterator;

use utils;

type Point = (i32, i32, i32, i32);

fn get_points<T: BufRead>(input: T) -> Vec<Point> {
    let lines = utils::get_lines(input);
    let mut points = Vec::new();

    for line in lines {
        if let [a, b, c, d] = line.split(",")
            .map(|v| v.trim().parse().expect("wat? no parse"))
            .collect::<Vec<i32>>()[..] {
            points.push((a, b, c, d));
        }
    }

    points
}

fn get_components(a: &Point) -> [i32; 4] {
    return [a.0, a.1, a.2, a.3];
}

fn from_components(a: &[i32]) -> Point {
    return (a[0], a[1], a[2], a[3]);
}

fn get_diff(a: &Point, b: &Point) -> Point {
    from_components(&get_components(a).iter()
        .zip(get_components(b).iter())
        .map(|(a, b)| b - a)
        .collect::<Vec<i32>>()[..])
}

fn get_length(a: &Point) -> u32 {
    get_components(a).iter().fold(0, |sum, c| sum + c.abs() as u32)
}

fn get_dist(a: &Point, b: &Point) -> u32 {
    let diff = get_diff(a, b);
    get_length(&diff)
}

fn unordered_pair_key<T: PartialOrd>(key: (T, T)) -> (T, T) {
    if key.0 >= key.1 {
        (key.1, key.0)
    } else {
        key
    }
}

pub fn solve_first<T: BufRead>(input: T) -> usize {
    let points = get_points(input);

    let mut connections = HashMap::new();
    let len = points.len();
    for a in points.iter() {
        let adj = connections.entry(a).or_insert(Vec::new());

        for b in points.iter() {
            if get_dist(a, b) <= 3 {
                adj.push(b);
            }
        }
    }

    let mut constellations: Vec<HashSet<Point>> = Vec::new();
    let mut visited = HashSet::new();

    for point in connections.keys() {
        let mut constellation = HashSet::new();
        let mut to_visit = VecDeque::from(vec![point]);

        while let Some(point) = to_visit.pop_front() {
            if visited.contains(point) {
                continue;
            }

            visited.insert(point);
            constellation.insert(**point);

            if let Some(connected) = connections.get(point) {
                to_visit.extend(connected);
            }
        }

        if !constellation.is_empty() {
            constellations.push(constellation);
        }
    }

    constellations.len()
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        _ => Result::Err("This problem only has 1 part!".to_string())
    }
}