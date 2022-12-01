mod days;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
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

    let filename: Option<String> = args.opt_value_from_str("-f")?;
    let input: Box<dyn BufRead> = match filename {
        Some(filename) => {
            let f = File::open(filename)?;
            Box::new(BufReader::new(f))
        }
        None => {
            Box::new(BufReader::new(io::stdin()))
        }
    };

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
