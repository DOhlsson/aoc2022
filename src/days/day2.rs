use std::io::BufRead;
use std::str::FromStr;
use Hand::*;
use Outcome::*;

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(input: &str) -> Result<Hand, Self::Err> {
        match input {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(()),
        }
    }
}

impl Hand {
    fn play(self: &Hand, opp: &Hand) -> i32 {
        match (self, opp) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 6, // Win
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3, // Draw
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 0, // Loss
        }
    }

    fn value(self: &Hand) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(()),
        }
    }
}

impl Outcome {
    fn solve(self: &Outcome, opp: &Hand) -> Hand {
        match (self, opp) {
            (Win, Scissors) | (Draw, Rock) | (Loss, Paper) => Rock,
            (Win, Rock) | (Draw, Paper) | (Loss, Scissors) => Paper,
            (Win, Paper) | (Draw, Scissors) | (Loss, Rock) => Scissors,
        }
    }

    fn value(self: &Outcome) -> i32 {
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
}

pub fn day2(input: Box<dyn BufRead>) {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines().map(|l| l.unwrap()) {
        let opp = Hand::from_str(&line[0..1]).unwrap();

        let part1_me = Hand::from_str(&line[2..3]).unwrap();
        let part1_points = part1_me.play(&opp) + part1_me.value();
        part1 += part1_points;

        let part2_outcome = Outcome::from_str(&line[2..3]).unwrap();
        let part2_me = part2_outcome.solve(&opp);
        let part2_points = part2_me.play(&opp) + part2_me.value();
        part2 += part2_points;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
