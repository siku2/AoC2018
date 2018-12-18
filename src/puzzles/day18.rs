use std::collections::HashSet;
use std::io::BufRead;

use utils;

type Position = (usize, usize);
type Grid = Vec<Vec<Tile>>;

#[derive(Clone, Eq, Hash, PartialEq)]
enum Tile {
    Open,
    Tree,
    Lumberyard,
}

#[allow(dead_code)]
fn render_grid(grid: &Grid) -> String {
    let mut render = String::new();

    for row in grid.iter() {
        for tile in row.iter() {
            let c = match tile {
                Tile::Open => ' ',
                Tile::Tree => '|',
                Tile::Lumberyard => '#',
            };

            render.push(c);
        }

        render.push('\n');
    }

    render
}

fn get_grid(lines: Vec<String>) -> Grid {
    let mut grid = Grid::new();

    for line in lines.iter() {
        let mut row = Vec::new();

        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Open,
                '|' => Tile::Tree,
                '#' => Tile::Lumberyard,
                _ => panic!("wat"),
            };

            row.push(tile);
        }

        grid.push(row);
    }

    grid
}

fn count_neighbours(grid: &Grid, pos: Position) -> (u8, u8, u8) {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let mut opens = 0;
    let mut trees = 0;
    let mut lumberyards = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let x = pos.0 as isize + i;
            let y = pos.1 as isize + j;

            if (x >= 0 && x < width) && (y >= 0 && y < height) {
                match grid[y as usize][x as usize] {
                    Tile::Open => opens += 1,
                    Tile::Tree => trees += 1,
                    Tile::Lumberyard => lumberyards += 1,
                };
            }
        }
    }

    (opens, trees, lumberyards)
}

fn simulate(mut grid: Grid, minutes: usize) -> Grid {
    let mut state_set: HashSet<Grid> = HashSet::new();
    let mut states: Vec<Grid> = Vec::new();

    let mut repeat_index = 0;

    for _ in 0..minutes {
        let mut next_grid = Grid::new();

        for (y, row) in grid.iter().enumerate() {
            let mut next_row = Vec::new();

            for (x, tile) in row.iter().enumerate() {
                let (_, tree_ns, lumberyard_ns) = count_neighbours(&grid, (x, y));

                let next_tile = match tile {
                    Tile::Open => if tree_ns >= 3 { Tile::Tree } else { Tile::Open },
                    Tile::Tree => if lumberyard_ns >= 3 { Tile::Lumberyard } else { Tile::Tree },
                    Tile::Lumberyard => if lumberyard_ns >= 1 && tree_ns >= 1 { Tile::Lumberyard } else { Tile::Open },
                };

                next_row.push(next_tile)
            }

            next_grid.push(next_row);
        }

        if state_set.contains(&next_grid) {
            repeat_index = states.iter()
                .position(|g| g == &next_grid)
                .unwrap();
            break;
        }

        states.push(next_grid.clone());
        state_set.insert(next_grid.clone());
        grid = next_grid;
    }

    let total_states = states.len();
    let cycle_len = total_states - repeat_index;
    let minutes_left = minutes - repeat_index;

    // this index calculation seriously ducked me up man...
    let index = repeat_index + ((minutes_left - 1) % cycle_len);
    let grid = states.remove(index);

    grid
}

fn calc_resource_value(grid: &Grid) -> u32 {
    let mut trees = 0;
    let mut lumberyards = 0;

    for tile in grid.iter().flatten() {
        match tile {
            Tile::Tree => trees += 1,
            Tile::Lumberyard => lumberyards += 1,
            _ => ()
        };
    }


    trees * lumberyards
}

pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let grid = get_grid(utils::get_lines(input));
    let grid = simulate(grid, 10);
    calc_resource_value(&grid)
}


pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let grid = get_grid(utils::get_lines(input));
    let grid = simulate(grid, 1000000000);
    calc_resource_value(&grid)
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}