use std::fs;

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize, x_max: isize, y_max: isize) -> Option<Self> {
        if (0..x_max).contains(&x) && (0..y_max).contains(&y) {
            Some(Self { x, y })
        } else {
            None
        }
    }

    fn neighbours(&self, x_max: isize, y_max: isize) -> [Option<Point>; 4] {
        let x = self.x;
        let y = self.y;
        [
            Point::new(x + 1, y, x_max, y_max),
            Point::new(x, y + 1, x_max, y_max),
            Point::new(x - 1, y, x_max, y_max),
            Point::new(x, y - 1, x_max, y_max),
        ]
    }
}

fn char_to_height(item: char) -> u8 {
    match item {
        'S' => 1,
        'E' => 26,
        _ => (item as u32).checked_sub(96).unwrap().try_into().unwrap(),
    }
}

fn find_start(heightmap: &Vec<Vec<char>>, map_height: usize, map_width: usize) -> Point {
    for i in 0..map_height {
        for j in 0..map_width {
            if heightmap[i][j] == 'S' {
                return Point::new(
                    i as isize,
                    j as isize,
                    map_height as isize,
                    map_width as isize,
                )
                .unwrap();
            }
        }
    }
    panic!("couldn't find start")
}

fn find_lowest(heightmap: &Vec<Vec<char>>, map_height: usize, map_width: usize) -> Vec<Point> {
    let mut points = Vec::new();

    for i in 0..map_height {
        for j in 0..map_width {
            if char_to_height(heightmap[i][j]) == 1 {
                points.push(
                    Point::new(
                        i as isize,
                        j as isize,
                        map_height as isize,
                        map_width as isize,
                    )
                    .unwrap(),
                );
            }
        }
    }

    points
}

fn get<T: Copy>(map: &Vec<Vec<T>>, point: &Point) -> T {
    let x = point.x as usize;
    let y = point.y as usize;
    map[x][y]
}

fn set<T>(map: &mut Vec<Vec<T>>, point: &Point, val: T) {
    let x = point.x as usize;
    let y = point.y as usize;
    map[x][y] = val;
}

fn explore_iter(
    heightmap: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    curr_locs: &mut Vec<Point>,
    next_locs: &mut Vec<Point>,
) -> bool {
    for loc in curr_locs {
        let curr_height = char_to_height(get(heightmap, loc));

        for next in loc.neighbours(heightmap.len() as isize, heightmap[0].len() as isize) {
            if let Some(next) = next {
                let char = get(heightmap, &next);
                if char_to_height(char) > curr_height + 1 || get(visited, &next) {
                    continue;
                }
                if char == 'E' {
                    return true;
                }
                set(visited, &next, true);
                next_locs.push(next);
            }
        }
    }

    false
}

fn explore(heightmap: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, start: Vec<Point>) -> usize {
    let mut curr_locs = start.clone();
    let mut next_locs = vec![];

    for loc in start {
        set(visited, &loc, true);
    }

    let mut found = false;
    let mut steps = 0;

    while !found {
        found = explore_iter(heightmap, visited, &mut curr_locs, &mut next_locs);
        curr_locs = next_locs.clone();
        next_locs.clear();
        steps += 1;
    }

    steps
}

fn main() {
    let contents = fs::read_to_string("../input/day12").expect("failed to read input file");

    let heightmap: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let map_height = heightmap.len();
    let map_width = heightmap.first().unwrap().len();

    let mut visited_1: Vec<Vec<bool>> = vec![vec![false; map_width]; map_height];

    let start = find_start(&heightmap, map_height, map_width);
    let steps_1 = explore(&heightmap, &mut visited_1, vec![start]);

    println!("part1: {}", steps_1);

    let mut visited_2: Vec<Vec<bool>> = vec![vec![false; map_width]; map_height];

    let lowest_points = find_lowest(&heightmap, map_height, map_width);
    let steps_2 = explore(&heightmap, &mut visited_2, lowest_points);

    println!("part2: {}", steps_2);
}
