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
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

pub fn solve<T>(day: u8, part: u8, input: T) -> Result<String, String> where T: BufRead {
    match day {
        01 => day1::solve(part, input),
        02 => day2::solve(part, input),
        03 => day3::solve(part, input),
        04 => day4::solve(part, input),
        05 => day5::solve(part, input),
        06 => day6::solve(part, input),
        07 => day7::solve(part, input),
        08 => day8::solve(part, input),
        09 => day9::solve(part, input),
        10 => day10::solve(part, input),
        11 => day11::solve(part, input),
        12 => day12::solve(part, input),
        13 => day13::solve(part, input),
        14 => day14::solve(part, input),
        15 => day15::solve(part, input),
        16 => day16::solve(part, input),
        17 => day17::solve(part, input),
        18 => day18::solve(part, input),
        19 => day19::solve(part, input),
        20 => day20::solve(part, input),
        21 => day21::solve(part, input),
        _ => Result::Err(format!("Can't handle day {} yet", day))
    }
}