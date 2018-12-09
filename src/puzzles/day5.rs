use std::collections::VecDeque;
use std::io::BufRead;
use std::iter::FromIterator;

use utils;

fn swap_case(c: &char) -> char {
    let mut next: Box<Iterator<Item=char>>;
    if c.is_uppercase() {
        next = Box::new(c.to_lowercase());
    } else {
        next = Box::new(c.to_uppercase());
    }

    next.next().unwrap()
}

fn reduce(input: &str) -> String {
    let mut input: VecDeque<char> = VecDeque::from_iter(input.chars());
    let mut remaining: VecDeque<char> = VecDeque::new();

    while let Some(inp) = input.pop_front() {
        if let Some(c) = remaining.pop_back() {
            if inp == swap_case(&c) {
                if let Some(c) = remaining.pop_back() {
                    input.push_front(c);
                }
            } else {
                remaining.push_back(c);
                remaining.push_back(inp);
            }
        } else {
            remaining.push_back(inp);
        }
    }

    String::from_iter(remaining)
}


pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let mut lines = utils::get_lines(input);
    let line = lines.first_mut().unwrap();
    reduce(line.as_str()).len() as u32
}

pub fn solve_second<T: BufRead>(input: T) -> i32 {
    let mut lines = utils::get_lines(input);
    let line = lines.first_mut().unwrap();

    let mut shortest: i32 = -1;

    for letter in "abcdefghijklmnopqrstuvwxyz".chars() {
        let line = line
            .replace(letter, "")
            .replace(letter.to_uppercase().next().unwrap(), "");

        let l = reduce(line.as_str()).len() as i32;

        if l < shortest || l == -1 {
            shortest = l;
        }
    }

    shortest
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}