use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

use regex::Regex;

use super::utils;

fn get_connections(lines: &Vec<String>) -> Vec<(&str, &str)> {
    let mut connections: Vec<(&str, &str)> = Vec::new();
    let line_parser = Regex::new(r#"Step (?P<first>\w+) must be finished before step (?P<second>\w+) can begin\."#).unwrap();

    for line in lines {
        let captures = line_parser.captures(line).unwrap();
        let first = captures.name("first").unwrap().as_str();
        let second = captures.name("second").unwrap().as_str();

        connections.push((first, second));
    }

    connections
}

fn build_dependency_graph<'a>(connections: Vec<(&'a str, &'a str)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for (from, to) in connections {
        match graph.entry(to) {
            Entry::Occupied(ref mut entry) => entry.get_mut().push(from),
            Entry::Vacant(entry) => { entry.insert(vec!(from)); }
        }

        if let Entry::Vacant(entry) = graph.entry(from) {
            entry.insert(Vec::new());
        }
    }

    graph
}

fn get_choices<'a>(graph: &HashMap<&'a str, Vec<&str>>, owned: &HashSet<&str>) -> Vec<&'a str> {
    let mut choices = Vec::new();

    for (&node, deps) in graph {
        if !owned.contains(node) && deps.iter().all(|dep| owned.contains(dep)) {
            choices.push(node);
        }
    }

    choices
}

pub fn solve_first<T: BufRead>(input: T) -> String {
    let lines = utils::get_lines(input);
    let connections = get_connections(&lines);
    let graph = build_dependency_graph(connections);

    let start = *graph.iter()
        .find(|&(_, n)| n.is_empty())
        .expect("no start, rip").0;

    let mut order = String::from(start);
    let mut owned: HashSet<&str> = HashSet::new();
    owned.insert(start);

    loop {
        let mut choices = get_choices(&graph, &owned);

        choices.sort();
        choices.reverse();

        if let Some(choice) = choices.pop() {
            owned.insert(choice);
            order.push_str(choice);
        } else {
            break;
        }
    }

    order
}

#[derive(Debug)]
struct Worker<'a> {
    id: u8,
    letter: Option<&'a str>,
    ticks: u32,
}

impl<'a> Worker<'a> {
    fn new(id: u8) -> Worker<'a> {
        Worker { id, letter: None, ticks: 0 }
    }

    fn get_time(letter: &str) -> u32 {
        let index = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()
            .position(|l| l.to_string().as_str() == letter)
            .expect("what dis?");

        60 + index as u32 + 1
    }

    fn is_done(&self) -> bool {
        self.letter.is_some() && self.can_work()
    }

    fn can_work(&self) -> bool {
        self.ticks == 0
    }

    fn get_letter(&mut self) -> &'a str {
        let letter = self.letter.unwrap();
        self.letter = None;
        letter
    }

    fn tick(&mut self) {
        if self.ticks > 0 {
            self.ticks -= 1;
        }
    }

    fn work(&mut self, letter: &'a str) {
        self.ticks = Worker::get_time(letter);
        self.letter = Some(letter);
    }
}

pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let lines = utils::get_lines(input);
    let connections = get_connections(&lines);
    let graph = build_dependency_graph(connections);

    let mut workers: Vec<Worker> = vec![Worker::new(1), Worker::new(2), Worker::new(3), Worker::new(3), Worker::new(4)];

    let mut total_ticks = 0;

    let mut order = String::new();
    let mut owned: HashSet<&str> = HashSet::new();

    loop {
        for worker in workers.iter_mut() {
            worker.tick();

            if worker.is_done() {
                let letter = worker.get_letter();
                owned.insert(letter);
                order.push_str(letter);
            }
        }

        let mut choices = get_choices(&graph, &owned);

        for worker in &workers {
            if let Some(letter) = worker.letter {
                if let Some(index) = choices.iter()
                    .position(|&choice| choice == letter) {
                    choices.remove(index);
                }
            }
        }

        choices.sort();
        choices.reverse();

        if choices.is_empty() && workers.iter().all(|w| w.can_work()) {
            break;
        }

        for worker in workers.iter_mut() {
            if worker.can_work() {
                if let Some(letter) = choices.pop() {
                    worker.work(letter);
                }
            }
        }

        total_ticks += 1;
    }

    total_ticks
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input)),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}