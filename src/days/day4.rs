use std::io::BufRead;

pub fn day4(input: Box<dyn BufRead>) {
    println!("hello day4");

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for line in input.lines().map(|l| l.unwrap()) {
        println!("{}", line);

        let mut iter = line.split(&['-', ',']);
        let elf1_start: u32 = iter.next().unwrap().parse().unwrap();
        let elf1_end: u32 = iter.next().unwrap().parse().unwrap();
        let elf2_start: u32 = iter.next().unwrap().parse().unwrap();
        let elf2_end: u32 = iter.next().unwrap().parse().unwrap();

        let elf1_contained = elf1_start >= elf2_start && elf1_end <= elf2_end;
        let elf2_contained = elf2_start >= elf1_start && elf2_end <= elf1_end;

        if elf1_contained || elf2_contained {
            println!("contained!");
            part1_sum += 1;
        }

        let overlap = inside(elf1_start, elf2_start, elf2_end) ||
                    inside(elf1_end, elf2_start, elf2_end) ||
                    inside(elf2_start, elf1_start, elf1_end) ||
                    inside(elf2_end, elf1_start, elf1_end);

        if overlap {
            println!("overlap!");
            part2_sum += 1;
        }
    }

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}

fn inside(i: u32, start: u32, end: u32) -> bool {
    i >= start && i <= end
}
