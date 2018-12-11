use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::i32;
use std::io::BufRead;

use utils;

fn get_grid_serial<T: BufRead>(input: T) -> i32 {
    let lines = utils::get_lines(input);
    let line: &String = lines.get(0).expect("No input rip");
    line.parse().expect("Not a number rip")
}

fn fill_grid(grid_serial: i32) -> [[i32; 300]; 300] {
    let mut grid = [[0; 300]; 300];

    for y in 0..300 as usize {
        for x in 0..300 as usize {
            let rack_id = (x as i32 + 1) + 10;
            let power_level = (((y as i32 + 1) * rack_id + grid_serial) * rack_id) / 100 % 10 - 5;

            let mut sum = power_level;

            if x > 0 {
                sum += grid[y][x - 1];
            }
            if y > 0 {
                sum += grid[y - 1][x];
            }

            if x > 0 && y > 0 {
                sum -= grid[y - 1][x - 1];
            }

            grid[y][x] = sum;
        }
    }

    grid
}

pub struct Square {
    x: u32,
    y: u32,
    size: u32,
    power: i32,
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({0}, {1} [{2}x{2}]): {3}", self.x + 1, self.y + 1, self.size, self.power)
    }
}

fn find_best_square(grid: &[[i32; 300]; 300], size: usize) -> Square {
    let mut best = i32::MIN;
    let mut start = (0, 0);

    for y in size..300 as usize {
        for x in size..300 as usize {
            let total = grid[y][x] - grid[y][x - size] - grid[y - size][x] + grid[y - size][x - size];
            if total > best {
                best = total;
                start = ((x - size + 1) as u32, (y - size + 1) as u32);
            }
        }
    }

    Square { x: start.0, y: start.1, size: size as u32, power: best }
}


pub fn solve_first<T: BufRead>(input: T) -> Square {
    let grid_serial = get_grid_serial(input);
    let grid = fill_grid(grid_serial);

    find_best_square(&grid, 3)
}

pub fn solve_second<T: BufRead>(input: T) -> Square {
    let grid_serial = get_grid_serial(input);
    let grid = fill_grid(grid_serial);

    let mut best: Option<Square> = None;
    for size in 1..=300 {
        let sq = find_best_square(&grid, size);
        if let Some(best) = &best {
            if sq.power < best.power {
                continue;
            }
        }

        best = Some(sq);
    }

    best.expect("no size found")
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}