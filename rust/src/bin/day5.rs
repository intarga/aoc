use std::fs;

fn parse_input(contents: &str) -> (Vec<Vec<char>>, impl Iterator<Item = Vec<u32>> + '_) {
    let mut lines = contents.lines();

    let mut crate_diag: Vec<Vec<char>> = Vec::new();

    for _ in 0..8 {
        let row = lines.next().unwrap().chars().collect();
        crate_diag.push(row);
    }

    let mut crate_matrix: Vec<Vec<char>> = Vec::new();

    for i in (1..35).step_by(4) {
        let mut matrix_row: Vec<char> = Vec::new();

        for j in (0..8).rev() {
            let crate_name = crate_diag[j][i];

            if crate_name == ' ' {
                break;
            }

            matrix_row.push(crate_name)
        }

        crate_matrix.push(matrix_row);
    }

    let instructions = lines.skip(2).map(|instruction| {
        instruction
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    });

    (crate_matrix, instructions)
}

fn main() {
    let contents = fs::read_to_string("input/day5").expect("failed to read input file");

    let (mut crate_matrix, instructions) = parse_input(&contents);

    for instruction in instructions {
        for _ in 0..instruction[0] {
            let crate_name = crate_matrix[instruction[1] as usize - 1].pop().unwrap();
            crate_matrix[instruction[2] as usize - 1].push(crate_name)
        }
    }

    let top_crates: String = crate_matrix.iter().map(|row| row.last().unwrap()).collect();

    println!("part1: {}", top_crates);

    let (mut crate_matrix, instructions) = parse_input(&contents);

    for instruction in instructions {
        let mut crane_arm: Vec<char> = Vec::new();

        for _ in 0..instruction[0] {
            let crate_name = crate_matrix[instruction[1] as usize - 1].pop().unwrap();
            crane_arm.push(crate_name)
        }
        for _ in 0..instruction[0] {
            let crate_name = crane_arm.pop().unwrap();
            crate_matrix[instruction[2] as usize - 1].push(crate_name)
        }
    }

    let top_crates: String = crate_matrix.iter().map(|row| row.last().unwrap()).collect();

    println!("part2: {}", top_crates);
}
