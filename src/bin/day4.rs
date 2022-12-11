use std::fs;

fn pairs_to_encapsulation(pairs: &str) -> u32 {
    let split = pairs.split(&[',', '-'][..]);

    let endpoints: Vec<u32> = split
        .map(|point_str| point_str.parse::<u32>().expect("failed to parse int"))
        .collect();

    (endpoints[0] <= endpoints[2] && endpoints[1] >= endpoints[3]
        || endpoints[2] <= endpoints[0] && endpoints[3] >= endpoints[1]) as u32
}

fn main() {
    let contents = fs::read_to_string("input/day4").expect("failed to read input file");

    let num_encapsulations: u32 = contents.lines().map(pairs_to_encapsulation).sum();

    println!("part1: {}", num_encapsulations);
    // println!("part2: {}",);
}
