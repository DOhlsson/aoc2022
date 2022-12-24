use std::{fmt::Display, io::BufRead};

use Argument::*;
use Operator::*;

type Num = u64;

#[derive(Clone, Debug)]
enum Argument {
    Old,
    Val(Num),
}

#[derive(Clone, Debug)]
enum Operator {
    Mul,
    Add,
}

#[derive(Clone, Debug)]
struct Monkey {
    num: usize,
    items: Vec<Num>,
    operator: Operator,
    argument: Argument,
    test_div: Num,
    if_true_target: usize,
    if_false_target: usize,
    inspections: Num,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}: {:?}", self.num, self.items)
    }
}

pub fn day11(input: Box<dyn BufRead>) {
    println!("Day 11");

    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut lines = input.lines().map(|l| l.unwrap());
    loop {
        let first = lines.next().unwrap();
        println!("{first}");
        let first = first
            .strip_prefix("Monkey ")
            .unwrap()
            .strip_suffix(":")
            .unwrap();
        let monkey_num = first.parse().unwrap();
        println!("monkey_num = {}", monkey_num);

        let second = lines.next().unwrap();
        println!("{second}");
        let second = second.strip_prefix("  Starting items: ").unwrap();
        let items = second
            .split(", ")
            .filter_map(|item| item.parse().ok())
            .collect();
        println!("items = {:?}", items);

        let third = lines.next().unwrap();
        println!("{third}");
        let third = third.strip_prefix("  Operation: new = old ").unwrap();
        let mut third_split = third.split(" ");
        let operator = match third_split.next().unwrap() {
            "*" => Mul,
            "+" => Add,
            _ => panic!("Unexpected input"),
        };
        let argument = match third_split.next().unwrap() {
            "old" => Old,
            nbr => Val(nbr.parse().unwrap()),
        };
        println!("op & arg = {:?} {:?}", operator, argument);

        let fourth = lines.next().unwrap();
        println!("{fourth}");
        let fourth = fourth.strip_prefix("  Test: divisible by ").unwrap();
        let test_div = fourth.parse().unwrap();
        println!("test_div = {}", test_div);

        let fifth = lines.next().unwrap();
        println!("{fifth}");
        let fifth = fifth.strip_prefix("    If true: throw to monkey ").unwrap();
        let if_true_target = fifth.parse().unwrap();
        println!("if_true_target = {}", if_true_target);

        let sixth = lines.next().unwrap();
        println!("{sixth}");
        let sixth = sixth
            .strip_prefix("    If false: throw to monkey ")
            .unwrap();
        let if_false_target = sixth.parse().unwrap();
        println!("if_false_target = {}", if_false_target);

        let monkey = Monkey {
            num: monkey_num,
            items,
            operator,
            argument,
            test_div,
            if_true_target,
            if_false_target,
            inspections: 0,
        };

        monkeys.push(monkey);

        let seventh = lines.next();
        if seventh.is_none() {
            break;
        }
    }

    let part1 = solve(&mut monkeys.clone(), 20, false);
    println!("Part 1: {}", part1);

    let part2 = solve(&mut monkeys, 10_000, true);
    println!("Part 2: {}", part2);
}

fn solve(monkeys: &mut Vec<Monkey>, rounds: i32, part2: bool) -> Num {
    let lcd = if part2 {
        let mut res = 1;
        for i in 0..monkeys.len() {
            res = res * monkeys[i].test_div;
        }
        res
    } else {
        0
    };

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                monkeys[i].inspections += 1;
                let mut worry = monkeys[i].items.remove(0);
                let val: Num = match &monkeys[i].argument {
                    Val(v) => v.clone(),
                    Old => worry.clone(),
                };

                worry = match monkeys[i].operator {
                    Mul => {
                        worry * val
                    },
                    Add => worry + val,
                };

                if part2 {
                    worry = worry % lcd;
                } else {
                    worry = worry / 3;
                }

                let test = worry.clone() % monkeys[i].test_div.clone() == 0u8.into();
                let if_true_target = monkeys[i].if_true_target;
                let if_false_target = monkeys[i].if_false_target;

                match test {
                    true => monkeys[if_true_target].items.push(worry.clone()),
                    false => monkeys[if_false_target].items.push(worry.clone()),
                }
            }
        }
    }

    for i in 0..monkeys.len() {
        println!("{}", monkeys[i]);
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    let part1 = monkeys[0].inspections * monkeys[1].inspections;
    return part1;
}
