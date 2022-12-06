use std::io::BufRead;

pub fn day5(input: Box<dyn BufRead>) {
    println!("hello day5");

    let mut stacks: [Vec<u8>; 9] = [(); 9].map(|_| Vec::new());

    let lines: Vec<String> = input.lines().map(|l| l.unwrap()).collect();

    let middle = lines.iter().position(|item| item.eq("")).unwrap();
    let (crates, procedures) = lines.split_at(middle);

    let mut crates_iter = crates.iter().rev();
    let width = crates_iter.next().unwrap().len();
    let columns = width / 4 + 1;

    for line in crates_iter {
        let bytes = line.as_bytes();
        for i in 0..columns {
            let idx = i*4 + 1;
            let c = bytes[idx];
            if c != b' ' {
                stacks[i].push(c);
            }
        }
    }

    let mut procedures_iter = procedures.iter();
    procedures_iter.next();

    for line in procedures_iter {
        println!("p {}", line);

        let mut split = line.split(" ");
        split.next(); // move
        let count = split.next().unwrap().parse::<usize>().unwrap();
        split.next(); // from
        let source = split.next().unwrap().parse::<usize>().unwrap() - 1;
        split.next(); // to
        let dest = split.next().unwrap().parse::<usize>().unwrap() - 1;

        println!("#{} {}->{}", count, source, dest);

        /* Part 1
        for _ in 0..count {
            let item = stacks[source].pop().unwrap();
            stacks[dest].push(item);
        }
        */

        /* Part 2 */
        let len = stacks[source].len();
        if len != 0 {
            println!("wut {} {}", count, len);
            let mut items: Vec<u8> = stacks[source].drain((len-count)..len).collect();
            stacks[dest].append(&mut items);
            println!("res {:?}", items);
        }
    }

    let top: Vec<u8> = stacks.iter_mut().map(|stack| stack.pop().unwrap_or(b' ')).collect();
    let part1 = String::from_utf8(top).unwrap();
    println!("Part 1: {}", part1);
}
