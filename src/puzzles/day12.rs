use std::collections::HashMap;
use std::collections::VecDeque;
use std::i32;
use std::i64;
use std::io::BufRead;

use utils;

fn get_input<T: BufRead>(input: T) -> (Vec<char>, HashMap<String, char>) {
    let lines = utils::get_lines(input);

    let split = lines.first().expect("No initial state found")
        .split(": ").take(2).collect::<Vec<&str>>();
    let initial_state = split.last().expect("Initial State invalid");

    let mut rules = HashMap::new();

    for rule in lines[1..].iter() {
        if let [state, result] = rule.split(" => ").collect::<Vec<&str>>()[..] {
            rules.insert(state.to_string(), result.chars().next().unwrap());
        }
    }


    (initial_state.chars().collect(), rules)
}

fn run_step(state: &mut Vec<char>, rules: &HashMap<String, char>, index_offset: &mut i64) {
    let mut pattern: VecDeque<char> = VecDeque::from(vec!['.', '.', '.', '.', '.']);

    let mut index = -2 + *index_offset;
    let mut next_state: Vec<char> = Vec::new();

    for c in state.iter().chain(vec!['.', '.', '.', '.', '.'].iter()) {
        pattern.pop_front();
        pattern.push_back(*c);

        let result = *rules.get(pattern.iter().collect::<String>().as_str())
            .expect(format!("Rules didn't include pattern {:?}", pattern.iter().collect::<String>().as_str()).as_str());

        next_state.push(result);
        if index < *index_offset {
            *index_offset = index;
        }

        index += 1;
    }

    while next_state.starts_with(&['.']) {
        next_state.remove(0);
        *index_offset += 1;
    }

    while next_state.ends_with(&['.']) {
        next_state.pop();
    }

    *state = next_state;
}

fn calculate_sum(state: &Vec<char>, index_offset: i64) -> i64 {
    let mut sum = 0;

    for (i, &c) in state.iter().enumerate() {
        if c == '#' {
            sum += i as i64 + index_offset;
        }
    }

    sum
}


pub fn solve_first<T: BufRead>(input: T) -> i32 {
    let (mut state, rules) = get_input(input);
    let mut index_offset = 0;

    for _ in 0..20 {
        run_step(&mut state, &rules, &mut index_offset);
    }

    calculate_sum(&state, index_offset) as i32
}

pub fn solve_second<T: BufRead>(input: T) -> i64 {
    let (mut state, rules) = get_input(input);
    let mut last_state = state.iter().collect::<String>();
    let mut index_offset = 0;
    let mut last_index_offset = 0;

    let mut it = 0;

    loop {
        run_step(&mut state, &rules, &mut index_offset);
        it += 1;
        let new_state = state.iter().collect::<String>();

        if new_state == last_state {
            break;
        } else {
            last_state = new_state.clone();
            last_index_offset = index_offset;
        }
    }

    index_offset += (50000000000 - it) * (index_offset - last_index_offset);

    calculate_sum(&state, index_offset)
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}