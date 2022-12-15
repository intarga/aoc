use std::collections::HashSet;
use std::fs;

fn track_tail_visits(input: &String, rope_length: usize) -> usize {
    let mut rope: Vec<(i32, i32)> = std::iter::repeat((0, 0)).take(rope_length).collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for (dir, count) in input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, count)| (dir, count.parse::<u32>().unwrap()))
    {
        for _ in 0..count {
            rope[0] = match dir {
                "U" => (rope[0].0, rope[0].1 + 1),
                "D" => (rope[0].0, rope[0].1 - 1),
                "L" => (rope[0].0 - 1, rope[0].1),
                "R" => (rope[0].0 + 1, rope[0].1),
                _ => panic!("unrecognised direction"),
            };

            for i in 1..rope_length {
                let hdiff = rope[i - 1].0 - rope[i].0;
                let vdiff = rope[i - 1].1 - rope[i].1;

                if std::cmp::max(hdiff.abs(), vdiff.abs()) >= 2 {
                    rope[i].0 += hdiff.signum();
                    rope[i].1 += vdiff.signum();
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn main() {
    let contents = fs::read_to_string("input/day9").expect("failed to read input file");

    println!("part1: {}", track_tail_visits(&contents, 2));

    println!("part2: {}", track_tail_visits(&contents, 10));
}
