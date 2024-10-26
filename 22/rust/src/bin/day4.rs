use std::fs;

fn pairs_to_endpoints(pairs: &str) -> [u32; 4] {
    let split = pairs.split(&[',', '-'][..]);

    split
        .map(|point_str| point_str.parse::<u32>().expect("failed to parse int"))
        .collect::<Vec<u32>>()
        .try_into()
        .expect("parsed wrong number of ints")
}

fn endpoints_to_encapsulation(endpoints: [u32; 4]) -> u32 {
    (endpoints[0] <= endpoints[2] && endpoints[1] >= endpoints[3]
        || endpoints[2] <= endpoints[0] && endpoints[3] >= endpoints[1]) as u32
}

fn endpoints_to_overlap(endpoints: [u32; 4]) -> u32 {
    (endpoints[0] <= endpoints[2] && endpoints[1] >= endpoints[2]
        || endpoints[0] <= endpoints[3] && endpoints[1] >= endpoints[3]
        || endpoints[2] <= endpoints[0] && endpoints[3] >= endpoints[0]
        || endpoints[2] <= endpoints[1] && endpoints[3] >= endpoints[1]) as u32
}

fn main() {
    let contents = fs::read_to_string("input/day4").expect("failed to read input file");

    let num_encapsulations: u32 = contents
        .lines()
        .map(pairs_to_endpoints)
        .map(endpoints_to_encapsulation)
        .sum();

    println!("part1: {}", num_encapsulations);

    let num_overlaps: u32 = contents
        .lines()
        .map(pairs_to_endpoints)
        .map(endpoints_to_overlap)
        .sum();

    println!("part2: {}", num_overlaps);
}
