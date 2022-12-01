mod days;

use std::error::Error;
use std::io;
use std::str::FromStr;

use pico_args::Arguments;
use strum_macros::EnumString;
use days::day1;

#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
enum AOCDay {
    Day1,
    Day2,
    NoDay,
}

use AOCDay::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Advent of Code 2022");

    let mut args = Arguments::from_env();

    let subcommand = args
        .subcommand()?
        .map(|s| AOCDay::from_str(&s).unwrap_or(AOCDay::NoDay));

    let input = io::stdin().lines();

    match subcommand {
        Some(Day1) => day1(input),
        Some(Day2) => {
            todo!();
        }
        Some(NoDay) => {
            println!("got some nonesense");
        }
        None => {
            println!("got nothing");
        }
    };

    Ok(())
}
