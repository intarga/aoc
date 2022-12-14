// use core::ops::Range;
use std::collections::HashSet;
use std::fs;

fn strafing_scan(
    ns_range: Vec<usize>,
    ew_range: Vec<usize>,
    tree_grid: &Vec<Vec<u32>>,
    trees_seen: &mut HashSet<(usize, usize)>,
    transpose: bool,
) {
    for i in ns_range {
        let mut highest_height = -1;
        for j in ew_range.clone() {
            let curr_height = if !transpose {
                tree_grid[i][j] as i32
            } else {
                tree_grid[j][i] as i32
            };

            if curr_height <= highest_height {
                continue;
            }

            if !transpose {
                trees_seen.insert((i, j));
            } else {
                trees_seen.insert((j, i));
            }

            highest_height = curr_height;
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input/day8").expect("failed to read input file");

    let tree_grid: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();
    let grid_height = tree_grid.len();
    let grid_width = tree_grid[0].len();

    let mut trees_seen: HashSet<(usize, usize)> = HashSet::new();
    strafing_scan(
        (0..grid_height).collect(),
        (0..grid_width).collect(),
        &tree_grid,
        &mut trees_seen,
        false,
    );
    strafing_scan(
        (0..grid_height).rev().collect(),
        (0..grid_width).rev().collect(),
        &tree_grid,
        &mut trees_seen,
        false,
    );
    strafing_scan(
        (0..grid_height).collect(),
        (0..grid_width).collect(),
        &tree_grid,
        &mut trees_seen,
        true,
    );
    strafing_scan(
        (0..grid_height).rev().collect(),
        (0..grid_width).rev().collect(),
        &tree_grid,
        &mut trees_seen,
        true,
    );

    println!("part1: {}", trees_seen.len());
    // println!("part2: {}",);
}
