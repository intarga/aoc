use std::{collections::HashSet, convert::identity, fs, iter::repeat, path::Path, vec};

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize, max_dims: &Point) -> Option<Self> {
        if (0..max_dims.x).contains(&x) && (0..max_dims.y).contains(&y) {
            Some(Self { x, y })
        } else {
            None
        }
    }

    fn neighbours(&self, max_dims: &Point) -> Vec<Point> {
        let x = self.x;
        let y = self.y;
        vec![
            Point::new(x + 1, y, max_dims),
            Point::new(x, y + 1, max_dims),
            Point::new(x - 1, y, max_dims),
            Point::new(x, y - 1, max_dims),
        ]
        .into_iter()
        .filter_map(identity)
        .collect()
    }
}

struct Map<T: Copy> {
    vals: Vec<Vec<T>>,
    dims: Point,
}

impl<T: Copy> Map<T> {
    fn new(vals: Vec<Vec<T>>) -> Self {
        let x = vals.len() as isize;
        let y = vals.first().map(|v| v.len()).unwrap_or(0) as isize;

        Self {
            vals,
            dims: Point { x, y },
        }
    }

    fn get(&self, point: &Point) -> T {
        let x = point.x as usize;
        let y = point.y as usize;
        self.vals[x][y]
    }

    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.dims.x)
            .map(|i| repeat(i).zip((0..self.dims.y).into_iter()))
            .flatten()
            .map(|(x, y)| Point { x, y })
    }
}

fn char_to_height(item: char) -> u8 {
    match item {
        'S' => 1,
        'E' => 26,
        _ => (item as u32).checked_sub(96).unwrap().try_into().unwrap(),
    }
}

fn find_start(heightmap: &Map<char>) -> Point {
    heightmap
        .points()
        .find(|point| heightmap.get(&point) == 'S')
        .expect("couldn't find start")
}

fn find_lowest(heightmap: &Map<char>) -> Vec<Point> {
    heightmap
        .points()
        .filter(|point| char_to_height(heightmap.get(&point)) == 1)
        .collect()
}

fn explore_iter(
    heightmap: &Map<char>,
    visited: &mut HashSet<Point>,
    curr_locs: Vec<Point>,
) -> (bool, Vec<Point>) {
    let mut next_locs = Vec::new();
    for loc in curr_locs {
        let curr_height = char_to_height(heightmap.get(&loc));

        for next in loc.neighbours(&heightmap.dims) {
            let char = heightmap.get(&next);
            if char_to_height(char) > curr_height + 1 || visited.contains(&next) {
                continue;
            }
            if char == 'E' {
                return (true, vec![]);
            }
            visited.insert(next.clone());
            next_locs.push(next);
        }
    }

    (false, next_locs)
}

fn explore(heightmap: &Map<char>, start: Vec<Point>) -> usize {
    let mut visited: HashSet<Point> = HashSet::from_iter(start.iter().cloned());

    let mut curr_locs = start;

    let mut found = false;
    let mut steps = 0;

    while !found {
        (found, curr_locs) = explore_iter(heightmap, &mut visited, curr_locs);
        steps += 1;
    }

    steps
}

fn parse(path: impl AsRef<Path>) -> Map<char> {
    Map::new(
        fs::read_to_string(path)
            .expect("failed to read input file")
            .lines()
            .map(|line| line.chars().collect())
            .collect(),
    )
}

fn part_1(input: &Map<char>) -> usize {
    let start = find_start(input);
    explore(input, vec![start])
}

fn part_2(input: &Map<char>) -> usize {
    let lowest_points = find_lowest(input);
    explore(input, lowest_points)
}

fn main() {
    let input = parse("../input/day12");
    println!("part1: {}", part_1(&input));
    println!("part2: {}", part_2(&input));
}
