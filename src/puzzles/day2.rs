use std::collections::HashMap;
use std::io::BufRead;

use utils;

pub fn solve_first<T: BufRead>(input: T) -> u32 {
    let words = utils::get_lines(input);

    let mut two_letters: u32 = 0;
    let mut three_letters: u32 = 0;

    for word in words {
        let mut char_map: HashMap<char, u32> = HashMap::new();
        for c in word.chars() {
            if char_map.contains_key(&c) {
                if let Some(value) = char_map.get_mut(&c) {
                    *value += 1;
                }
            } else {
                char_map.insert(c, 1);
            }
        }

        let mut found_two_letters = false;
        let mut found_three_letters = false;

        for value in char_map.values() {
            if !found_two_letters && *value == 2 {
                two_letters += 1;
                found_two_letters = true;
            }
            if !found_three_letters && *value == 3 {
                three_letters += 1;
                found_three_letters = true;
            }

            if found_two_letters && found_three_letters {
                break;
            }
        }
    }

    two_letters * three_letters
}

fn get_shared_letters(first: &String, second: &String) -> Vec<char> {
    let first_chars = first.chars();
    let mut second_chars = second.chars();

    first_chars.filter(|c| *c == second_chars.next().unwrap()).collect()
}

pub fn solve_second<T: BufRead>(input: T) -> String {
    let words = utils::get_lines(input);

    for i in 0..words.len() - 1 {
        let first: &String = words.get(i).unwrap();

        for j in i + 1..words.len() {
            let second: &String = words.get(j).unwrap();

            let shared_chars = get_shared_letters(first, second);
            if shared_chars.len() == first.len() - 1 {
                return shared_chars.into_iter().collect();
            }
        }
    }

    panic!("Couldn't find correct box...")
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}