use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day1").expect("failed to read input file");

    let mut in_bag = 0;
    let mut max = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            max = std::cmp::max(max, in_bag);
            in_bag = 0
        } else {
            in_bag += line.parse::<u32>().expect("failed to parse number")
        }
    }

    max = std::cmp::max(max, in_bag);

    println!("part1: {}", max);
}
