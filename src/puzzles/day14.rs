use std::io::BufRead;

use utils;

fn get_number_of_recipes<T>(input: T) -> u32 where T: BufRead {
    let lines = utils::get_lines(input);
    lines.first().expect("rip input?")
        .parse::<u32>().expect("Not a Number rip?")
}


pub fn solve_first<T: BufRead>(input: T) -> String {
    let recipes = get_number_of_recipes(input);

    let mut current_indices: [usize; 2] = [0, 1];
    let mut scoreboard: Vec<u8> = vec![3, 7];

    while scoreboard.len() < recipes as usize + 10 {
        let sum: u8 = current_indices.iter().map(|&i| scoreboard[i]).sum();

        if sum >= 10 {
            scoreboard.push(1);
            scoreboard.push(sum - 10);
        } else {
            scoreboard.push(sum);
        }

        for i in current_indices.iter_mut() {
            let moves = scoreboard[*i] as usize + 1;
            *i = (*i + moves) % scoreboard.len();
        }
    }

    let len = scoreboard.len();
    scoreboard[len - 10..].iter().map(|&s| s.to_string()).collect::<String>()
}

pub fn solve_second<T: BufRead>(input: T) -> usize {
    let recipes = get_number_of_recipes(input);
    let recipe_digits: Vec<u8> = recipes.to_string().chars()
        .map(|d| d.to_digit(10).unwrap() as u8)
        .collect();

    let recipe_len = recipe_digits.len();

    let mut current_indices: [usize; 2] = [0, 1];
    let mut scoreboard: Vec<u8> = vec![3, 7];

    loop {
        let sum: u8 = current_indices.iter().map(|&i| scoreboard[i]).sum();

        if sum >= 10 {
            scoreboard.push(1);
            let len = scoreboard.len();
            if len >= recipe_len && &recipe_digits as &[u8] == &scoreboard[len - recipe_len..] {
                break;
            }
            scoreboard.push(sum - 10);
        } else {
            scoreboard.push(sum);
        }

        let len = scoreboard.len();
        if len >= recipe_len && &recipe_digits as &[u8] == &scoreboard[len - recipe_len..] {
            break;
        }

        for i in current_indices.iter_mut() {
            let moves = scoreboard[*i] as usize + 1;
            *i = (*i + moves) % scoreboard.len();
        }
    }

    scoreboard.len() - recipe_len
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input)),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}