use std::io::BufRead;

use Instruction::*;

#[derive(Clone, Debug)]
enum Instruction {
    _ADDX,
    ADDX(i32),
    NOOP,
}

#[derive(Clone, Debug)]
struct CPU {
    instructions: Vec<Instruction>,
    program_counter: usize,
    register_x: i32,
}

impl Iterator for CPU {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.instructions.get(self.program_counter) {
            Some(NOOP) | Some(_ADDX) => Some(self.register_x),
            Some(ADDX(num)) => {
                let val = self.register_x;
                self.register_x += num;
                Some(val)
            },
            None => None,
        };

        self.program_counter += 1;

        return res;
    }
}

pub fn day10(input: Box<dyn BufRead>) {
    println!("Day 10");

    let mut cpu = CPU {
        instructions: Vec::new(),
        register_x: 1,
        program_counter: 0,
    };

    for line in input.lines().map(|l| l.unwrap()) {
        let mut split = line.split(" ");
        match (split.next(), split.next()) {
            (Some("noop"), None) => {
                cpu.instructions.push(NOOP);
            }
            (Some("addx"), Some(nbr)) => {
                let num = nbr.parse::<i32>().unwrap();
                cpu.instructions.push(_ADDX);
                cpu.instructions.push(ADDX(num));
            }
            _ => {
                panic!("Unexpected input");
            }
        }
    }

    let mut enumerated = cpu.clone().enumerate();
    enumerated.nth(18).unwrap();
    let part1: i32 = enumerated.step_by(40).map(|(i, x)| (i as i32 + 1) * x).sum();
    println!("Part 1: {}", part1);
}
