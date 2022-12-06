use std::io::BufRead;

pub fn day6(input: Box<dyn BufRead>) {
    println!("hello day6");

    let mut lines = input.lines();
    let line = lines.next().unwrap().unwrap();

    let mut part1: Option<usize> = None;
    let mut part2: Option<usize> = None;

    let mut iter = line.chars();
    'search: for i in 0.. {
        iter.next();

        if part1.is_none() {
            let mut iter_clone = iter.clone().take(4);
            for _ in 0..3 {
                let item = iter_clone.next().unwrap();
                if iter_clone.clone().find(|it| it.eq(&item)).is_some() {
                    continue 'search;
                }
            }
            part1 = Some(i + 4);
        }

        if part2.is_none() {
            let mut iter_clone = iter.clone().take(14);
            for _ in 0..13 {
                let item = iter_clone.next().unwrap();
                if iter_clone.clone().find(|it| it.eq(&item)).is_some() {
                    continue 'search;
                }
            }
            part2 = Some(i + 14);
            break 'search;
        }
    }

    println!("Part 1: {}", part1.unwrap());
    println!("Part 2: {}", part2.unwrap());
}
