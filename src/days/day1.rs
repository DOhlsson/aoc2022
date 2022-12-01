use std::io::BufRead;

pub fn day1(input: Box<dyn BufRead>) {
    let mut numbers = Vec::new();
    let mut sum = 0;
    for line in input.lines().map(|l| l.unwrap()) {
        match line.parse::<i32>() {
            Ok(i) => sum += i,
            Err(_) => {
                numbers.push(sum);
                sum = 0;
            }
        }
    }
    numbers.push(sum);

    numbers.sort_by(|a, b| b.cmp(a));

    let part1 = numbers[0];
    let part2 = numbers[0] + numbers[1] + numbers[2];

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
