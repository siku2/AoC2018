use std::collections::VecDeque;
use std::error::Error;
use std::io::BufRead;

use regex::Regex;

use utils;

fn rotate_deque<T>(deque: &mut VecDeque<T>, steps: isize) {
    if steps >= 0 {
        for _ in 0..steps {
            let el = deque.pop_back().unwrap();
            deque.push_front(el);
        }
    } else {
        for _ in 0..steps.abs() {
            let el = deque.pop_front().unwrap();
            deque.push_back(el);
        }
    }
}

fn simulate_game(players: u32, marbles: u32) -> u64 {
    let mut game: VecDeque<u32> = VecDeque::from(vec![0]);
    let mut player_scores: Vec<u64> = vec![0; players as usize];

    for marble in 1..marbles {
        if marble % 23 == 0 {
            rotate_deque(&mut game, 7);
            let bonus = game.pop_back().unwrap();
            rotate_deque(&mut game, -1);

            let player_score: &mut u64 = player_scores.get_mut((marble % players) as usize).unwrap();
            *player_score += (marble + bonus) as u64;
        } else {
            rotate_deque(&mut game, -1);
            game.push_back(marble);
        }
    }

    *player_scores.iter().max().unwrap()
}

fn get_input_params<T: BufRead>(input: T) -> Result<(u32, u32), Box<Error>> {
    let regex = Regex::new(r#"(?P<players>\d+) players; last marble is worth (?P<marbles>\d+) points"#).unwrap();
    let lines = utils::get_lines(input);
    let text = lines.get(0).unwrap();
    let captures = regex.captures(text.as_str()).ok_or("No match!")?;

    let players: u32 = captures.name("players").unwrap().as_str().parse()?;
    let marbles: u32 = captures.name("marbles").unwrap().as_str().parse()?;

    Ok((players, marbles))
}

pub fn solve_first<T: BufRead>(input: T) -> u64 {
    let (players, marbles) = get_input_params(input).unwrap();
    simulate_game(players, marbles)
}

pub fn solve_second<T: BufRead>(input: T) -> u64 {
    let (players, marbles) = get_input_params(input).unwrap();
    simulate_game(players, marbles * 100)
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}