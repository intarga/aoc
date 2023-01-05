use std::fs;

fn char_to_height(item: char) -> u8 {
    match item {
        'S' => 1,
        'E' => 26,
        _ => (item as u32).checked_sub(96).unwrap().try_into().unwrap(),
    }
}

// fn find_start(heightmap: &Vec<Vec<char>>, map_height: usize, map_width: usize) -> (usize, usize) {
//     for i in 0..map_height {
//         for j in 0..map_width {
//             if heightmap[i][j] == 'S' {
//                 return (i, j);
//             }
//         }
//     }
//     panic!("couldn't find start")
// }

fn explore(
    heightmap: &Vec<Vec<char>>,
    stepmap: &mut Vec<Vec<Option<u32>>>,
    coords: (usize, usize),
    prev_height: u32,
    curr_steps: u32,
) {
    // if heightmap[coords.0][coords.1]
}

fn main() {
    let contents = fs::read_to_string("input/day12").expect("failed to read input file");

    // println!("{}", contents);

    let heightmap: Vec<Vec<u8>> = contents
        .lines()
        .map(|line| line.chars().map(char_to_height).collect())
        .collect();
    let map_height = heightmap.len();
    let map_width = heightmap.first().unwrap().len();

    let stepmap: Vec<Vec<Option<u32>>> = vec![vec![None; map_width]; map_height];

    // let start_coords = find_start(&heightmap, map_height, map_width);
    let start_coords: (usize, usize) = (20, 0);

    // println!("start: ({},{})", start_coords.0, start_coords.1)
    println!(
        "a: {}, z: {}, S: {}, E: {}",
        char_to_height('a'),
        char_to_height('z'),
        char_to_height('S'),
        char_to_height('E')
    )

    // println!("part1: {}",);
    // println!("part2: {}",);
}
