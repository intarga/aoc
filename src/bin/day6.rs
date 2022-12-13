use std::fs;

fn is_start_of_packet(pair: &(&[char], isize)) -> bool {
    for i in 0..(pair.0.len() - 1) {
        for j in (i + 1)..(pair.0.len()) {
            if pair.0[i] == pair.0[j] {
                return false;
            }
        }
    }

    true
}

fn main() {
    let contents = fs::read_to_string("input/day6")
        .expect("failed to read input file")
        .chars()
        .collect::<Vec<char>>();

    let start_of_packet = contents
        .windows(4)
        .zip(0..)
        .find(is_start_of_packet)
        .expect("no start of packet found")
        .1
        + 4;

    println!("part1: {}", start_of_packet);
    // println!("part2: {}",);
}
