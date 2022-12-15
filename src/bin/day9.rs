use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day9").expect("failed to read input file");

    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    let mut visited: HashSet<(i32, i32)> = HashSet::from([tail]);

    for (dir, count) in contents
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, count)| (dir, count.parse::<u32>().unwrap()))
    {
        for _ in 0..count {
            let new_head = match dir {
                "U" => (head.0, head.1 + 1),
                "D" => (head.0, head.1 - 1),
                "L" => (head.0 - 1, head.1),
                "R" => (head.0 + 1, head.1),
                _ => panic!("unrecognised direction"),
            };

            if std::cmp::max((new_head.0 - tail.0).abs(), (new_head.1 - tail.1).abs()) >= 2 {
                tail = head;
                visited.insert(tail);
            }
            head = new_head;
        }
    }

    let num_visited = visited.len();

    println!("part1: {}", num_visited);
    // println!("part2: {}",);
}
