mod days;

use std::fs::File;
use std::io::{stdin, BufReader, BufRead};

use pico_args::Arguments;
use days::*;

fn main() {
    println!("Advent of Code 2022");

    let mut args = Arguments::from_env();

    let subcommand = args.subcommand().unwrap();
    let day = subcommand.expect("Need day command");

    let filename: Option<String> = args.opt_value_from_str("-f").unwrap();
    let input: Box<dyn BufRead> = match filename {
        Some(filename) => {
            let f = File::open(filename).unwrap();
            Box::new(BufReader::new(f))
        }
        None => {
            Box::new(BufReader::new(stdin()))
        }
    };

    match day.as_str() {
        "day1" => day1(input),
        "day2" => day2(input),
        "day3" => day3(input),
        "day4" => day4(input),
        "day5" => day5(input),
        "day6" => day6(input),
        _ => (),
    };
}
