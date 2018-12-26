use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

use regex::Regex;

use utils;

type SideType = String;
type AttackType = String;
type Weakness = String;
type Immunity = String;

#[derive(Clone, Debug, PartialEq)]
struct Group {
    id: usize,
    side: SideType,
    units: u32,
    hp: u32,
    attack_damage: u32,
    attack_type: AttackType,
    initiative: u32,
    weaknesses: HashSet<Weakness>,
    immunities: HashSet<Immunity>,
}

impl Group {
    fn parse(text: &str, id: usize, side: SideType) -> Option<Group> {
        lazy_static! {
            static ref REGEX:Regex = Regex::new(r#"(?P<units>\d+) units each with (?P<hp>\d+) hit points(?: \((?P<traits>.+)\))? with an attack that does (?P<attack_damage>\d+) (?P<attack_type>\w+) damage at initiative (?P<initiative>\d+)"#)
            .unwrap();
        };
//        let REGEX: Regex = Regex::new(r#""#).unwrap();

        let captures = REGEX.captures(text)?;

        let units: u32 = captures.name("units").unwrap().as_str().parse().unwrap();
        let hp: u32 = captures.name("hp").unwrap().as_str().parse().unwrap();
        let attack_damage: u32 = captures.name("attack_damage").unwrap().as_str().parse().unwrap();
        let initiative: u32 = captures.name("initiative").unwrap().as_str().parse().unwrap();
        let attack_type: AttackType = captures.name("attack_type").unwrap().as_str().to_string();

        let mut weaknesses = HashSet::new();
        let mut immunities = HashSet::new();

        if let Some(traits) = captures.name("traits").and_then(|m| Some(m.as_str())) {
            traits.split("; ")
                .for_each(|part| {
                    if let [trait_type, attack_types] = part.split(" to ").collect::<Vec<&str>>()[..] {
                        let attack_types = attack_types.split(", ")
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>();

                        match trait_type {
                            "weak" => &mut weaknesses,
                            "immune" => &mut immunities,
                            _ => panic!("nope, won't deal with that shit lol")
                        }.extend(attack_types);
                    }
                })
        }


        Some(Group { id, side, units, hp, attack_damage, initiative, attack_type, weaknesses, immunities })
    }

    fn effective_power(&self) -> u32 {
        self.units * self.attack_damage
    }

    fn alive(&self) -> bool {
        return self.units > 0;
    }

    fn pick_target<'a, T: Iterator<Item=&'a Group>>(&self, enemies: &mut T) -> Option<&'a Group> {
        let target = enemies
            .max_by(|a, b|
                (self.damage_against(a), a.effective_power(), a.initiative)
                    .cmp(&(self.damage_against(b), b.effective_power(), b.initiative))
            )?;

        if self.damage_against(target) > 0 {
            Some(target)
        } else {
            None
        }
    }

    fn damage_against(&self, other: &Group) -> u32 {
        if other.immunities.contains(&self.attack_type) {
            0
        } else if other.weaknesses.contains(&self.attack_type) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }

    fn take_damage(&mut self, amount: u32) {
        let deaths = amount / self.hp;
        if deaths >= self.units {
            self.units = 0;
        } else {
            self.units -= deaths;
        }
    }

    fn take_damage_from(&mut self, other: &Group) {
        let dmg = other.damage_against(&self);
        self.take_damage(dmg);
    }
}

fn get_groups<T: BufRead>(input: T) -> HashMap<usize, Group> {
    let lines = utils::get_lines(input);

    let mut units = HashMap::new();

    let mut target = "immune";
    let mut id = 0;

    for line in lines {
        if line == "Immune System:" {
            target = "immune";
        } else if line == "Infection:" {
            target = "infection";
        } else if line == "" {
            continue;
        } else {
            units.insert(id, Group::parse(line.as_str(), id, target.to_string())
                .expect("couldn't get group data"));

            id += 1;
        }
    }


    units
}

fn battle(units_map: &mut HashMap<usize, Group>) -> bool {
    loop {
        let mut unit_ids: Vec<usize> = units_map.keys().map(|id| id.clone()).collect();

        unit_ids.sort_unstable_by(|a, b|
            (units_map[a].effective_power(), units_map[a].initiative)
                .cmp(&(units_map[b].effective_power(), units_map[b].initiative)));

        let mut chosen = HashSet::new();
        let mut target_map = HashMap::new();

        for unit_id in unit_ids.iter().rev() {
            let unit = units_map.get(unit_id).unwrap();

            let target = unit.pick_target(&mut unit_ids.iter()
                .filter_map(|id| {
                    let u = units_map.get(id).unwrap();
                    if u.alive() && u.side != unit.side && !chosen.contains(id) {
                        Some(u)
                    } else {
                        None
                    }
                }));

            if let Some(target) = target {
                target_map.insert(unit.id, target.id);
                chosen.insert(target.id);
            }
        }

        unit_ids.sort_unstable_by(|a, b|
            units_map[a].initiative.cmp(&units_map[b].initiative));

        let mut any_killed = false;

        for unit_id in unit_ids.iter().rev() {
            let unit = units_map.get(unit_id).unwrap().clone();
            if !unit.alive() {
                continue;
            }

            if let Some(target_id) = target_map.get(unit_id) {
                let target = units_map.get_mut(target_id).unwrap();
                let pre_units = target.units;

                target.take_damage_from(&unit);

                if pre_units != target.units {
                    any_killed = true;
                }
            }
        }

        let mut alive_count = HashMap::new();
        let mut to_remove = HashSet::new();

        for unit_id in unit_ids.iter() {
            let unit = units_map.get(unit_id).unwrap();

            if unit.alive() {
                let side = unit.side.clone();
                let count = alive_count.get(&side).unwrap_or(&0) + 1;
                alive_count.insert(side, count);
            } else {
                to_remove.insert(unit_id.clone());
            }
        }

        for id in to_remove {
            units_map.remove(&id);
        }

        if !any_killed {
            return false;
        }

        if alive_count.len() < 2 {
            break;
        }
    }

    true
}

pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let mut units_map = get_groups(input);
    battle(&mut units_map);

    units_map.values().map(|g| g.units).sum::<u32>()
}

pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let original_units_map = get_groups(input);
    let mut i = 0;

    loop {
        let mut units_map = original_units_map.clone();

        for (_, unit) in units_map.iter_mut() {
            if unit.side == "immune" {
                unit.attack_damage += i;
            }
        }

        if battle(&mut units_map) {
            let remaining = units_map.iter().next().unwrap().1;
            if remaining.side == "immune" {
                return units_map.values().map(|u| u.units).sum();
            }
        }

        i += 1;
    }
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}