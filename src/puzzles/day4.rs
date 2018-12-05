use std::collections::HashMap;
use std::io::BufRead;

use chrono::{NaiveDateTime, Timelike};
use regex::Regex;

use super::utils;

struct Sleep {
    start: NaiveDateTime,
    end: Option<NaiveDateTime>,
}

impl Sleep {
    fn new(start: NaiveDateTime) -> Sleep {
        Sleep { start, end: None }
    }

    fn sleep_minutes(&self) -> u32 {
        (self.end.unwrap() - self.start).num_minutes() as u32
    }
}

struct Shift {
    _start: NaiveDateTime,
    sleeps: Vec<Sleep>,
}

impl Shift {
    fn new(start: NaiveDateTime) -> Shift {
        Shift { _start: start, sleeps: Vec::new() }
    }

    fn fall_asleep(&mut self, time: NaiveDateTime) {
        if let Some(sleep) = self.sleeps.last() {
            if sleep.end.is_none() {
                return;
            }
        }

        self.sleeps.push(Sleep::new(time));
    }

    fn wake_up(&mut self, time: NaiveDateTime) {
        if let Some(sleep) = self.sleeps.last_mut() {
            if sleep.end.is_none() {
                sleep.end = Some(time);
            }
        }
    }

    fn total_sleep_minutes(&self) -> u32 {
        let mut total = 0;

        for sleep in &self.sleeps {
            total += sleep.sleep_minutes();
        }

        total
    }
}

struct Guard {
    id: u32,
    shifts: Vec<Shift>,
}

impl Guard {
    fn new(id: u32, start: NaiveDateTime) -> Guard {
        Guard { id, shifts: vec!(Shift::new(start)) }
    }

    fn last_shift(&mut self) -> &mut Shift {
        self.shifts.last_mut().unwrap()
    }

    fn total_sleep_minutes(&self) -> u32 {
        let mut total = 0;

        for shift in &self.shifts {
            total += shift.total_sleep_minutes();
        }

        total
    }

    fn most_asleep_minute(&self) -> (u8, u32) {
        let mut minutes: HashMap<u8, u32> = HashMap::new();

        for shift in &self.shifts {
            for sleep in &shift.sleeps {
                for minute in sleep.start.minute()..sleep.end.unwrap().minute() {
                    *minutes.entry(minute as u8).or_insert(1) += 1;
                }
            }
        }

        let entry = minutes.iter()
            .max_by(|&(_, x), &(_, y)| x.cmp(y));

        if let Some(entry) = entry {
            return (*entry.0, *entry.1);
        } else {
            return (0, 0);
        }
    }
}

struct Record {
    time: NaiveDateTime,
    action: String,
    id: Option<u32>,
}

impl Record {
    fn parse(text: &String) -> Option<Record> {
        let line_parser = Regex::new(r#"\[(?P<time>[\d\-: ]+)] (?P<action>(?:Guard #(?P<id>\d+) begins shift)|(?:falls asleep)|(?:wakes up))"#).unwrap();
        let captures = line_parser.captures(text.as_str())?;

        let time = NaiveDateTime::parse_from_str(captures.name("time")?.as_str(), "%Y-%m-%d %H:%M").unwrap();
        let action = captures.name("action")?.as_str().to_string();
        let mut id: Option<u32> = None;

        if let Some(m) = captures.name("id") {
            id = Some(m.as_str().parse().unwrap());
        }

        Some(Record { time, action, id })
    }

    fn get_records(lines: Vec<String>) -> Vec<Record> {
        let mut records: Vec<Record> = lines.iter().map(|line| Record::parse(line).unwrap()).collect();
        records.sort_by(|x, y| x.time.cmp(&y.time));

        records
    }
}

fn build_guards_map<T: BufRead>(input: T) -> HashMap<u32, Guard> {
    let records = Record::get_records(utils::get_lines(input));

    let mut guards: HashMap<u32, Guard> = HashMap::new();
    let mut current_guard_id: Option<u32> = None;

    for record in records {
        let time = record.time;

        if let Some(id) = record.id {
            if !guards.contains_key(&id) {
                guards.insert(id, Guard::new(id, time));
            }

            current_guard_id = Some(id);
        } else {
            let current_guard = guards.get_mut(&current_guard_id.unwrap()).unwrap();

            match record.action.as_str() {
                "falls asleep" => current_guard.last_shift().fall_asleep(time),
                "wakes up" => current_guard.last_shift().wake_up(time),
                &_ => panic!("What the hell is that action: {}", record.action)
            }
        }
    }

    guards
}


pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let guards = build_guards_map(input);

    let most_slept = guards.iter()
        .max_by(|&(_, x), &(_, y)|
            x.total_sleep_minutes().cmp(&y.total_sleep_minutes())
        )
        .unwrap().1;

    most_slept.id * most_slept.most_asleep_minute().0 as u32
}

pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let guards = build_guards_map(input);

    let most_freq_slept = guards.iter()
        .max_by(|&(_, x), &(_, y)|
            x.most_asleep_minute().1.cmp(&y.most_asleep_minute().1)
        )
        .expect("Couldn't find most freq sleeper").1;

    most_freq_slept.id * most_freq_slept.most_asleep_minute().0 as u32
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}