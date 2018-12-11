use std::collections::HashSet;
use std::error::Error;
use std::i32;
use std::io::BufRead;
use std::ops;

use regex::Regex;

use utils;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn neighbours(&self) -> [Point; 8] {
        [
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1)
        ]
    }
}

impl<'a> ops::AddAssign<&'a Point> for Point {
    fn add_assign(&mut self, rhs: &'a Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct Fragment {
    position: Point,
    velocity: Point,
}

impl Fragment {
    fn new(position: Point, velocity: Point) -> Fragment {
        Fragment { position, velocity }
    }
    fn parse(text: &str) -> Result<Fragment, Box<Error>> {
        let regex = Regex::new(r#"position=< *(?P<x>-?\d+), *(?P<y>-?\d+)> velocity=< *(?P<x_vel>-?\d+), *(?P<y_vel>-?\d+)>"#)
            .unwrap();
        let captures = regex.captures(text).ok_or("Didn't match line")?;

        let x = captures.name("x").unwrap().as_str().parse().unwrap();
        let y = captures.name("y").unwrap().as_str().parse().unwrap();

        let x_vel = captures.name("x_vel").unwrap().as_str().parse().unwrap();
        let y_vel = captures.name("y_vel").unwrap().as_str().parse().unwrap();

        Ok(Fragment::new(Point::new(x, y), Point::new(x_vel, y_vel)))
    }
    fn tick(&mut self) {
        self.position += &self.velocity;
    }
}

fn render_map(map: &HashSet<(i32, i32)>) -> String {
    let mut min_x: i32 = i32::MAX;
    let mut min_y: i32 = i32::MAX;
    let mut max_x: i32 = i32::MIN;
    let mut max_y: i32 = i32::MIN;

    for &(x, y) in map {
        if x > max_x {
            max_x = x;
        } else if x < min_x {
            min_x = x;
        }

        if y > max_y {
            max_y = y;
        } else if y < min_y {
            min_y = y;
        }
    }

    let mut canvas = String::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains(&(x, y)) {
                canvas.push('#');
            } else {
                canvas.push('.');
            }
        }
        canvas.push('\n');
    }

    canvas
}

fn simulate(mut fragments: Vec<Fragment>) -> (HashSet<(i32, i32)>, u32) {
    let mut time_passed = 0;
    let mut map: HashSet<(i32, i32)> = HashSet::new();

    'outer: loop {
        time_passed += 1;
        for fragment in fragments.iter_mut() {
            fragment.tick();
            map.insert(fragment.position.as_tuple());
        }

        let mut complete = true;
        for fragment in &fragments {
            let pos = &fragment.position;
            if !pos.neighbours().iter().any(|p| map.contains(&p.as_tuple())) {
                complete = false;
                break;
            }
        }

        if complete {
            break 'outer;
        }

        map.clear();
    }

    (map, time_passed)
}


pub fn solve_first<T: BufRead>(input: T) -> String {
    let lines = utils::get_lines(input);
    let fragments: Vec<Fragment> = lines.iter()
        .map(|line| Fragment::parse(line.as_str()).unwrap())
        .collect();

    let map = simulate(fragments).0;
    render_map(&map)
}

pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let lines = utils::get_lines(input);
    let fragments: Vec<Fragment> = lines.iter()
        .map(|line| Fragment::parse(line.as_str()).unwrap())
        .collect();

    simulate(fragments).1
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input)),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}