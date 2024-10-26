use std::collections::HashMap;
use std::fs;

fn process_dir(path: String, input: &[&str], dir_map: &mut HashMap<String, u32>) -> (u32, usize) {
    let mut size = 0;

    let mut i = 0;
    while i < input.len() {
        let line = input[i];

        match &line[..4] {
            "$ ls" => (),
            "dir " => (),
            "$ cd" => {
                let new_dir_name = line.split(' ').nth(2).unwrap();
                if new_dir_name == ".." {
                    dir_map.insert(path, size);
                    return (size, i + 1);
                } else {
                    let new_path = match path.as_str() {
                        "" => new_dir_name.to_string(),
                        "/" => format!("/{}", new_dir_name),
                        _ => format!("{}/{}", path, new_dir_name),
                    };
                    let (subir_size, offset) = process_dir(new_path, &input[i + 1..], dir_map);
                    size += subir_size;
                    i += offset;
                }
            }
            _ => {
                size += line.split(' ').nth(0).unwrap().parse::<u32>().unwrap();
            }
        }
        i += 1;
    }

    dir_map.insert(path, size);
    (size, i)
}

fn main() {
    let contents = fs::read_to_string("input/day7").expect("failed to read input file");

    let mut dirs: HashMap<String, u32> = HashMap::new();

    process_dir(
        String::from(""),
        &contents.lines().collect::<Vec<&str>>(),
        &mut dirs,
    );

    let size_sum: u32 = dirs.values().filter(|size| **size <= 100000).sum();

    println!("part1: {}", size_sum);

    let space_needed = 30000000 - (70000000 - dirs.get("/").unwrap());

    let deletion_candidate_size = dirs
        .values()
        .filter(|size| **size >= space_needed)
        .min()
        .unwrap();

    println!("part2: {}", deletion_candidate_size);
}
