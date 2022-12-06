use std::io::BufRead;

pub fn day3(input: Box<dyn BufRead>) {
    println!("hello");

    let mut part1_sum: u32 = 0;
    let mut part2_sum: u32 = 0;

    let backpacks: Vec<String> = input.lines().map(|l| l.unwrap()).collect();

    for backpack in &backpacks {
        let halfway = backpack.len() / 2;
        let (comp1_raw, comp2_raw) = backpack.split_at(halfway);
        let mut comp1 = Vec::from(comp1_raw);
        let mut comp2 = Vec::from(comp2_raw);
        comp1.sort();
        comp2.sort();

        let dup;
        let mut i = 0;
        let mut j = 0;
        loop {
            if comp1[i] == comp2[j] {
                dup = comp1[i];
                break;
            } else if comp1[i] < comp2[j] {
                i += 1;
            } else {
                j += 1;
            }
        }

        let dup_priority = into_priority(dup);
        part1_sum += dup_priority as u32;

        println!(
            "lol {} {} {} {}",
            comp1_raw, comp2_raw, dup as char, dup_priority
        );
    }

    for i in (0..backpacks.len()).step_by(3) {
        let mut backpack_one = Vec::from(backpacks[i].as_str().as_bytes());
        let backpack_two = Vec::from(backpacks[i + 1].as_str().as_bytes());
        let backpack_three = Vec::from(backpacks[i + 2].as_str().as_bytes());

        backpack_one.retain(|item| backpack_two.contains(item));
        backpack_one.retain(|item| backpack_three.contains(item));

        let badge = backpack_one[0];
        let priority = into_priority(badge);

        part2_sum += priority as u32;
        println!("group{} {} {}", i / 3, badge as char, priority);
    }

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}

fn into_priority(item: u8) -> u8 {
    // in range
    if item >= 97 && item <= 122 {
        // a-z
        return item - 96;
    } else if item >= 65 && item <= 90 {
        return item - 64 + 26;
    } else {
        panic!("lolol");
    }
}
