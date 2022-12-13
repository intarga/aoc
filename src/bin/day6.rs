use std::fs;

fn is_marker(pair: &(&[char], isize)) -> bool {
    for i in 0..(pair.0.len() - 1) {
        for j in (i + 1)..(pair.0.len()) {
            if pair.0[i] == pair.0[j] {
                return false;
            }
        }
    }

    true
}

fn find_marker(input: &Vec<char>, length: usize) -> usize {
    input
        .windows(length)
        .zip(0..)
        .find(is_marker)
        .expect("no start of packet found")
        .1 as usize
        + length
}

fn main() {
    let contents = fs::read_to_string("input/day6")
        .expect("failed to read input file")
        .chars()
        .collect::<Vec<char>>();

    let start_of_packet = find_marker(&contents, 4);

    println!("part1: {}", start_of_packet);

    let start_of_message = find_marker(&contents, 14);

    println!("part2: {}", start_of_message);
}
