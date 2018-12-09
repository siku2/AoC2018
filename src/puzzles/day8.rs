use std::io::BufRead;

use utils;

fn count_meta<'a, T: Iterator<Item=&'a String>>(input: &mut T) -> u32 {
    let mut sum = 0;
    let child_nodes = input.next().unwrap().parse().unwrap();
    let meta_nodes = input.next().unwrap().parse().unwrap();

    for _ in 0..child_nodes {
        sum += count_meta(input);
    }

    for _ in 0..meta_nodes {
        let value: u32 = input.next().unwrap().parse().unwrap();
        sum += value;
    }

    sum
}


pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let lines = utils::get_lines(input);

    count_meta(&mut lines.iter())
}

fn count_meta_referenced<'a, T: Iterator<Item=&'a String>>(input: &mut T) -> u32 {
    let mut sum = 0;
    let child_nodes = input.next().unwrap().parse().unwrap();
    let meta_nodes = input.next().unwrap().parse().unwrap();

    if child_nodes > 0 {
        let mut sums: Vec<u32> = Vec::new();

        for _ in 0..child_nodes {
            sums.push(count_meta_referenced(input));
        }

        for _ in 0..meta_nodes {
            let index: usize = input.next().unwrap().parse().unwrap();
            if index == 0 {
                continue;
            }
            sum += sums.get(index - 1).unwrap_or(&0);
        }
    } else {
        for _ in 0..meta_nodes {
            let value: u32 = input.next().unwrap().parse().unwrap();
            sum += value;
        }
    }

    sum
}


pub fn solve_second<T: BufRead>(input: T) -> u32 {
    let lines = utils::get_lines(input);

    count_meta_referenced(&mut lines.iter())
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}