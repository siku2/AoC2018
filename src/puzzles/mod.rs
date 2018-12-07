use std::io::BufRead;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

mod utils;

pub fn solve<T>(day: u8, part: u8, input: T) -> Result<String, String> where T: BufRead {
    match day {
        1 => day1::solve(part, input),
        2 => day2::solve(part, input),
        3 => day3::solve(part, input),
        4 => day4::solve(part, input),
        5 => day5::solve(part, input),
        6 => day6::solve(part, input),
        7 => day7::solve(part, input),
        _ => Result::Err(format!("Can't handle day {} yet", day))
    }
}