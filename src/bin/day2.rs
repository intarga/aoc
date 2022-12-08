use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day2").expect("failed to read input file");

    // println!("{}", contents);

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

    // let scores = contents
    //     .lines()
    //     .map(|strategy| match strategy {
    //         "A X" => 4,
    //         "A Y" => 8,
    //         "A Z" => 3,
    //         "B X" => 1,
    //         "B Y" => 5,
    //         "B Z" => 9,
    //         "C X" => 7,
    //         "C Y" => 2,
    //         "C Z" => 6,
    //         _ => panic!("unrecognised strategy"),
    //     })
    //     .zip(contents.lines());

    // for line in scores {
    //     println!("strat: {}, score: {}", line.1, line.0)
    // }

    println!("part1: {}", total_score);

    // println!("part2: {}",);
}
