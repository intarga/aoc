// use core::ops::Range;
use std::collections::HashSet;
use std::fs;
use std::iter;

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

fn compute_viewing_distance(
    coords: Vec<(usize, usize)>,
    height: u32,
    tree_grid: &Vec<Vec<u32>>,
) -> u32 {
    let mut viewing_distance = 0;

    for (i, j) in coords {
        viewing_distance += 1;
        if tree_grid[i][j] >= height {
            break;
        }
    }

    viewing_distance
}

fn compute_scenic_score(i: usize, j: usize, t_g: &Vec<Vec<u32>>) -> u32 {
    let h = t_g[i][j];
    let g_h = t_g.len();
    let g_w = t_g[0].len();

    compute_viewing_distance((0..i).rev().zip(iter::repeat(j)).collect(), h, t_g)
        * compute_viewing_distance(iter::repeat(i).zip((j..g_w).skip(1)).collect(), h, t_g)
        * compute_viewing_distance((i..g_h).skip(1).zip(iter::repeat(j)).collect(), h, t_g)
        * compute_viewing_distance(iter::repeat(i).zip((0..j).rev()).collect(), h, t_g)
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

    let mut scenic_map: Vec<Vec<u32>> = Vec::new();

    for i in 0..grid_height {
        let mut row: Vec<u32> = Vec::new();
        for j in 0..grid_width {
            row.push(compute_scenic_score(i, j, &tree_grid))
        }
        scenic_map.push(row);
    }

    let max_scenic_score = scenic_map.iter().flatten().max().unwrap();

    println!("part2: {}", max_scenic_score);
}
