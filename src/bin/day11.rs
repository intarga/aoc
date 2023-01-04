use std::{collections::VecDeque, fs};

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: u32,
}

fn parse_monkey(lines: &[&str]) -> Monkey {
    let operand: Option<u64> = match lines[2].get(25..).unwrap() {
        "old" => None,
        num => Some(num.parse().unwrap()),
    };

    Monkey {
        items: lines[1]
            .get(18..)
            .unwrap()
            .split(", ")
            .map(|num| num.parse().unwrap())
            .collect(),
        operation: match lines[2].chars().nth(23).unwrap() {
            '+' => Box::new(move |old| old + operand.unwrap_or(old)),
            '*' => Box::new(move |old| old * operand.unwrap_or(old)),
            _ => panic!("unrecognised operator"),
        },
        test: lines[3].get(21..).unwrap().parse().unwrap(),
        true_monkey: lines[4].get(29..).unwrap().parse().unwrap(),
        false_monkey: lines[5].get(30..).unwrap().parse().unwrap(),
        inspection_count: 0,
    }
}

fn process_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        while let Some(item) = monkeys[i].items.pop_front() {
            let new_worry = (monkeys[i].operation)(item) / 3;

            let target = if new_worry % monkeys[i].test == 0 {
                monkeys[i].true_monkey
            } else {
                monkeys[i].false_monkey
            };

            monkeys[target].items.push_back(new_worry);

            monkeys[i].inspection_count += 1;
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input/day11").expect("failed to read input file");

    let mut monkeys: Vec<Monkey> = contents
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(parse_monkey)
        .collect();

    for _ in 0..20 {
        process_round(&mut monkeys);
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<u32>>();
    inspection_counts.sort();
    inspection_counts.reverse();

    let monkey_business: u32 = inspection_counts.iter().take(2).product();

    println!("part1: {}", monkey_business);
    // println!("part2: {}",);
}
