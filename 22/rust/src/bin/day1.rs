use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day1").expect("failed to read input file");

    let mut counts: Vec<u32> = Vec::new();
    let mut in_bag = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            counts.push(in_bag);
            in_bag = 0;
        } else {
            in_bag += line.parse::<u32>().expect("failed to parse number")
        }
    }
    counts.push(in_bag);

    counts.sort_by(|a, b| b.cmp(a));

    println!("part1: {}", counts.first().unwrap());

    let top_3 = &counts[0..3];

    println!("part2: {}", top_3.iter().sum::<u32>());
}
