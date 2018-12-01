#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

use clap::App;

mod puzzles;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let day: u8 = value_t_or_exit!(matches, "DAY", u8);
    let part: u8 = value_t_or_exit!(matches, "part", u8);

    let stdin = stdin();
    let reader: Box<BufRead>;

    if let Some(input) = matches.value_of("input") {
        let file = File::open(input);
        reader = Box::new(BufReader::new(file.unwrap()));
    } else {
        reader = Box::new(stdin.lock());
    }

    println!("Solving day {}/{}", day, part);

    let out = puzzles::solve(day, part, reader);

    println!("{}", out.unwrap());
}