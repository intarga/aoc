use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day2").expect("failed to read input file");

    let total_score: u32 = contents
        .lines()
        .map(|strategy| match strategy {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => panic!("unrecognised strategy"),
        })
        .sum();

    println!("part1: {}", total_score);

    let total_score: u32 = contents
        .lines()
        .map(|strategy| match strategy {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => panic!("unrecognised strategy"),
        })
        .sum();

    println!("part2: {}", total_score);
}
