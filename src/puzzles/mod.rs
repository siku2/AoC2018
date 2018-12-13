use std::io::BufRead;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;

pub fn solve<T>(day: u8, part: u8, input: T) -> Result<String, String> where T: BufRead {
    match day {
        1 => day1::solve(part, input),
        2 => day2::solve(part, input),
        3 => day3::solve(part, input),
        4 => day4::solve(part, input),
        5 => day5::solve(part, input),
        6 => day6::solve(part, input),
        7 => day7::solve(part, input),
        8 => day8::solve(part, input),
        9 => day9::solve(part, input),
        10 => day10::solve(part, input),
        11 => day11::solve(part, input),
        12 => day12::solve(part, input),
        13 => day13::solve(part, input),
        _ => Result::Err(format!("Can't handle day {} yet", day))
    }
}