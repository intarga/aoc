use std::fs;

fn item_to_priority(item: char) -> Option<u32> {
    if item.is_uppercase() {
        (item as u32).checked_sub(38)
    } else {
        (item as u32).checked_sub(96)
    }
}

fn rucksack_to_match_priority(rucksack: &str) -> Option<u32> {
    let halfway = rucksack.len() / 2;

    for item in rucksack[0..halfway].chars() {
        if rucksack[halfway..].contains(item) {
            return item_to_priority(item);
        }
    }

    None
}

fn three_rucksacks_to_match_priority(rucksacks: &[&str]) -> Option<u32> {
    assert!(rucksacks.len() == 3);

    for item in rucksacks[0].chars() {
        if rucksacks[1].contains(item) && rucksacks[2].contains(item) {
            return item_to_priority(item);
        }
    }

    None
}

fn main() {
    let contents = fs::read_to_string("input/day3").expect("failed to read input file");

    let priority_sum: u32 = contents
        .lines()
        .map(|rucksack| rucksack_to_match_priority(rucksack).expect("failed to find duplicate"))
        .sum();

    println!("part1: {}", priority_sum);

    let priority_sum: u32 = contents
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|rucksacks| {
            three_rucksacks_to_match_priority(rucksacks).expect("failed to find duplicate")
        })
        .sum();

    println!("part2: {}", priority_sum);
}
