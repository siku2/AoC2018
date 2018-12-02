use std::collections::HashSet;
use std::io::BufRead;

use super::utils;

pub fn solve_first<T: BufRead>(input: T) -> i32 {
    let mut frequency = 0;

    for change in utils::get_lines(input) {
        let change: i32 = change.parse().unwrap();
        frequency += change;
    }

    frequency
}

pub fn solve_second<T: BufRead>(input: T) -> i32 {
    let mut frequency_changes = Vec::new();

    for change in utils::get_lines(input) {
        let change: i32 = change.parse().unwrap();
        frequency_changes.push(change);
    }


    let mut frequency = 0;
    let mut frequency_table = HashSet::new();
    frequency_table.insert(frequency);

    for change in frequency_changes.iter().cycle() {
        frequency += change;
        if frequency_table.contains(&frequency) {
            return frequency;
        }

        frequency_table.insert(frequency);
    }

    panic!("Didn't reach any frequency twice!");
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}