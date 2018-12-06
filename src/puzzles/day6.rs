use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;

use super::utils;

#[derive(Clone, Copy, Debug)]
struct AreaOrigin {
    x: u32,
    y: u32,
    valid: bool,
    size: u32,
}

impl PartialEq for AreaOrigin {
    fn eq(&self, other: &AreaOrigin) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl AreaOrigin {
    fn new(x: u32, y: u32) -> AreaOrigin {
        AreaOrigin { x, y, valid: true, size: 0 }
    }

    fn distance_to(&self, x: u32, y: u32) -> u32 {
        let diff_x = if self.x > x { self.x - x } else { x - self.x };
        let diff_y = if self.y > y { self.y - y } else { y - self.y };

        diff_x + diff_y
    }
}

#[derive(Debug)]
struct AreaPoint {
    nearest: Option<usize>,
    distance: u32,
}

impl AreaPoint {
    fn new(nearest: usize, distance: u32) -> AreaPoint {
        AreaPoint { nearest: Some(nearest), distance }
    }
}

fn get_coordinates(lines: &Vec<String>) -> Vec<AreaOrigin> {
    let mut coordinates: Vec<AreaOrigin> = Vec::new();

    for line in lines {
        if let [x, y] = line.split(", ").take(2).collect::<Vec<&str>>()[..] {
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();

            coordinates.push(AreaOrigin::new(x, y));
        }
    }

    coordinates
}

#[derive(Debug)]
struct BoundingBox {
    tl: (u32, u32),
    br: (u32, u32),
}

fn find_bounding_box(coordinates: &Vec<AreaOrigin>) -> Result<BoundingBox, Box<Error>> {
    let mut tl: Option<(u32, u32)> = None;
    let mut br: Option<(u32, u32)> = None;

    for coordinate in coordinates {
        let x = coordinate.x;
        let y = coordinate.y;

        if let Some(ref mut tl_coord) = tl {
            if x < tl_coord.0 {
                tl_coord.0 = x;
            } else if y < tl_coord.1 {
                tl_coord.1 = y;
            }
        } else {
            tl = Some((x, y));
        }

        if let Some(ref mut br_coord) = br {
            if x > br_coord.0 {
                br_coord.0 = x;
            } else if y > br_coord.1 {
                br_coord.1 = y;
            }
        } else {
            br = Some((x, y));
        }
    }

    let tl: (u32, u32) = tl.ok_or("no coordinate found")?;
    let br: (u32, u32) = br.unwrap();

    Ok(BoundingBox { tl, br })
}

fn draw_grid(grid: &HashMap<(u32, u32), AreaPoint>, bounds: &BoundingBox) -> String {
    let mut draw = String::new();

    for y in bounds.tl.1..=bounds.br.1 {
        for x in bounds.tl.0..=bounds.br.0 {
            if let Some(point) = grid.get(&(x, y)) {
                draw.push_str(&point.nearest
                    .and_then(|i| Some(i.to_string()))
                    .unwrap_or(".".to_string()).as_str()
                );
            }
        }

        draw.push('\n');
    }

    draw
}

fn perform_expansion(coordinates: &mut Vec<AreaOrigin>) -> (HashMap<(u32, u32), AreaPoint>, BoundingBox) {
    let bounds = find_bounding_box(&coordinates).unwrap();
    let mut grid: HashMap<(u32, u32), AreaPoint> = HashMap::new();

    for (i, coord) in coordinates.iter().enumerate() {
        for x in bounds.tl.0..=bounds.br.0 {
            for y in bounds.tl.1..=bounds.br.1 {
                let dist = coord.distance_to(x, y);

                match grid.entry((x, y)) {
                    Entry::Occupied(mut entry) => {
                        let point = entry.get_mut();
                        if point.distance > dist {
                            point.nearest = Some(i);
                            point.distance = dist;
                        } else if point.distance == dist {
                            point.nearest = None;
                        }
                    }
                    Entry::Vacant(entry) => { entry.insert(AreaPoint::new(i, dist)); }
                }
            }
        }
    }

    for x in bounds.tl.0..=bounds.br.0 {
        for y in bounds.tl.1..=bounds.br.1 {
            if let Some(point) = grid.get_mut(&(x, y)) {
                if let Some(area) = point.nearest.and_then(|i| coordinates.get_mut(i)) {
                    if x == bounds.tl.0 || x == bounds.br.0 || y == bounds.tl.1 || y == bounds.br.1 {
                        area.valid = false;
                    } else {
                        area.size += 1;
                    }
                }
            }
        }
    }

    (grid, bounds)
}


pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let mut coordinates = get_coordinates(&utils::get_lines(input));
    perform_expansion(&mut coordinates);

    coordinates.iter()
        .filter(|&area| area.valid)
        .max_by(|&x, &y| x.size.cmp(&y.size))
        .unwrap()
        .size
}

pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let mut coordinates = get_coordinates(&utils::get_lines(input));
    let (_, bounds) = perform_expansion(&mut coordinates);

    let mut size = 0;

    for x in bounds.tl.0..=bounds.br.0 {
        for y in bounds.tl.1..=bounds.br.1 {
            let mut sum = 0;
            for coord in &coordinates {
                sum += coord.distance_to(x, y);
            }

            if sum < 10000 {
                size += 1;
            }
        }
    }

    size
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}