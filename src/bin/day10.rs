use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day10").expect("failed to read input file");

    let mut cycle = 1;
    let mut x = 1;
    let mut signal_sum = 0;

    for instruction in contents
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .flatten()
    {
        if (cycle - 20) % 40 == 0 {
            signal_sum += cycle * x;
        }

        match instruction {
            "noop" | "addx" => (),
            _ => x += instruction.parse::<i32>().unwrap(),
        }
        cycle += 1;
    }

    println!("part1: {}", signal_sum);
    // println!("part2: {}",);
}
