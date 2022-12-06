use std::io::BufRead;

pub fn day6(input: Box<dyn BufRead>) {
    println!("hello day6");

    let mut lines = input.lines();
    let line = lines.next().unwrap().unwrap();

    let mut ans: usize = 0;
    'search: for i in 0..line.len() - 3 {
        let slice = &line[i..i + 4];
        let mut iter = slice.chars();

        for _ in 0..3 {
            let item = iter.next().unwrap();
            if iter.clone().find(|it| it.eq(&item)).is_some() {
                continue 'search;
            }
        }

        ans = i + 4;
        break 'search;
    }

    println!("Part 1: {}", ans);
}
