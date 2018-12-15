use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufRead;
use std::iter::FromIterator;

use rayon::prelude::*;

use utils;

#[derive(Clone, Debug, PartialEq)]
enum Race {
    Goblin,
    Elf,
}

#[derive(Clone)]
struct Unit {
    race: Race,
    hp: u32,
    ap: u16,
    alive: bool,
    x: u32,
    y: u32,
}

impl Unit {
    fn new(x: u32, y: u32, race: Race) -> Unit {
        Unit { x, y, race, hp: 200, ap: 3, alive: true }
    }

    fn positions_in_range(&self) -> Vec<Pos> {
        get_neighbours((self.x, self.y)).to_vec()
    }

    fn take_hit(&mut self, assailant: &Unit) {
        let ap = assailant.ap as u32;
        if self.hp > ap {
            self.hp -= ap;
        } else {
            self.hp = 0;
            self.alive = false;
        }
    }

    fn move_to(&mut self, pos: Pos) {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
    }
}

#[derive(PartialEq)]
enum Tile {
    Wall,
    Open,
}

type Map = Vec<Vec<Tile>>;
type Pos = (u32, u32);

fn order_lexicographically(a: Pos, b: Pos) -> Ordering {
    let y_cmp = a.1.cmp(&b.1);

    if Ordering::Equal == y_cmp {
        a.0.cmp(&b.0)
    } else {
        y_cmp
    }
}


fn get_neighbours(pos: Pos) -> [Pos; 4] {
    let (x, y) = pos;
    [
        (x, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x, y + 1)
    ]
}

fn get_map_tile(map: &Map, pos: Pos) -> Option<&Tile> {
    map.get(pos.1 as usize)
        .and_then(|row: &Vec<Tile>| row.get(pos.0 as usize))
}

fn sort_units(units: &mut Vec<Unit>) {
    units.par_sort_unstable_by(|a, b| order_lexicographically((a.x, a.y), (b.x, b.y)));
}

fn get_enemy_in_range<'a>(unit: &Unit, enemies: &'a mut [&'a mut Unit]) -> Option<&'a mut Unit> {
    let positions: Vec<Pos> = unit.positions_in_range();

    let mut potential: Vec<&mut Unit> = Vec::new();

    for enemy in enemies.iter_mut() {
        if positions.contains(&(enemy.x, enemy.y)) {
            potential.push(enemy);
        }
    }


    let enemy = potential.drain(..)
        .min_by(|a, b| a.hp.cmp(&b.hp));

    enemy
}

fn get_in_range_positions(targets: &[&Unit], map: &Map, occupied: &HashSet<Pos>) -> Vec<Pos> {
    let mut positions = Vec::new();

    for target in targets {
        for pos in target.positions_in_range() {
            if let Some(Tile::Open) = get_map_tile(map, pos) {
                if !occupied.contains(&pos) {
                    positions.push(pos);
                }
            }
        }
    }

    positions
}

fn nearest_position_move(current: Pos, positions: &Vec<Pos>, map: &Map, occupied: &HashSet<Pos>) -> Option<Pos> {
    let mut seen: HashSet<Pos> = HashSet::new();
    let mut to_visit: VecDeque<(Pos, u32)> = VecDeque::from(vec![(current, 0)]);
    let mut path: HashMap<Pos, (Pos, u32)> = HashMap::new();
    path.insert(current, (current, 0));

    while !to_visit.is_empty() {
        let (pos, dist) = to_visit.pop_front().unwrap();
        for &nb in get_neighbours(pos).iter() {
            if let Some(ref tile) = get_map_tile(map, nb) {
                if **tile == Tile::Wall || occupied.contains(&nb) {
                    continue;
                }

                match path.entry(nb) {
                    Entry::Occupied(mut entry) => {
                        let prev = entry.get_mut();
                        if prev.1 > dist + 1 {
                            *prev = (pos, dist + 1);
                        }
                    }
                    Entry::Vacant(entry) => { entry.insert((pos, dist + 1)); }
                }

                if seen.contains(&nb) {
                    continue;
                }

                if !to_visit.iter().any(|&visit| nb == visit.0) {
                    to_visit.push_back((nb, dist + 1));
                }
            }
        }

        seen.insert(pos);
    }

    let (_, mut target) = path.iter()
        .filter_map(|(&pos, &(_, dist))| if positions.contains(&pos) { Some((dist, pos)) } else { None })
        .min_by(|&a, &b| {
            let (dist_a, pos_a) = a;
            let (dist_b, pos_b) = b;
            let dist_comp = dist_a.cmp(&dist_b);
            if dist_comp == Ordering::Equal {
                order_lexicographically(pos_a, pos_b)
            } else {
                dist_comp
            }
        })?;

    loop {
        if let Some(&(parent, dist)) = path.get(&target) {
            if dist > 1 {
                target = parent;
                continue;
            }
        }
        break;
    }

    Some(target)
}

fn get_next_move(current: Pos, targets: &[&Unit], map: &Map, occupied: &HashSet<Pos>) -> Option<Pos> {
    let potential = get_in_range_positions(targets, map, occupied);
    nearest_position_move(current, &potential, map, occupied)
}

#[allow(dead_code)]
fn render_game(map: &Map, units: &Vec<Unit>) -> String {
    let units: HashMap<Pos, &Unit> = HashMap::from_iter(units.iter().map(|u| ((u.x, u.y), u)));
    let mut rendered = String::new();

    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let mut tile_rendered = match tile {
                Tile::Wall => '#',
                Tile::Open => '.'
            };

            if let Some(unit) = units.get(&(x as u32, y as u32)) {
                if unit.alive {
                    tile_rendered = match unit.race {
                        Race::Elf => 'E',
                        Race::Goblin => 'G'
                    }
                }
            }

            rendered.push(tile_rendered);
        }
        rendered.push('\n');
    }

    rendered
}

fn simulate(map: &Map, units: &mut Vec<Unit>, abort_on_elf_death: bool) -> Option<u32> {
    let mut rounds_passed: u32 = 0;

    'simulation: loop {
        sort_units(units);

        for i in 0..units.len() {
            let mut done = false;
            let mut unit = units.remove(i);

            if unit.alive {
                let occupied = HashSet::from_iter(units.iter()
                    .filter_map(|u| if u.alive { Some((u.x, u.y)) } else { None }));

                let mut targets: Vec<&mut Unit> = units.iter_mut()
                    .filter(|u| u.alive && u.race != unit.race).collect();

                if !targets.is_empty() {
                    if let Some(target) = get_next_move(
                        (unit.x, unit.y),
                        targets.iter().map(|ref u| &***u)
                            .collect::<Vec<&Unit>>().as_slice(),
                        &map,
                        &occupied) {
                        unit.move_to(target);
                    }

                    if let Some(enemy) = get_enemy_in_range(&unit, &mut targets) {
                        enemy.take_hit(&unit);

                        if abort_on_elf_death && enemy.race == Race::Elf && !enemy.alive {
                            return None;
                        }
                    }
                } else {
                    done = true;
                }
            }

            units.insert(i, unit);

            if done {
                break 'simulation;
            }
        }

        rounds_passed += 1;
    }

    Some(rounds_passed * units.iter().filter_map(|u| if u.alive { Some(u.hp) } else { None }).sum::<u32>())
}

fn parse_input<T: BufRead>(input: T) -> (Map, Vec<Unit>) {
    let lines = utils::get_lines(input);

    let mut map: Map = Vec::new();
    let mut units: Vec<Unit> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Wall,
                _ => Tile::Open,
            };

            row.push(tile);

            let race = match c {
                'G' => Some(Race::Goblin),
                'E' => Some(Race::Elf),
                _ => None
            };

            if let Some(race) = race {
                units.push(Unit::new(x as u32, y as u32, race));
            }
        }

        map.push(row);
    }

    (map, units)
}


pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let (map, mut units) = parse_input(input);
    simulate(&map, &mut units, false).unwrap()
}

pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let (map, units) = parse_input(input);
    let mut outcome: u32 = 0;

    for ap in 4..100 {
        let mut units = units.clone();
        units.iter_mut()
            .filter(|u| u.race == Race::Elf)
            .for_each(|elf| elf.ap = ap);

        if let Some(result) = simulate(&map, &mut units, true) {
            outcome = result;
            break;
        }
    }

    outcome
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}