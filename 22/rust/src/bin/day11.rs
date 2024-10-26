use std::{collections::VecDeque, fs};

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: u64,
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

fn process_round(monkeys: &mut Vec<Monkey>, divide_worry: bool, divisor_prod: u64) {
    for i in 0..monkeys.len() {
        while let Some(item) = monkeys[i].items.pop_front() {
            let mut new_worry = (monkeys[i].operation)(item) % divisor_prod;

            if divide_worry {
                new_worry /= 3;
            }

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

fn calculate_monkey_business(input: &String, rounds: u32, divide_worry: bool) -> u64 {
    let mut monkeys: Vec<Monkey> = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(parse_monkey)
        .collect();

    let divisor_prod = monkeys.iter().map(|monkey| monkey.test).product();

    for _ in 0..rounds {
        process_round(&mut monkeys, divide_worry, divisor_prod);
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<u64>>();
    inspection_counts.sort();
    inspection_counts.reverse();

    inspection_counts.iter().take(2).product()
}

fn main() {
    let contents = fs::read_to_string("input/day11").expect("failed to read input file");

    let monkey_business = calculate_monkey_business(&contents, 20, true);

    println!("part1: {}", monkey_business);

    let monkey_business = calculate_monkey_business(&contents, 10000, false);

    println!("part2: {}", monkey_business);
}
